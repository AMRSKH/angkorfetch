#!/usr/bin/env python3
"""Tests for scripts/sync_package_manifests.py.

Run with:

    python -m unittest discover -s scripts -p 'test_*.py' -v

The point of these tests is narrower than "the script works". The Homebrew
formula previously drifted into a shape that `brew audit --strict` rejects -- an
explicit `version` stanza and `Hardware::CPU` conditionals -- and the old
regex-patching generator faithfully reproduced that shape on every release. So
the assertions below are written to fail if the old shape can be produced or
committed again, not merely to check that a sync updates some digits.
"""

from __future__ import annotations

import shutil
import tempfile
import unittest
from pathlib import Path

import sync_package_manifests as sync

HERE = Path(__file__).resolve().parent
GOLDEN = HERE / "testdata" / "angkorfetch.rb.golden"

# Read with universal newlines so the fixture is LF in memory regardless of how
# git checked it out. Without this, every byte comparison below depends on the
# platform: the fixture arrives CRLF on a Windows checkout, and the CRLF cases
# would then convert it a second time into \r\r\n. `.gitattributes` pins the file
# to LF as well, but the tests should not be the thing that enforces that.
GOLDEN_TEXT = GOLDEN.read_text(encoding="utf-8")

VERSION = "9.9.9"
SUMS = {
    "angkorfetch-macos-aarch64.tar.gz": "1" * 64,
    "angkorfetch-macos-x86_64.tar.gz": "2" * 64,
    "angkorfetch-linux-aarch64.tar.gz": "3" * 64,
    "angkorfetch-linux-x86_64.tar.gz": "4" * 64,
    "angkorfetch-windows-x86_64.zip": "5" * 64,
}


def write(path: Path, text: str, newline: str = "\n") -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="") as handle:
        handle.write(text.replace("\n", newline) if newline != "\n" else text)


def read_bytes(path: Path) -> bytes:
    return path.read_bytes()


class RenderShapeTests(unittest.TestCase):
    """The rendered formula must match the shape audited in the tap."""

    def setUp(self) -> None:
        self.rendered = sync.render_homebrew_formula(VERSION, SUMS)

    def test_matches_golden_fixture(self) -> None:
        expected = GOLDEN_TEXT
        # The fixture is maintained by hand and mirrors AMRSKH/homebrew-tap's
        # Formula/angkorfetch.rb, so any change to the template that is not also
        # a deliberate change to the canonical tap shape fails here.
        self.assertEqual(self.rendered, expected)

    def test_has_no_version_stanza(self) -> None:
        # `brew audit --strict`: "`version 1.1.1` is redundant with version
        # scanned from URL".
        for line in self.rendered.splitlines():
            self.assertNotRegex(line, r'^\s*version\s+"')

    def test_uses_no_hardware_cpu_conditionals(self) -> None:
        # FormulaAudit/OnSystemConditionals rejects `if Hardware::CPU.arm?`.
        self.assertNotIn("Hardware::CPU", self.rendered)
        self.assertNotIn("else", self.rendered)

    def test_declares_on_arm_and_on_intel_within_each_os(self) -> None:
        for os_name in sync.BREW_OSES:
            self.assertIn(f"  on_{os_name} do", self.rendered)
        for arch in sync.BREW_ARCHES:
            self.assertEqual(self.rendered.count(f"    on_{arch} do"), len(sync.BREW_OSES))

    def test_pairs_every_architecture_with_its_own_artifact(self) -> None:
        # A url/sha256 pair that names the wrong architecture still passes a
        # checksum check, so assert the mapping explicitly.
        version, parsed = sync.parse_homebrew_formula(self.rendered)
        self.assertEqual(version, VERSION)
        self.assertEqual(parsed, {name: SUMS[name] for name in sync.BREW_ARTIFACTS.values()})

    def test_omits_artifacts_homebrew_does_not_consume(self) -> None:
        self.assertNotIn("windows", self.rendered)
        self.assertNotIn(".deb", self.rendered)
        self.assertNotIn(".rpm", self.rendered)

    def test_missing_artifact_is_an_error(self) -> None:
        partial = {k: v for k, v in SUMS.items() if "linux-aarch64" not in k}
        with self.assertRaises(sync.SyncError) as caught:
            sync.render_homebrew_formula(VERSION, partial)
        self.assertIn("angkorfetch-linux-aarch64.tar.gz", str(caught.exception))

    def test_line_length_within_homebrew_limit(self) -> None:
        # Layout/LineLength Max is 118 in Homebrew's RuboCop config.
        for line in self.rendered.splitlines():
            if line.strip().startswith("url "):
                continue  # exempt via AllowedPatterns
            self.assertLessEqual(len(line), 118, line)


