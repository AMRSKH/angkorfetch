Author: AI Agent (Claude Code)
Date: 2026-11-11
Task: TASK-011
Status: In Review

# TASK-011: Update Package Version Pins And Fix The RPM Build Helper

## Objective

Move the hand-set package definitions from 1.1.1 to the current release 1.2.1, and make
`linux/rpm/build-rpm.sh` produce a correct source archive, so the source-build definitions match
the shipped version and the helper is usable.

## Background

TASK-009 and TASK-010 recorded these as follow-ups. The maintainer approved fixing them as
TASK-011 (2026-11-11, "Yes, TASK-011").

Facts checked by the main session on 2026-11-11:

| Item | Fact |
|---|---|
| `Cargo.toml` version | 1.2.1 |
| GitHub release `v1.2.1` | Published 2026-11-02; tag `v1.2.1` is commit `ebf5823`, an ancestor of this branch |

Pins still at 1.1.1:

| File | Line | Field |
|---|---|---|
| `linux/arch/PKGBUILD` | 3 | `pkgver` |
| `linux/deb/build-deb.sh` | 4 | `VERSION` |
| `linux/rpm/build-rpm.sh` | 4 | `VERSION` |
| `linux/rpm/angkorfetch.spec` | 2, 35 | `Version:`; no 1.2.1 `%changelog` entry |
| `snap/snapcraft.yaml` | 3, 27 | `version`, `source-tag` |
| `flatpak/io.github.AMRSKH.angkorfetch.yml` | 20 | `tag` |
| `flatpak/io.github.AMRSKH.angkorfetch.metainfo.xml` | 21 | newest `<release>` |

`linux/rpm/build-rpm.sh` defects (recorded in TASK-009, read only, never run):

1. From the repository root it fails at the relative `cp "$SPEC"` (line 12).
2. From `linux/rpm/`, `git archive ... HEAD` (line 10) packs only that subdirectory, so the archive
   has no `Cargo.toml`.
3. It writes `angkorfetch-<version>.tar.gz`, but `rpmbuild` looks for the basename of the spec's
   `Source0` URL (`v<version>.tar.gz`, spec line 8).

## Requirements

- Every pin in the table above equals 1.2.1 (`v1.2.1` for tags), and the release metadata uses the
  real release date, 2026-11-02.
- `linux/rpm/build-rpm.sh` works when run from any directory: it archives the whole repository and
  names the source file as the spec's `Source0` expects. Keep `set -euo pipefail`.
- `docs/PACKAGING.md` section 3 no longer says the helper is broken in ways this task fixes, and
  states honestly what remains unverified.
- Release `.deb` / `.rpm` production in CI is unaffected.
- No release-pipeline, workflow, or installer change.

## Scope

- `linux/arch/PKGBUILD` (`pkgver` only), `linux/deb/build-deb.sh`, `linux/rpm/build-rpm.sh`,
  `linux/rpm/angkorfetch.spec`, `snap/snapcraft.yaml`,
  `flatpak/io.github.AMRSKH.angkorfetch.yml`, `flatpak/io.github.AMRSKH.angkorfetch.metainfo.xml`
- `docs/PACKAGING.md` section 3 (and any other sentence this task makes false)

## Out of Scope

- `linux/arch/PKGBUILD` `sha256sums=('SKIP')`, the Flatpak Rust SDK extension and vendored sources,
  a homebrew-core formula (separate follow-ups).
- `Formula/`, `winget-pkgs/`, any SHA256 value (AGENTS.md section 7); merging
  `origin/automation/sync-packages-v1.2.1`.
- `.github/workflows/`, `scripts/`, `src/`, `Cargo.*`, installers, `README.md`.
- Publishing or submitting anywhere (external actions).

## Assigned Roles

- Advisor: `advisor` subagent (Claude Code).
- Dev: `dev` subagent (Claude Code).
- QA Review: `qa-review` subagent (Claude Code), independent of Dev.
- Docs: `docs` subagent (Claude Code).
- Orchestration, final validation, commits, and the final report: main session.

## Implementation Plan

Advisor, 2026-11-11. Repository facts were read on this branch (`claude/awesome-pascal-f6d9aa`,
HEAD `f49f08f`). Line numbers are guides; match on the old text.

### Approach for `build-rpm.sh`

