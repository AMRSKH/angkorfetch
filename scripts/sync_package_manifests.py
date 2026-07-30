#!/usr/bin/env python3
"""Sync the Homebrew formula and winget manifests to a published release.

Reads sha256 values from a release's ``checksums.txt`` and rewrites the package
definitions in place.

Two design choices matter for the automation built on top of this script:

1. Edits are driven by the *artifact filename* embedded in each existing URL,
   not by matching the old version string. That keeps the script correct no
   matter how far behind the package definitions have drifted.
2. Writing identical values produces no diff. The caller can therefore treat
   "git reports no changes" as "already current" and skip opening a pull
   request, which is what makes the surrounding workflow idempotent.

Line endings are preserved byte-for-byte so a sync never introduces unrelated
whitespace churn.

The Homebrew formula is *rendered from a template* rather than patched in place.
The winget manifests are patched, because they carry hand-maintained metadata
(description, tags, publisher URLs) that no template should own.

That asymmetry is deliberate. A patching approach can only ever preserve
whatever shape it is handed, so it happily reproduced a formula that
``brew audit --strict`` rejects:

  - an explicit ``version`` stanza, which audit reports as "redundant with
    version scanned from URL";
  - ``if Hardware::CPU.arm?`` conditionals, which ``FormulaAudit/
    OnSystemConditionals`` rejects in favour of ``on_arm``/``on_intel``.

Rendering makes the audited shape the *only* shape this script can emit, so a
future release cannot regenerate the old one. ``--check`` enforces the same
property against the committed file: it re-renders from the version and digests
the file itself declares and fails on any byte of drift, which also catches hand
edits.
"""

from __future__ import annotations

import argparse
import difflib
import re
import sys
from pathlib import Path

REPO = "AMRSKH/angkorfetch"
WINGET_BASE = Path("winget-pkgs/manifests/a/AMRSKH/AngkorFetch")

# Homebrew resolves a tap's formulae from `Formula/`, `HomebrewFormula/` or the
# tap root. `Formula/` is the only one of the three that matches the
# `**/{Formula,Casks}/**/*.rb` include patterns in Homebrew's RuboCop config, so
# it is the only one where the `FormulaAudit` cops actually run. The tap uses it
# for that reason and this copy matches, keeping one canonical shape in one
# canonical location.
HOMEBREW_FORMULA = Path("Formula/angkorfetch.rb")

DOWNLOAD_BASE = f"https://github.com/{REPO}/releases/download"

# (os, arch) -> release artifact. Explicit rather than inferred: a filename
# whose architecture does not match its block installs a binary that cannot run,
# and a checksum check cannot catch that because the file is intact.
BREW_ARTIFACTS: dict[tuple[str, str], str] = {
    ("macos", "arm"): "angkorfetch-macos-aarch64.tar.gz",
    ("macos", "intel"): "angkorfetch-macos-x86_64.tar.gz",
    ("linux", "arm"): "angkorfetch-linux-aarch64.tar.gz",
    ("linux", "intel"): "angkorfetch-linux-x86_64.tar.gz",
}
BREW_OSES = ("macos", "linux")
BREW_ARCHES = ("arm", "intel")

# A release download URL, split so the tag can be swapped while the trailing
# artifact filename is captured for checksum lookup.
DOWNLOAD_URL = re.compile(
    r"(?P<prefix>https://github\.com/[^/\s\"]+/[^/\s\"]+/releases/download/)"
    r"(?P<tag>[^/\s\"]+)/"
    r"(?P<filename>[^/\s\"]+)"
)

RE_WINGET_VERSION = re.compile(r'^(?P<lead>PackageVersion:\s*")(?P<value>[^"]*)(?P<tail>".*)$', re.S)
RE_WINGET_SHA = re.compile(
    r"^(?P<lead>\s*InstallerSha256:\s*)(?P<value>[0-9a-fA-F]+)(?P<tail>\s*)$", re.S
)
RE_SEMVER = re.compile(r"^\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.\-]+)?$")