class UpdateHomebrewTests(unittest.TestCase):
    def setUp(self) -> None:
        self.root = Path(self.enterContext(tempfile.TemporaryDirectory()))
        self.formula = self.root / sync.HOMEBREW_FORMULA
        write(self.formula, GOLDEN_TEXT)

    def test_rewrites_to_the_requested_version(self) -> None:
        sums = dict(SUMS)
        sums["angkorfetch-macos-aarch64.tar.gz"] = "a" * 64
        sync.update_homebrew(self.root, "1.2.3", sums)
        version, parsed = sync.parse_homebrew_formula(
            self.formula.read_text(encoding="utf-8", newline="")
        )
        self.assertEqual(version, "1.2.3")
        self.assertEqual(parsed["angkorfetch-macos-aarch64.tar.gz"], "a" * 64)

    def test_running_twice_produces_no_further_change(self) -> None:
        sync.update_homebrew(self.root, "1.2.3", SUMS)
        first = read_bytes(self.formula)
        sync.update_homebrew(self.root, "1.2.3", SUMS)
        self.assertEqual(first, read_bytes(self.formula))

    def test_rewriting_an_already_current_file_is_a_no_op(self) -> None:
        before = read_bytes(self.formula)
        sync.update_homebrew(self.root, VERSION, SUMS)
        self.assertEqual(before, read_bytes(self.formula))

    def test_preserves_lf_line_endings(self) -> None:
        sync.update_homebrew(self.root, "1.2.3", SUMS)
        blob = read_bytes(self.formula)
        self.assertNotIn(b"\r\n", blob)

    def test_preserves_crlf_line_endings(self) -> None:
        write(self.formula, GOLDEN_TEXT, newline="\r\n")
        sync.update_homebrew(self.root, "1.2.3", SUMS)
        blob = read_bytes(self.formula)
        self.assertIn(b"\r\n", blob)
        # Every LF must belong to a CRLF; stripping CRLFs must leave no bare LF.
        self.assertNotIn(b"\n", blob.replace(b"\r\n", b""))

    def test_rewrites_the_legacy_shape_into_the_audited_one(self) -> None:
        # The regression this whole change exists to prevent: given the old
        # unaudited formula, a sync must produce the audited shape rather than
        # preserving what it was handed.
        write(
            self.formula,
            'class Angkorfetch < Formula\n'
            '  desc "Fast, cross-platform system fetch tool"\n'
            '  homepage "https://github.com/AMRSKH/angkorfetch"\n'
            '  license "MIT"\n'
            '  version "0.0.1"\n'
            "\n"
            "  on_macos do\n"
            "    if Hardware::CPU.arm?\n"
            '      url "https://github.com/AMRSKH/angkorfetch/releases/download/v0.0.1/'
            'angkorfetch-macos-aarch64.tar.gz"\n'
            '      sha256 "' + "0" * 64 + '"\n'
            "    else\n"
            '      url "https://github.com/AMRSKH/angkorfetch/releases/download/v0.0.1/'
            'angkorfetch-macos-x86_64.tar.gz"\n'
            '      sha256 "' + "0" * 64 + '"\n'
            "    end\n"
            "  end\n"
            "end\n",
        )
        sync.update_homebrew(self.root, VERSION, SUMS)
        self.assertEqual(
            self.formula.read_text(encoding="utf-8", newline=""),
            GOLDEN_TEXT,
        )