| Option | Result | Verdict |
|---|---|---|
| **A. Resolve the script's directory, take the root from `git rev-parse --show-toplevel`, archive with `git -C <root>`, name the file `v$VERSION.tar.gz`, copy the spec from the script's directory** | Two new lines, three changed lines. Works from any directory. Same `SCRIPT_DIR` idiom as `linux/arch/build-arch.sh:4`. Output name and prefix match the spec | **Recommended** |
| B. Archive the tag `v$VERSION` instead of `HEAD` | Exact release source, but fails during a release bump (RELEASING "Cutting a release" step 1 sets `VERSION` before the tag exists) and on clones without the tag | Rejected |
| C. Download `Source0` with `spectool -g` | Adds a tool and a network step, needs the published tag, and can never test unreleased commits | Rejected |

Rules for Dev:

- Edit only the files listed below (plus this task file). No other change to any of them.
- **Line endings** (`git ls-files --eol`, with `core.autocrlf=true` here):

  | Files | Index | Working tree | What to do |
  |---|---|---|---|
  | `linux/deb/build-deb.sh`, `linux/rpm/build-rpm.sh` | LF | LF (`.gitattributes`: `*.sh text eol=lf`) | Keep LF. They run directly in Git Bash; no conversion is needed |
  | `PKGBUILD`, `angkorfetch.spec`, `snapcraft.yaml`, Flatpak `.yml`, `.metainfo.xml`, `docs/PACKAGING.md` | LF | CRLF | Keep CRLF in the working tree, including every new line. After `git add`, each must still show `i/lf` |

- Keep the file mode of `build-rpm.sh` (`100644`; it is invoked with `bash`).
- `Formula/`, `winget-pkgs/`, and every SHA256 stay untouched. `sha256sums=('SKIP')` stays.

### Convention decisions

| Field | Decision | Reason |
|---|---|---|
| `pkgrel=1` (`PKGBUILD:4`) | Stay | Already 1, the correct value for a new upstream version |
| `Release:  1%{?dist}` (spec:3) | Stay | Same; the `%changelog` suffix is `-1` |
| `grade: stable` (`snapcraft.yaml:11`) | Stay | v1.2.1 is a stable release, not a prerelease |
| Metainfo 1.1.1 and older entries | Keep | The file keeps history, newest first |
| Spec older `%changelog` entries | Keep | Same |

CI impact: none. `version-check` (`release.yml:17-32`) compares only the tag with `Cargo.toml`. The
`packages` job writes its own spec and takes its version from the tag (lines 227-278). `git grep`
over `scripts/` and `.github/` finds no reader of `linux/`, `flatpak/`, or `snap/`.

### Version pins

| File | Line | Old | New |
|---|---|---|---|
| `linux/arch/PKGBUILD` | 3 | `pkgver=1.1.1` | `pkgver=1.2.1` |
| `linux/deb/build-deb.sh` | 4 | `VERSION="1.1.1"` | `VERSION="1.2.1"` |
| `linux/rpm/angkorfetch.spec` | 2 | `Version:  1.1.1` | `Version:  1.2.1` |
| `snap/snapcraft.yaml` | 3 | `version: "1.1.1"` | `version: "1.2.1"` |
| `snap/snapcraft.yaml` | 27 | `    source-tag: v1.1.1` | `    source-tag: v1.2.1` |
| `flatpak/io.github.AMRSKH.angkorfetch.yml` | 20 | `        tag: v1.1.1` | `        tag: v1.2.1` |

### `linux/rpm/angkorfetch.spec` `%changelog`

Insert these two lines plus one blank line directly after `%changelog` (line 34), above the 1.1.1
entry. Copy the author and email from existing line 35 byte for byte; add no new personal data.

```text
* Mon Nov 02 2026 <author and email exactly as on line 35> - 1.2.1-1
- Fix battery, display, GPU, and package detection on Linux, macOS, and Windows
```

The weekday was verified: 2026-11-02 is a Monday. The text comes from `3376c8c` (the v1.2.1 change
to the shipped binary). Tag `v1.1.1` exists neither locally nor on `origin`, so the range was read
as `ad0b1d9..v1.2.1`, where `ad0b1d9` is the "Bump to v1.1.1" commit. The other commits in that
range change installers, CI, or docs, which the RPM does not contain. No `%` in the text, because
rpm would expand it as a macro.

### `flatpak/io.github.AMRSKH.angkorfetch.metainfo.xml`

Insert above line 21, same indentation:

```xml
    <release version="1.2.1" date="2026-11-02" />
```

### `linux/rpm/build-rpm.sh`

Only line 4 (`VERSION`), line 10 (archive), and line 12 (`cp`) change, and `SCRIPT_DIR` and
`REPO_ROOT` are added after line 6. Exact result:

```bash
#!/usr/bin/env bash
set -euo pipefail

VERSION="1.2.1"
SPEC="angkorfetch.spec"
RPMBUILD_DIR="$HOME/rpmbuild"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel)"

mkdir -p "$RPMBUILD_DIR"/{SOURCES,SPECS,RPMS,SRPMS,BUILD}

git -C "$REPO_ROOT" archive --format=tar.gz -o "$RPMBUILD_DIR/SOURCES/v$VERSION.tar.gz" --prefix="angkorfetch-$VERSION/" HEAD

cp "$SCRIPT_DIR/$SPEC" "$RPMBUILD_DIR/SPECS/"

rpmbuild -ba "$RPMBUILD_DIR/SPECS/$SPEC"

echo "RPMS built at: $RPMBUILD_DIR/RPMS/"
```

Why each part is correct:

| Need | How it is met | Verified |
|---|---|---|
| Whole repository | `git -C <root> archive`; run from a subdirectory, `git archive` packs only that subdirectory (seen: just `angkorfetch.spec` and `build-rpm.sh`) | Prototype run in the scratchpad from a directory outside the repository: `Cargo.toml`, `Cargo.lock`, `README.md`, `LICENSE`, `src/main.rs`, and the spec are present |
| `Source0` name | The spec's `Source0` basename is `v%{version}.tar.gz`, and `rpmbuild` looks there in `%_sourcedir` (`~/rpmbuild/SOURCES` by default) | By reading; `rpmbuild` is unavailable here |
| `%setup -q` | Default directory is `%{name}-%{version}`, which is the prefix `angkorfetch-1.2.1/` | One top directory confirmed in the prototype; `%setup` itself not run |
| Any working directory | Script-relative `SCRIPT_DIR` | Stubbed test below |

`HEAD` versus the tag: archiving `HEAD` is acceptable, because the helper is a pre-tag test tool
(`VERSION` is bumped before the tag exists). Copr and Fedora take the source from the `Source0` URL
and do not use it. The result is only the published source when `HEAD` is the tag. Today `HEAD`
differs from `v1.2.1` in `src/main.rs` by two `#[cfg(any(windows, test))]` lines. The docs must say
this, and must not tell readers to check out `v1.2.1` and run the helper: that tag still contains
the old broken script, so the fix first ships with the next release. Uncommitted changes are not
archived, while the spec is copied from the working tree.

### `docs/PACKAGING.md` section 3

Lines 129 and 140 onward stay. Replace lines 131-138 with:

````markdown
`linux/rpm/angkorfetch.spec` and `linux/rpm/build-rpm.sh` build from source, for Copr or a Fedora review. They are not run in CI. The helper needs `git`, `rpmbuild`, and the `cargo` RPM, and runs from any directory:

```bash
bash linux/rpm/build-rpm.sh
```

