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
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = "AMRSKH/angkorfetch"
WINGET_BASE = Path("winget-pkgs/manifests/a/AMRSKH/AngkorFetch")
HOMEBREW_FORMULA = Path("HomebrewFormula/angkorfetch.rb")

# A release download URL, split so the tag can be swapped while the trailing
# artifact filename is captured for checksum lookup.
DOWNLOAD_URL = re.compile(
    r"(?P<prefix>https://github\.com/[^/\s\"]+/[^/\s\"]+/releases/download/)"
    r"(?P<tag>[^/\s\"]+)/"
    r"(?P<filename>[^/\s\"]+)"
)

RE_BREW_VERSION = re.compile(r'^(?P<lead>\s*version\s+")(?P<value>[^"]*)(?P<tail>".*)$', re.S)
RE_BREW_SHA = re.compile(r'^(?P<lead>\s*sha256\s+")(?P<value>[0-9a-fA-F]*)(?P<tail>".*)$', re.S)
RE_WINGET_VERSION = re.compile(r'^(?P<lead>PackageVersion:\s*")(?P<value>[^"]*)(?P<tail>".*)$', re.S)
RE_WINGET_SHA = re.compile(
    r"^(?P<lead>\s*InstallerSha256:\s*)(?P<value>[0-9a-fA-F]+)(?P<tail>\s*)$", re.S
)
RE_SEMVER = re.compile(r"^\d+\.\d+\.\d+(?:[-+][0-9A-Za-z.\-]+)?$")


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


def update_homebrew(root: Path, version: str, sums: dict[str, str]) -> list[str]:
    """Rewrite the formula's version and every url/sha256 pair."""
    path = root / HOMEBREW_FORMULA
    if not path.is_file():
        raise SyncError(f"missing {path}")

    tag = f"v{version}"
    lines = read_lines(path)
    touched: list[str] = []

    for idx, line in enumerate(lines):
        if RE_BREW_VERSION.match(line):
            lines[idx] = _substitute(line, RE_BREW_VERSION, version)
            continue

        if "url " not in line or "releases/download/" not in line:
            continue

        new_line, filenames = _swap_tag(line, tag)
        if len(filenames) != 1:
            raise SyncError(f"{path}: expected exactly one artifact URL on line: {line!r}")
        lines[idx] = new_line

        filename = filenames[0]
        if filename not in sums:
            raise SyncError(
                f"{path}: {filename} is referenced by the formula but absent from checksums.txt"
            )

        # Homebrew always pairs a url with the sha256 on the following code line.
        sha_idx = _next_code_line(lines, idx + 1)
        if sha_idx is None or not RE_BREW_SHA.match(lines[sha_idx]):
            raise SyncError(f"{path}: no sha256 line follows the url for {filename}")
        lines[sha_idx] = _substitute(lines[sha_idx], RE_BREW_SHA, sums[filename])
        touched.append(filename)

    if not touched:
        raise SyncError(f"{path}: no release URLs found; refusing to write a formula I cannot verify")

    write_lines(path, lines)
    return touched


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
    parser.add_argument("--version", required=True, help="release version without a leading v")
    parser.add_argument("--checksums", required=True, type=Path, help="path to checksums.txt")
    parser.add_argument("--repo-root", default=Path("."), type=Path)
    parser.add_argument(
        "--summary-out", type=Path, help="write a markdown summary of updated artifacts here"
    )
    args = parser.parse_args(argv)

    version = args.version.lstrip("v")
    if not RE_SEMVER.match(version):
        raise SyncError(f"{args.version!r} is not a recognisable version")
    if not args.checksums.is_file():
        raise SyncError(f"missing checksums file {args.checksums}")

    sums = parse_checksums(args.checksums)
    brew = update_homebrew(args.repo_root, version, sums)
    winget = update_winget(args.repo_root, version, sums)

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