# Used only by --check, to recover the version and digests a committed formula
# declares so it can be re-rendered and compared.
RE_BREW_URL = re.compile(
    r'^\s*url\s+"' + re.escape(DOWNLOAD_BASE) + r'/v(?P<version>[^/"]+)/(?P<filename>[^"]+)"'
)
RE_BREW_SHA256 = re.compile(r'^\s*sha256\s+"(?P<digest>[0-9a-f]{64})"')


class SyncError(Exception):
    """A condition that should fail the job rather than silently produce a bad manifest."""


def parse_checksums(path: Path) -> dict[str, str]:
    """Parse ``sha256sum`` output into {filename: sha256}."""
    sums: dict[str, str] = {}
    for lineno, raw in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        line = raw.strip()
        if not line:
            continue
        parts = line.split(None, 1)
        if len(parts) != 2:
            raise SyncError(f"{path}:{lineno}: cannot parse checksum line: {raw!r}")
        digest, name = parts[0].lower(), parts[1].strip().lstrip("*")
        if not re.fullmatch(r"[0-9a-f]{64}", digest):
            raise SyncError(f"{path}:{lineno}: not a sha256 digest: {parts[0]!r}")
        if name in sums and sums[name] != digest:
            raise SyncError(f"{path}: conflicting digests for {name}")
        sums[name] = digest
    if not sums:
        raise SyncError(f"{path}: no checksums found")
    return sums


def read_lines(path: Path) -> list[str]:
    """Read keeping original line endings so rewrites cause no whitespace churn."""
    with path.open("r", encoding="utf-8", newline="") as handle:
        return handle.read().splitlines(keepends=True)


def write_lines(path: Path, lines: list[str]) -> None:
    with path.open("w", encoding="utf-8", newline="") as handle:
        handle.write("".join(lines))


def _next_code_line(lines: list[str], start: int) -> int | None:
    for idx in range(start, len(lines)):
        if lines[idx].strip():
            return idx
    return None


def _swap_tag(line: str, tag: str) -> tuple[str, list[str]]:
    """Rewrite the tag segment of every release URL on the line."""
    filenames: list[str] = []

    def repl(match: re.Match[str]) -> str:
        filenames.append(match.group("filename"))
        return f"{match.group('prefix')}{tag}/{match.group('filename')}"

    return DOWNLOAD_URL.sub(repl, line), filenames


def _substitute(line: str, pattern: re.Pattern[str], value: str) -> str:
    match = pattern.match(line)
    if not match:
        raise SyncError(f"expected pattern {pattern.pattern!r} to match: {line!r}")
    return f"{match.group('lead')}{value}{match.group('tail')}"


def detect_newline(path: Path) -> str:
    """Return the line ending already used by a file, defaulting to LF."""
    if not path.is_file():
        return "\n"
    with path.open("rb") as handle:
        blob = handle.read()
    return "\r\n" if b"\r\n" in blob else "\n"


def render_homebrew_formula(version: str, sums: dict[str, str], newline: str = "\n") -> str:
    """Render the formula in the shape audited in the AMRSKH/homebrew-tap repo.

    Deliberately omits a ``version`` stanza and uses ``on_arm``/``on_intel``
    rather than ``Hardware::CPU`` conditionals; both are hard requirements of
    ``brew audit --strict``. Because the whole file is produced here, neither can
    be reintroduced by a sync.
    """
    tag = f"v{version}"

    os_blocks: list[str] = []
    for os_name in BREW_OSES:
        arch_blocks: list[str] = []
        for arch in BREW_ARCHES:
            filename = BREW_ARTIFACTS[(os_name, arch)]
            if filename not in sums:
                raise SyncError(
                    f"{filename} is required by the Homebrew formula but absent from checksums.txt"
                )
            arch_blocks.append(
                f"    on_{arch} do\n"
                f'      url "{DOWNLOAD_BASE}/{tag}/{filename}"\n'
                f'      sha256 "{sums[filename]}"\n'
                f"    end"
            )
        os_blocks.append(f"  on_{os_name} do\n" + "\n\n".join(arch_blocks) + "\n  end")

    body = "\n\n".join(os_blocks)
    text = f"""class Angkorfetch < Formula
  desc "Fast, cross-platform system fetch tool"
  homepage "https://github.com/{REPO}"
  # No explicit `version` stanza: Homebrew scans it from the vX.Y.Z path segment
  # of the URLs below, and `brew audit --strict` rejects restating it.
  license "MIT"

{body}

  def install
    bin.install "angkorfetch"
  end

  test do
    assert_match "AngkorFetch", shell_output("#{{bin}}/angkorfetch --version")
  end
end
"""
    if newline != "\n":
        text = text.replace("\n", newline)
    return text