class CheckHomebrewTests(unittest.TestCase):
    def setUp(self) -> None:
        self.root = Path(self.enterContext(tempfile.TemporaryDirectory()))
        self.formula = self.root / sync.HOMEBREW_FORMULA
        write(self.formula, GOLDEN_TEXT)

    def test_accepts_the_generated_shape(self) -> None:
        sync.check_homebrew(self.root)  # must not raise

    def test_accepts_crlf(self) -> None:
        write(self.formula, GOLDEN_TEXT, newline="\r\n")
        sync.check_homebrew(self.root)

    def test_rejects_a_reintroduced_version_stanza(self) -> None:
        text = GOLDEN_TEXT
        write(self.formula, text.replace('  license "MIT"', '  version "9.9.9"\n  license "MIT"'))
        with self.assertRaises(sync.SyncError):
            sync.check_homebrew(self.root)

    def test_rejects_hardware_cpu_conditionals(self) -> None:
        text = GOLDEN_TEXT
        write(self.formula, text.replace("    on_arm do", "    if Hardware::CPU.arm?"))
        with self.assertRaises(sync.SyncError):
            sync.check_homebrew(self.root)

    def test_rejects_a_hand_edited_stanza(self) -> None:
        text = GOLDEN_TEXT
        write(self.formula, text.replace('license "MIT"', 'license "Apache-2.0"'))
        with self.assertRaises(sync.SyncError):
            sync.check_homebrew(self.root)

    def test_rejects_mixed_versions(self) -> None:
        text = GOLDEN_TEXT
        write(self.formula, text.replace("v9.9.9/angkorfetch-linux-x86_64", "v8.8.8/angkorfetch-linux-x86_64"))
        with self.assertRaises(sync.SyncError) as caught:
            sync.check_homebrew(self.root)
        self.assertIn("mixes versions", str(caught.exception))

    def test_rejects_a_url_without_a_following_sha256(self) -> None:
        text = GOLDEN_TEXT
        write(self.formula, text.replace('      sha256 "' + "1" * 64 + '"\n', ""))
        with self.assertRaises(sync.SyncError) as caught:
            sync.check_homebrew(self.root)
        self.assertIn("no sha256", str(caught.exception))

    def test_error_names_the_regeneration_command(self) -> None:
        text = GOLDEN_TEXT
        write(self.formula, text.replace('license "MIT"', 'license "Apache-2.0"'))
        with self.assertRaises(sync.SyncError) as caught:
            sync.check_homebrew(self.root)
        self.assertIn("sync_package_manifests.py", str(caught.exception))


class RepositoryFormulaTests(unittest.TestCase):
    """The formula actually committed to this repository must be generator output."""

    def test_committed_formula_matches_the_generator(self) -> None:
        sync.check_homebrew(HERE.parent)

    def test_committed_formula_lives_where_the_audit_cops_run(self) -> None:
        # Homebrew's RuboCop config only applies the FormulaAudit cops to paths
        # matching **/{Formula,Casks}/**/*.rb. HomebrewFormula/ and the repo root
        # both miss that pattern, so the formula would go unaudited there.
        self.assertEqual(sync.HOMEBREW_FORMULA.parts[0], "Formula")
        self.assertTrue((HERE.parent / sync.HOMEBREW_FORMULA).is_file())