- It archives the whole repository at the committed `HEAD` (uncommitted changes are left out) into `~/rpmbuild/SOURCES/v<version>.tar.gz`, the file name at the end of the spec's `Source0` URL, under the top directory `angkorfetch-<version>/` that `%setup -q` expects. The archive equals the published `v<version>` source only when `HEAD` is that tag.
- A full `rpmbuild` run with these files has not been verified; only the archive step has been tested, with `rpmbuild` stubbed. The spec's `cargo build` downloads crates, so the build needs network access.
- Both files set the version by hand (`VERSION`, `Version:`); keep them in step with `Cargo.toml` (see [RELEASING.md](../RELEASING.md#cutting-a-release), step 1).
````

Other sentences checked; none becomes false, so no other doc edit:

| Location | Finding |
|---|---|
| `docs/PACKAGING.md` line 125, section 4 (`VERSION` by hand), section 5 (`pkgver` archive), section 6, section 7 | Still true |
| Root `RELEASING.md` "Cutting a release" step 1 | Still the correct list |
| Root `RELEASING.md` "Why package definitions lag" | Covers only `Formula/` and winget; still true |
| `docs/MAINTAINING.md` section 3 step 1, section 4 "Fedora Copr" | Still true |

### Risks

| Risk | Mitigation |
|---|---|
| A CRLF line in a `.sh` file breaks Bash | `.gitattributes` forces LF; `bash -n` and `git ls-files --eol` confirm |
| Mixed line endings in a CRLF working file | Dev keeps CRLF on new lines; `git diff --check` and `i/lf` confirm |
| Docs claim more than was tested | The text says a full `rpmbuild` is unverified; the stubbed test below is what backs "only the archive step" |
| Spec still fails under real `rpmbuild` | Out of this task's objective (correct source archive). See "Recorded only" |
| Pins point at a tag that does not exist | `v1.2.1` confirmed on `origin` (`ebf5823`); `Cargo.toml` and `Cargo.lock` at the tag both say 1.2.1, so `--locked` builds agree |
| Release `.deb` / `.rpm` affected | The `packages` job reads none of these files |

### Test plan (additions to Testing Requirements)

Stub `rpmbuild` and run the real script from three directories. Use a POSIX scratchpad path
(`cygpath -u`): MSYS `tar` reads `C:/...` as a remote host and fails (seen during planning).

```bash
R="$(git rev-parse --show-toplevel)"
S="$(cygpath -u '<scratchpad>')"
mkdir -p "$S/stub" "$S/elsewhere"
printf '#!/usr/bin/env bash\nprintf "%%s\\n" "$@" > "$HOME/rpmbuild.args"\n' > "$S/stub/rpmbuild"
# Repeat with a fresh "$S/h" for each: (a) cd "$R"; bash linux/rpm/build-rpm.sh
# (b) cd "$R/linux/rpm"; bash build-rpm.sh   (c) cd "$S/elsewhere"; bash "$R/linux/rpm/build-rpm.sh"
# each run with: HOME="$S/h" PATH="$S/stub:$PATH"
```

After each run confirm:

- `$S/h/rpmbuild/SOURCES/v1.2.1.tar.gz` exists and `angkorfetch-1.2.1.tar.gz` does not.
- `tar tzf` lists `angkorfetch-1.2.1/` + `Cargo.toml`, `Cargo.lock`, `README.md`, `LICENSE`, and
  `cut -d/ -f1 | sort -u` gives only `angkorfetch-1.2.1`.
- `$S/h/rpmbuild/SPECS/angkorfetch.spec` exists; `rpmbuild.args` holds `-ba` and that path.

Check names only, not archive bytes: the archive packs `HEAD`, so before the work commit its inner
files are the old versions, and text conversion can depend on the host's Git settings.

Other checks:

- `git grep -n '1\.1\.1' -- linux flatpak snap` gives exactly two lines: the spec's 1.1.1
  `%changelog` entry and the metainfo 1.1.1 `<release>`.
- XML: parse with `xml.etree.ElementTree`; the first `release` is version 1.2.1, date 2026-11-02.
- YAML: Python's standard library has no YAML parser and no package may be added, so confirm the
  three YAML lines with `git grep` and review the diff.
- `git ls-files --eol` on every changed file: `i/lf`; `.sh` files `w/lf`.
- `git diff --stat`: exactly the eight files of this plan (`PKGBUILD`, two helpers, spec, snap, two
  Flatpak files, `docs/PACKAGING.md`).
- Not runnable here, and to be reported as not run: `rpmbuild`, `dpkg-deb`, `makepkg`, `snapcraft`,
  `flatpak-builder`.

### Recorded only (not fixed here)

| Finding | Effect | Certainty |
|---|---|---|
| `Cargo.toml` has `strip = true`, and the spec does not disable the debuginfo package | On Fedora, `rpmbuild` may stop with an empty `debugsourcefiles.list`; the usual fix is `%global debug_package %{nil}` | Not sure; needs a real Fedora `rpmbuild` |
| `BuildRequires` lists only `cargo` (no `rust` or C compiler) | From memory, Fedora's `cargo` pulls in `rust`, and `rust` pulls in a C compiler | Not sure; not checkable here |
| `Requires: gcc-c++` (spec:11) | Does not block a build. A compiler is a wrong runtime dependency for a Rust binary and would be installed for users | Certain by reading; the dependency's purpose is unknown |
| `cargo build --locked` with no vendored crates | Needs network access; fails in an offline builder such as Fedora Koji. Copr needs network enabled | Certain by reading for the network need; builder settings not checked |
| `%license LICENSE`, `README.md`, `Cargo.lock` | Present in the archive; no defect | Verified |
| Tag `v1.1.1` is absent locally and on `origin` | The old pins named a missing tag. The `gh workflow run ... tag=v1.1.1` and `gh release download v1.1.1` examples in root `RELEASING.md` (lines 235, 238, 348) would fail if run literally | Verified for the tag |
| `docs/MAINTAINING.md` section 3 step 1 omits `linux/arch/PKGBUILD`, which root `RELEASING.md` step 1 lists | Pin could be missed at the next release | Verified |

### Approvals

None. No dependency, workflow, push, tag, or external action. No maintainer decision is needed.

## Dependencies

- TASK-009 and TASK-010 (follow-up lists, `docs/PACKAGING.md` section 3).
- Tag `v1.2.1` on `origin` (`ebf5823`, confirmed), which `Source0`, `PKGBUILD`, snap, and Flatpak
  now reference.
- No new tools or packages. Real package builds need Linux tools absent from this host.

## Acceptance Criteria

- [x] No 1.1.1 pin remains in `linux/`, `flatpak/`, or `snap/` except historical changelog or release entries
- [x] Spec `%changelog` and metainfo have a 1.2.1 entry with the real release date
- [x] `build-rpm.sh` archives the whole repository from any working directory
- [x] The source archive name matches the spec's `Source0` basename, and its top directory matches `%setup`
- [x] `docs/PACKAGING.md` section 3 is accurate after the fix
- [x] `bash -n` passes on `linux/deb/build-deb.sh` and `linux/rpm/build-rpm.sh`
- [x] `python scripts/sync_package_manifests.py --check` and script tests still pass
- [x] Independent QA accepted

## Testing Requirements

- `git grep -n '1\.1\.1' -- linux flatpak snap`: only historical entries remain.
- `bash -n` on both helpers.
- Exercise the archive step of `build-rpm.sh` from the repository root and from `linux/rpm/` with
  `rpmbuild` absent or stubbed and `HOME` pointed at the scratchpad; list the archive and confirm
  `Cargo.toml`, `Cargo.lock`, `README.md`, `LICENSE`, and the name `v1.2.1.tar.gz`.
- Parse the YAML and XML files with Python's standard library where possible.
- `python scripts/sync_package_manifests.py --check`
- `python -m unittest discover -s scripts -p "test_*.py"`
- `git diff --stat` shows only files in scope; `git diff --check` is clean.

### Dev results (2026-11-11, `dev` subagent, Claude Code)

Implemented the Implementation Plan as written, with no deviation and no plan correction needed.
Nothing was staged or committed. Edits were byte-exact, so CRLF working-tree files stayed CRLF. The
new spec `%changelog` author and email were copied programmatically from the 1.1.1 entry.

Files changed (8, plus this task file): `linux/arch/PKGBUILD`, `linux/deb/build-deb.sh`,
`linux/rpm/build-rpm.sh`, `linux/rpm/angkorfetch.spec`, `snap/snapcraft.yaml`,
`flatpak/io.github.AMRSKH.angkorfetch.yml`, `flatpak/io.github.AMRSKH.angkorfetch.metainfo.xml`,
`docs/PACKAGING.md` (section 3, the planned replacement only).

Stubbed test of the real `linux/rpm/build-rpm.sh`: `rpmbuild` replaced by a stub on `PATH`, `HOME`
in the session scratchpad and recreated for each run. No `~/rpmbuild` exists in the real home after
the runs.

| Check | (a) repo root | (b) `linux/rpm/` | (c) outside the repo |
|---|---|---|---|
| Exit code | 0 | 0 | 0 |
| `SOURCES/v1.2.1.tar.gz` exists; `angkorfetch-1.2.1.tar.gz` absent | Pass | Pass | Pass |
| Archive has `angkorfetch-1.2.1/` + `Cargo.toml`, `Cargo.lock`, `README.md`, `LICENSE`, `src/main.rs`, `linux/rpm/angkorfetch.spec` (133 entries) | Pass | Pass | Pass |
| `cut -d/ -f1 \| sort -u` gives only `angkorfetch-1.2.1` | Pass | Pass | Pass |
| `SPECS/angkorfetch.spec` exists and says `Version:  1.2.1` | Pass | Pass | Pass |
| Stub received `-ba` and `.../rpmbuild/SPECS/angkorfetch.spec` | Pass | Pass | Pass |

Other checks:

| Command / check | Result |
|---|---|
| `git grep -n '1\.1\.1' -- linux flatpak snap` | Pass: exactly 2 lines, the spec 1.1.1 `%changelog` entry and the metainfo 1.1.1 `<release>` |
| `xml.etree.ElementTree` parse of the metainfo | Pass: first release is 1.2.1, date 2026-11-02; 4 releases, newest first |
| YAML lines by `git grep` (no stdlib YAML parser) | Pass: `snapcraft.yaml:3` `version: "1.2.1"`, `:27` `source-tag: v1.2.1`, Flatpak `.yml:20` `tag: v1.2.1`; diff reviewed |
| Spec `%changelog` | Pass: author and email equal the 1.1.1 entry byte for byte; blank separator line; no `%` in the new text |
| `bash -n linux/deb/build-deb.sh`, `bash -n linux/rpm/build-rpm.sh` | Pass: exit 0 for both |
| `python scripts/sync_package_manifests.py --check` | Pass: exit 0, `Formula/angkorfetch.rb matches the generated shape (v1.1.1)` |
| `python -m unittest discover -s scripts -p "test_*.py"` | Pass: `Ran 35 tests`, `OK` |
| `git ls-files --eol` on the 8 files, before and after | Pass: unchanged. All `i/lf`; the 2 `.sh` files `w/lf`; the other 6 `w/crlf` |
| Byte count for mixed endings | Pass: 0 bare LF in the 6 CRLF files, 0 CR in the 2 `.sh` files |
| `build-rpm.sh` file mode | Pass: `100644` in the index; `git diff --summary` reports no mode change |
| `git diff --stat` | Pass: exactly the 8 planned files, 22 insertions, 15 deletions |
| `git diff --check` | Pass: exit 0, no output |
| `git status` for `Formula/`, `winget-pkgs/`, `.github/`, `scripts/`, `src/`, `Cargo.*`, installers, `README.md` | Pass: no changes; `sha256sums` line not in the diff |

Not verified:

| Item | Reason |
|---|---|
| Real `rpmbuild` (`Source0` lookup in `%_sourcedir`, `%setup -q`, the `cargo` build, the `%changelog` date check) | `rpmbuild` is not installed on this Windows host; the stub only records its arguments |
| `dpkg-deb`, `makepkg`, `snapcraft`, `flatpak-builder` | Not installed on this host; no package was built |
| YAML parsed by a real parser | Python's standard library has none and no package may be added |
| The helper on Linux | Run only in Git Bash on Windows |
| Archive file contents | Names only, by plan: the archive packs `HEAD`, which does not yet contain this task's edits |

`cargo fmt`, `clippy`, `test`, and `build` were not run: no Rust code or `Cargo.*` file changed.

## QA Results

### Round 1 (QA, 2026-11-11, qa-review subagent)

Verdict: Accepted

Independence: this review ran as an independent `qa-review` subagent in Claude Code, which did not
write the change. All results below were produced in this review. Reviewed at HEAD `f49f08f` with
the 8 files uncommitted. No Critical, High, or Medium findings. The two Lows below are accepted and
do not block the commit.

Findings (`Severity | file:line | finding`):

| Severity | File:line | Finding, evidence, fix |
|---|---|---|
| Low | `docs/PACKAGING.md:137` | "uncommitted changes are left out" is true of the archive, but the helper copies the spec from the working tree (`linux/rpm/build-rpm.sh:14`), so an uncommitted spec edit is used against the `HEAD` source. The Advisor plan named both halves; the doc has only the first. Fix when next touched: add "the spec is copied from the working tree". |
| Low | `linux/rpm/build-rpm.sh:6` | Pre-existing, not introduced here: the helper writes to `$HOME/rpmbuild`, while `rpmbuild` reads `%_topdir`. A user whose `~/.rpmmacros` overrides `%_topdir` would get a missing-source error (by reading; not run). `release.yml:278` already passes `--define "_topdir ..."`; the same would remove the assumption. |

Checks run (Git Bash on Windows; `rpmbuild` stubbed on `PATH`, `HOME` in the scratchpad):

| Check | Result |
|---|---|
| Spec `Source0` with `%{version}` expanded | Basename `v1.2.1.tar.gz`; `%setup -q` directory `angkorfetch-1.2.1` (from spec `Name` and `Version`) |
| Helper from repo root, `linux/rpm/`, outside the repo | Exit 0 for all three. `SOURCES/` holds only `v1.2.1.tar.gz`; one top directory, `angkorfetch-1.2.1`; stub received `-ba` and `SPECS/angkorfetch.spec`; copied spec says `Version:  1.2.1` |
| Whole repository archived | File list compared with `git ls-tree -r HEAD`: identical, 90 files (133 entries with directories) |
| Spaces in repo path and in `HOME` (clone in the scratchpad with the working-tree helper and spec) | Exit 0; same archive results; stub received the full path as one argument |
| Script outside a git checkout | `fatal: not a git repository`, exit 128, stops before `mkdir` (the test `HOME` stayed empty) |
| `rpmbuild` absent / stub exits 3 | Exit 127 / exit 3; the "RPMS built" line is not printed |
| Real home | `C:/Users/AMSDev/rpmbuild` does not exist after the runs |
| `bash -n linux/deb/build-deb.sh`, `bash -n linux/rpm/build-rpm.sh` | Exit 0, each |
| `git grep -n '1\.1\.1' -- linux flatpak snap`, plus plain `grep -rn` on the working tree | Exactly 2 lines, both history: `linux/rpm/angkorfetch.spec:38`, metainfo line 22. All 9 pin lines read 1.2.1 or `v1.2.1` |
| Metainfo parsed with `xml.etree.ElementTree` | 1.2.1 (2026-11-02), 1.1.1, 1.1.0, 1.0.1; dates newest first |
| `gh release view v1.2.1` (read-only) | Published `2026-11-02T06:38:40Z`, not draft or prerelease; matches the metainfo and changelog date |
| `%changelog` | `Mon Nov 02 2026` is a valid weekday (Python); valid RPM date form; newest first; older weekdays also valid; no `%` in the new text; author and email identical to the existing entry, so no new personal data; `1.2.1-1` matches `Version` and `Release` |
| Changelog text against `git log ad0b1d9..v1.2.1` | True of `3376c8c`: battery (Linux, macOS), display (Linux), GPU (Linux, Windows), package detection (`dpkg-query`, `snap list`). It is a summary, not exhaustive: `c8dde34` also changed macOS Wi-Fi detection in `src/main.rs`. Nothing stated is false |
| Tag `v1.2.1` for Flatpak, snap, PKGBUILD | Tag exists on `origin` (`ebf5823`) and is an ancestor of HEAD; holds `Cargo.toml` and `Cargo.lock` (both 1.2.1), `LICENSE`, `README.md`, `src/main.rs`. Release workflow run on that commit concluded `success`; the workflow at the tag has `cargo test --release --locked` |
| CI effects | `git grep` for `linux/`, `flatpak/`, `snap/`, `build-rpm`, `build-deb`, `PKGBUILD`, `snapcraft` in `.github` and `scripts`: no matches. `release.yml:263` writes its own spec; its `Version:` is `$VER` |
| Docs | `docs/PACKAGING.md` section 3 accurate: no claim of a full `rpmbuild`; old "need fixes" and `cd linux/rpm` text gone (`git grep`: no matches outside task files). No sentence in `RELEASING.md`, `docs/MAINTAINING.md`, `docs/DEVELOPMENT.md`, or other `docs/PACKAGING.md` sections is made false |
| `python scripts/sync_package_manifests.py --check` | Exit 0, `matches the generated shape (v1.1.1)` |
| `python -m unittest discover -s scripts -p "test_*.py"` | `OK` |
| `git diff --check` | Exit 0, no output |
| `git ls-files --eol` on the 8 files | All `i/lf`; 2 `.sh` files `w/lf`; other 6 `w/crlf`; none mixed |
| `git status --short`, `git diff --cached --stat`, `git diff --summary` | Only the 8 files plus this task file; nothing staged; no mode change |
| Out of scope paths | No change in `Formula/`, `winget-pkgs/`, `.github/`, `scripts/`, `src/`, `Cargo.*`, installers, `README.md`; `sha256sums` not in the diff |

Not verified:

| Item | Reason |
|---|---|
| Real `rpmbuild`: `Source0` lookup, `%setup -q`, `%changelog` date parsing, `cargo` build, and whether paths with spaces work inside `rpmbuild` itself | `rpmbuild` is not on this Windows host; the stub only records arguments. Conclusions come from reading the spec |
| Helper on real Linux | Read as Linux shell and run in Git Bash only |
| `dpkg-deb`, `makepkg`, `snapcraft`, `flatpak-builder` builds | Tools not on this host |
| YAML with a real parser | No parser in the Python standard library; the three lines were checked by `git grep` and diff |
| Whether the Advisor's "Recorded only" spec risks (debuginfo with `strip = true`, build dependencies) bite on Fedora | Needs a real Fedora build; `docs/PACKAGING.md:138` says the full build is unverified |
| `cargo fmt`, `clippy`, `test`, `build` | Not run: no Rust or `Cargo.*` change |

## Documentation Updates

Docs, 2026-11-11. This stage ran in the `docs` subagent (Claude Code).

No further documentation changes needed. `docs/PACKAGING.md` section 3 was updated by Dev and
reviewed by QA; this stage did not edit it. Searched for `build-rpm`, `build-deb`, `rpmbuild`,
`1.1.1`, `lag`, the changed paths, and the removed helper text (`need fixes`, `cd linux/rpm`,
`angkorfetch-<version>.tar.gz`; no matches outside task files).

| File checked | Result |
|---|---|
| `README.md` | No helper instructions. Lines 59-60 give Arch and Flatpak status only, still **Planned** |
| Root `RELEASING.md` | Step 1 (lines 121-126) is still the correct pin list. "Why package definitions lag" covers only `Formula/` and winget. The `1.1.1` mentions are command examples, a quoted audit message, and the tap, not these pins |
| `docs/RELEASING.md` | No mention of the helper or the pins; lists release assets from CI, unaffected |
| `docs/MAINTAINING.md` | Section 3 step 1 and section 4 say what to update and remain true |
| `docs/DEVELOPMENT.md` | Lines 25-37 only name the files and their purpose; still true |
| `CONTRIBUTING.md` | No mention of the helper or the pins |
| `docs/PACKAGING.md` sections 4-7 | Section 4 (run from the root, `VERSION` set by hand), section 5 (`pkgver` archive), section 6 (tag in `sources`, unverified), section 7 (`source-tag`) are still true. Status markers unchanged. The `1.1.1` mentions at lines 11, 15, 34, 86, and 323 describe crates.io, the tap, and the WinGet submission, not these pins |

Follow-ups (none fixed here):

| Source | Item |
|---|---|
| QA Low | `docs/PACKAGING.md:137`: add that the spec is copied from the working tree, when next touched |
| QA Low | `linux/rpm/build-rpm.sh:6`: helper assumes `$HOME/rpmbuild`; a custom `%_topdir` could cause a missing-source error (pre-existing) |
| Advisor | Spec may need `%global debug_package %{nil}` because of `strip = true` (unconfirmed; needs a Fedora build) |
| Advisor | `BuildRequires` lists only `cargo` (unconfirmed whether that is enough) |
| Advisor | `Requires: gcc-c++` (spec:11) is a wrong runtime dependency for a Rust binary |
| Advisor | `cargo build --locked` with no vendored crates needs network access; fails in offline builders such as Koji |
| Advisor | Tag `v1.1.1` does not exist, so the `v1.1.1` command examples in root `RELEASING.md` (lines 235, 238, 348) would fail if run literally |
| Advisor | `docs/MAINTAINING.md` section 3 step 1 omits `linux/arch/PKGBUILD`, which root `RELEASING.md` step 1 lists |

## Final Validation (main session, 2026-11-11)

| Check | Result |
|---|---|
| `python -m unittest discover -s scripts -p "test_*.py"` | Ran 35 tests, OK |
| `python scripts/sync_package_manifests.py --check` | exit 0 (v1.1.1; `Formula/` is out of scope) |
| `bash -n` on `linux/deb/build-deb.sh`, `linux/rpm/build-rpm.sh` | exit 0 |
| `git diff --check` | clean |
| `git grep -n '1\.1\.1' -- linux flatpak snap` | 2 lines, both history (spec `%changelog`, metainfo `<release>`) |
| metainfo XML parse | newest release 1.2.1, 2026-11-02 |
| `git ls-files --eol` | `.sh` `w/lf`; the other six `w/crlf` |
| `git diff --stat` | 8 files; 22 insertions, 15 deletions |
| 64-hex (SHA256-like) values on diff lines | 0 |

Not run: Rust checks (no Rust or `Cargo.*` change).

## Git Commit

- Commit Message: `fix(packaging): update version pins to 1.2.1 and repair the RPM helper [TASK-011]`
- Commit Hash:

## Final Report

- Pins in `linux/`, `flatpak/`, and `snap/` moved to 1.2.1, with a 1.2.1 spec `%changelog` entry and
  metainfo release dated 2026-11-02.
- `linux/rpm/build-rpm.sh` now resolves the repository root from its own location, archives the
  whole repository as `v<version>.tar.gz`, and finds the spec from any working directory.
- `docs/PACKAGING.md` section 3 updated; it does not claim a full build ran.
- Stages: Advisor, Dev, QA (Round 1 Accepted, two Lows accepted and left), and Docs ran in
  subagents; orchestration, final validation, and commits in the main session.
- Not verified: a real `rpmbuild` run, the helper on real Linux, `dpkg-deb`, `makepkg`, `snapcraft`,
  `flatpak-builder` (not on this host). Open items are under Documentation Updates.

## Notes and Limitations

- `rpmbuild`, `dpkg-deb`, `makepkg`, `snapcraft`, and `flatpak-builder` are not installed on this
  Windows host, so full package builds cannot be run here unless a stage states otherwise.