def update_homebrew(root: Path, version: str, sums: dict[str, str]) -> list[str]:
    """Render the formula from scratch, preserving the file's existing line endings."""
    path = root / HOMEBREW_FORMULA
    if not path.is_file():
        raise SyncError(f"missing {path}")

    rendered = render_homebrew_formula(version, sums, detect_newline(path))
    with path.open("w", encoding="utf-8", newline="") as handle:
        handle.write(rendered)
    return [BREW_ARTIFACTS[(os_name, arch)] for os_name in BREW_OSES for arch in BREW_ARCHES]


def parse_homebrew_formula(text: str) -> tuple[str, dict[str, str]]:
    """Recover the version and {filename: sha256} a formula declares.

    Each ``url`` must be followed by its ``sha256``; that pairing is what makes a
    formula correct, so a file that violates it is rejected rather than guessed at.
    """
    lines = text.splitlines()
    versions: set[str] = set()
    sums: dict[str, str] = {}

    for idx, line in enumerate(lines):
        url = RE_BREW_URL.match(line)
        if not url:
            continue
        versions.add(url.group("version"))
        filename = url.group("filename")

        following = next((lines[i] for i in range(idx + 1, len(lines)) if lines[i].strip()), None)
        digest = RE_BREW_SHA256.match(following) if following else None
        if not digest:
            raise SyncError(f"no sha256 line follows the url for {filename}")
        sums[filename] = digest.group("digest")

    if not sums:
        raise SyncError("no release URLs found in the formula")
    if len(versions) != 1:
        raise SyncError(f"formula mixes versions: {sorted(versions)}")
    return versions.pop(), sums


def check_homebrew(root: Path) -> None:
    """Fail if the committed formula is not exactly what the generator would emit.

    Re-renders using the version and digests the file itself declares, so this
    validates *shape* independently of which release it points at. It therefore
    catches a hand edit, a reordered block, a reintroduced ``version`` stanza or a
    url/sha256 pair that drifted apart.
    """
    path = root / HOMEBREW_FORMULA
    if not path.is_file():
        raise SyncError(f"missing {path}")

    actual = path.read_text(encoding="utf-8", newline="")
    version, sums = parse_homebrew_formula(actual)
    expected = render_homebrew_formula(version, sums, detect_newline(path))
    if actual == expected:
        print(f"{HOMEBREW_FORMULA.as_posix()} matches the generated shape (v{version})")
        return

    diff = difflib.unified_diff(
        expected.splitlines(),
        actual.splitlines(),
        fromfile="expected (generated)",
        tofile=f"actual ({HOMEBREW_FORMULA.as_posix()})",
        lineterm="",
    )
    raise SyncError(
        "the committed Homebrew formula is not what the generator produces.\n"
        "Regenerate it with scripts/sync_package_manifests.py rather than editing by hand.\n"
        + "\n".join(diff)
    )


def _winget_dir(root: Path, version: str) -> Path:
    """Resolve the manifest directory, renaming the previous version if needed."""
    base = root / WINGET_BASE
    if not base.is_dir():
        raise SyncError(f"missing {base}")

    target = base / version
    if target.is_dir():
        return target

    existing = sorted(p for p in base.iterdir() if p.is_dir())
    if len(existing) != 1:
        raise SyncError(
            f"{base}: expected exactly one version directory to rename to {version}, "
            f"found {[p.name for p in existing]}"
        )
    existing[0].rename(target)
    return target