class ChecksumParsingTests(unittest.TestCase):
    def setUp(self) -> None:
        self.root = Path(self.enterContext(tempfile.TemporaryDirectory()))

    def test_parses_sha256sum_output(self) -> None:
        path = self.root / "checksums.txt"
        path.write_text(f"{'1' * 64}  a.tar.gz\n{'2' * 64} *b.zip\n", encoding="utf-8")
        self.assertEqual(sync.parse_checksums(path), {"a.tar.gz": "1" * 64, "b.zip": "2" * 64})

    def test_rejects_a_non_digest(self) -> None:
        path = self.root / "checksums.txt"
        path.write_text("notahash  a.tar.gz\n", encoding="utf-8")
        with self.assertRaises(sync.SyncError):
            sync.parse_checksums(path)

    def test_rejects_conflicting_digests_for_one_file(self) -> None:
        path = self.root / "checksums.txt"
        path.write_text(f"{'1' * 64}  a.tar.gz\n{'2' * 64}  a.tar.gz\n", encoding="utf-8")
        with self.assertRaises(sync.SyncError):
            sync.parse_checksums(path)

    def test_rejects_an_empty_file(self) -> None:
        path = self.root / "checksums.txt"
        path.write_text("\n\n", encoding="utf-8")
        with self.assertRaises(sync.SyncError):
            sync.parse_checksums(path)


class WingetTests(unittest.TestCase):
    """The winget manifests are patched, not rendered, so cover that path too."""

    INSTALLER = (
        "PackageIdentifier: AMRSKH.AngkorFetch\n"
        'PackageVersion: "0.0.1"\n'
        "InstallerType: zip\n"
        "Installers:\n"
        "  - Architecture: x64\n"
        "    InstallerUrl: https://github.com/AMRSKH/angkorfetch/releases/download/v0.0.1/"
        "angkorfetch-windows-x86_64.zip\n"
        "    InstallerSha256: " + "0" * 64 + "\n"
    )
    VERSION_MANIFEST = (
        "PackageIdentifier: AMRSKH.AngkorFetch\nPackageVersion: \"0.0.1\"\nManifestType: version\n"
    )

    def setUp(self) -> None:
        self.root = Path(self.enterContext(tempfile.TemporaryDirectory()))
        self.old = self.root / sync.WINGET_BASE / "0.0.1"
        write(self.old / "AngkorFetch.installer.yaml", self.INSTALLER)
        write(self.old / "AngkorFetch.yaml", self.VERSION_MANIFEST)

    def test_renames_the_version_directory_and_updates_fields(self) -> None:
        sync.update_winget(self.root, VERSION, SUMS)
        new = self.root / sync.WINGET_BASE / VERSION
        self.assertTrue(new.is_dir())
        self.assertFalse(self.old.exists())
        text = (new / "AngkorFetch.installer.yaml").read_text(encoding="utf-8")
        self.assertIn(f'PackageVersion: "{VERSION}"', text)
        self.assertIn(f"v{VERSION}/angkorfetch-windows-x86_64.zip", text)
        self.assertIn("5" * 64, text)

    def test_running_twice_produces_no_further_change(self) -> None:
        sync.update_winget(self.root, VERSION, SUMS)
        new = self.root / sync.WINGET_BASE / VERSION
        before = {p.name: read_bytes(p) for p in new.glob("*.yaml")}
        sync.update_winget(self.root, VERSION, SUMS)
        after = {p.name: read_bytes(p) for p in new.glob("*.yaml")}
        self.assertEqual(before, after)

    def test_missing_installer_checksum_is_an_error(self) -> None:
        with self.assertRaises(sync.SyncError):
            sync.update_winget(self.root, VERSION, {"unrelated.tar.gz": "0" * 64})

    def test_ambiguous_version_directories_are_an_error(self) -> None:
        shutil.copytree(self.old, self.root / sync.WINGET_BASE / "0.0.2")
        with self.assertRaises(sync.SyncError) as caught:
            sync.update_winget(self.root, VERSION, SUMS)
        self.assertIn("exactly one version directory", str(caught.exception))


if __name__ == "__main__":
    unittest.main(verbosity=2)