def update_winget(root: Path, version: str, sums: dict[str, str]) -> list[str]:
    """Rewrite PackageVersion across manifests plus the installer url and hash."""
    directory = _winget_dir(root, version)
    tag = f"v{version}"
    manifests = sorted(directory.glob("*.yaml"))
    if not manifests:
        raise SyncError(f"{directory}: no manifests found")

    touched: list[str] = []
    saw_installer = False

    for path in manifests:
        lines = read_lines(path)
        for idx, line in enumerate(lines):
            if RE_WINGET_VERSION.match(line):
                lines[idx] = _substitute(line, RE_WINGET_VERSION, version)
                continue

            if "InstallerUrl:" in line and "releases/download/" in line:
                new_line, filenames = _swap_tag(line, tag)
                if len(filenames) != 1:
                    raise SyncError(f"{path}: expected one artifact URL on line: {line!r}")
                lines[idx] = new_line
                filename = filenames[0]
                if filename not in sums:
                    raise SyncError(
                        f"{path}: {filename} is referenced by the manifest but absent "
                        "from checksums.txt"
                    )
                sha_idx = _next_code_line(lines, idx + 1)
                if sha_idx is None or not RE_WINGET_SHA.match(lines[sha_idx]):
                    raise SyncError(f"{path}: no InstallerSha256 follows the url for {filename}")
                lines[sha_idx] = _substitute(lines[sha_idx], RE_WINGET_SHA, sums[filename])
                touched.append(filename)
                saw_installer = True

        write_lines(path, lines)

    if not saw_installer:
        raise SyncError(f"{directory}: no InstallerUrl found; refusing to write unverifiable manifests")

    return touched


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--version", help="release version without a leading v")
    parser.add_argument("--checksums", type=Path, help="path to checksums.txt")
    parser.add_argument("--repo-root", default=Path("."), type=Path)
    parser.add_argument(
        "--summary-out", type=Path, help="write a markdown summary of updated artifacts here"
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="verify the committed Homebrew formula matches the generated shape, then exit",
    )
    args = parser.parse_args(argv)

    if args.check:
        if args.version or args.checksums:
            parser.error("--check inspects the committed file and takes no --version/--checksums")
        check_homebrew(args.repo_root)
        return 0

    if not args.version or not args.checksums:
        parser.error("--version and --checksums are required unless --check is given")

    version = args.version.lstrip("v")
    if not RE_SEMVER.match(version):
        raise SyncError(f"{args.version!r} is not a recognisable version")
    if not args.checksums.is_file():
        raise SyncError(f"missing checksums file {args.checksums}")

    sums = parse_checksums(args.checksums)
    brew = update_homebrew(args.repo_root, version, sums)
    winget = update_winget(args.repo_root, version, sums)

    # The formula was just rendered, so this is a self-check on the renderer
    # rather than on the file: it fails loudly if the template and the parser ever
    # disagree, instead of committing something audit would reject later.
    check_homebrew(args.repo_root)

    print(f"synced package definitions to {version}")
    for filename in brew:
        print(f"  homebrew  {filename}  {sums[filename]}")
    for filename in winget:
        print(f"  winget    {filename}  {sums[filename]}")

    if args.summary_out:
        rows = [f"| `{f}` | `{sums[f]}` | Homebrew |" for f in brew]
        rows += [f"| `{f}` | `{sums[f]}` | winget |" for f in winget]
        args.summary_out.write_text(
            "| artifact | sha256 | consumer |\n| --- | --- | --- |\n" + "\n".join(rows) + "\n",
            encoding="utf-8",
        )

    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except SyncError as error:
        print(f"error: {error}", file=sys.stderr)
        sys.exit(1)
