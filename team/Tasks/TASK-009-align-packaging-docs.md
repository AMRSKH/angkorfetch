Author: AI Agent (Claude Code)
Date: 2026-11-11
Task: TASK-009
Status: Completed

# TASK-009: Align Packaging Docs With README

## Objective

Make `docs/PACKAGING.md` and `Formula/README.md` agree with the channel statuses and install
commands in `README.md` (as cleaned up in TASK-008), so no document presents a channel as
published or a command as working when it is not.

## Background

TASK-008 recorded, under its Dependencies section, where `docs/PACKAGING.md` and
`Formula/README.md` contradict the README. The maintainer approved fixing them as a separate
task (2026-11-11, "Yes, TASK-009").

The README follows the maintainer's latest explicit channel decision, commit `c40983a`
(2026-11-02): Homebrew is **Planned (homebrew-core)** and the `AMRSKH/tap` instructions were
removed from the primary install path; WinGet is **Pending** because the first submission was
rejected. That decision is the source of truth for statuses in this task.

Live facts checked by the main session on 2026-11-11 (read-only `gh api` and the crates.io API):

| Item | Fact |
|---|---|
| `AMRSKH/homebrew-tap` | Exists, not archived, last push 2026-07-30; `Formula/angkorfetch.rb` there still points to v1.1.1 |
| WinGet PR `microsoft/winget-pkgs#409790` ("New package: AMRSKH.AngkorFetch version 1.1.1") | `CLOSED`, not merged, 2026-08-20 |
| GitHub latest release | `v1.2.1` with 9 assets: 6 archives (incl. `angkorfetch-linux-x86_64-musl.tar.gz`), `angkorfetch_1.2.1_amd64.deb`, `angkorfetch-1.2.1-1.x86_64.rpm` (no dist suffix), `checksums.txt` |
| crates.io `angkorfetch` | Latest published version `1.1.1` (2026-07-30); 1.2.1 is not published there |

Contradictions TASK-008 listed (see `team/Tasks/TASK-008-readme-cleanup.md`, Dependencies):

1. Homebrew is "Published (Tap)" with `brew install AMRSKH/tap/angkorfetch` in
   `docs/PACKAGING.md` and `Formula/README.md`; README says Planned (homebrew-core). The
   overview row also pairs "Published (Tap)" with the untapped `brew install angkorfetch`.
2. WinGet says "Pending community PR approval at #409790"; the PR is closed unmerged, and
   README says rejected, resubmission pending. Overview uses the moniker `angkorfetch`;
   README uses the ID `AMRSKH.AngkorFetch`.
3. Arch overview row uses `sudo pacman -S`; the planned channel is the AUR (`yay`/`paru`).
4. crates.io is not listed, although README and AGENTS.md treat it as live.
5. Version-pinned examples use 1.1.1 while the current version is 1.2.1.
6. DEB/RPM upgrade commands (`apt upgrade`, `dnf upgrade angkorfetch`) cannot fetch a newer
   version while the packages exist only as release files; the Built rows show
   `sudo dnf install angkorfetch` / `sudo apt install angkorfetch`, which need a repository.

## Requirements

- Channel statuses in `docs/PACKAGING.md` and `Formula/README.md` match README's Package
  Status table. No status is changed in README.
- Commands shown as working are working today; Pending or Planned commands are clearly
  labelled as such (AGENTS.md section 11).
- The tap may be described as existing but no longer the documented install path, with its
  stale version stated; it must not be presented as the supported Homebrew channel.
- crates.io is listed as Live, with the fact that the published version lags the release.
- Version examples that describe the current release use 1.2.1; examples that are explicitly
  historical may keep their version if labelled.
- Maintainer procedures in `docs/PACKAGING.md` (how to submit to WinGet, homebrew-core, AUR,
  Flathub, Snap, Copr) stay intact unless they are factually wrong.

## Scope

- `docs/PACKAGING.md`
- `Formula/README.md` (hand-written; not produced by `scripts/sync_package_manifests.py`)

## Out of Scope

- `README.md` (unless the Docs stage finds a direct contradiction this task introduces).
- `Formula/angkorfetch.rb`, `winget-pkgs/`, `linux/`, `flatpak/`, `snap/`, and any SHA256
  value (AGENTS.md section 7).
- Publishing to crates.io, updating the tap, or resubmitting to WinGet (external actions).
- Stale artifact counts in `docs/RELEASING.md`, `docs/MAINTAINING.md`, and root `RELEASING.md`,
  and the 1.1.1 pins in `linux/arch/PKGBUILD` and `snap/snapcraft.yaml` (record only).

## Assigned Roles

- Advisor: `advisor` subagent (Claude Code).
- Dev: `dev` subagent (Claude Code).
- QA Review: `qa-review` subagent (Claude Code), independent of Dev.
- Docs: `docs` subagent (Claude Code).
- Orchestration, live fact checks, final validation, commits, and the final report: main session.

## Implementation Plan

Advisor, 2026-11-11. Repository facts were read on this branch (`claude/awesome-pascal-f6d9aa`,
HEAD `4daf6b9`). Live facts come from Background and were not re-fetched.

### Approach

| Option | Result | Verdict |
|---|---|---|
| A. Patch only the six TASK-008 contradictions (status cells, the named commands, versions) | Small diff, but steps that fail as written stay: `wingetcreate update` for a package that is not in `microsoft/winget-pkgs`, `bash linux/rpm/build-rpm.sh` from the root, an AUR push without `.SRCINFO`, path-based `brew audit`/`brew install`, and the claim that CI builds the `.deb`/`.rpm` from `linux/` | Rejected |
| **B. Section-by-section correction**: fix the six contradictions; correct each procedure step that is factually wrong (reason given per step); add a crates.io row and section; keep every other procedure and the current structure. In `Formula/README.md`, rewrite the top two sections and keep the last one | Both files agree with README and the repo; all maintainer procedures kept | **Recommended** |
| C. Drop the PACKAGING overview table and link to README for statuses | Smaller file, but removes maintainer detail; the requirement expects PACKAGING to state matching statuses | Rejected |

Rules for Dev:

- Edit only `docs/PACKAGING.md` and `Formula/README.md` (plus this task file). Both are
  `i/lf w/crlf`; keep CRLF in the working tree.
- Keep the headings, section numbers 1-7, bold `**Status:**` markers, and `---` separators.
  Add section 8 at the end; no renumbering (`git grep 'PACKAGING.md#'` is empty, so no anchor
  breaks).
- Copy status values verbatim from README's Package Status table (README lines 54-61).
- "Keep" means byte-identical. Do not reword anything this plan does not name.
- Do not name the current pin version (1.1.1) in text about `linux/`, `flatpak/`, or `snap/`
  files; describe the field instead, so the docs do not go stale when the pins are bumped.
  The only 1.1.1 mentions left are the dated live facts (tap, crates.io) and the PR #409790 title.

### docs/PACKAGING.md

**Title and intro (lines 1-3): keep. Add this paragraph after line 3 (blank line before and after):**

```markdown
Channel statuses here must match the [Package Status table in the README](../README.md#package-status--availability); change both together. **Live**: installable from the channel today. **Built**: the package file is attached to each GitHub Release, but no repository serves it. **Pending**: a submission was made and must be redone. **Planned**: not yet submitted to the target channel; the manifest, spec, or formula in this repository is the starting point.
```

**Packaging Overview & Status (lines 5-15): keep the heading and the header row; replace the
seven data rows with these eight (README order: Live, Built, Pending, Planned):**

```markdown
| **crates.io (Cargo)** | `angkorfetch` | `cargo install angkorfetch` | **Live** | crates.io; latest published version 1.1.1, behind the v1.2.1 release |
| **Fedora (DNF)** | `angkorfetch` | `sudo dnf install ./angkorfetch-*.x86_64.rpm` | **Built** | `.rpm` (x86_64) in GitHub Releases; Copr / Fedora repo planned |
| **Debian/Ubuntu (APT)** | `angkorfetch` | `sudo apt install ./angkorfetch_*_amd64.deb` | **Built** | `.deb` (amd64) in GitHub Releases; PPA / APT repo planned |
| **WinGet** | `AMRSKH.AngkorFetch` | `winget install AMRSKH.AngkorFetch` | **Pending** | `microsoft/winget-pkgs`: first PR #409790 closed unmerged; resubmission pending |
| **Homebrew** | `angkorfetch` | `brew install angkorfetch` | **Planned (homebrew-core)** | `Homebrew/homebrew-core` submission planned; `AMRSKH/homebrew-tap` is not the install path (formula there at v1.1.1) |
| **Arch Linux (AUR)** | `angkorfetch` | `yay -S angkorfetch` | **Planned** | PKGBUILD in `linux/arch/`; not yet on the AUR |
| **Flatpak** | `io.github.AMRSKH.angkorfetch` | `flatpak install flathub io.github.AMRSKH.angkorfetch` | **Planned** | Manifest and AppStream metainfo in `flatpak/`; Flathub submission pending |
| **Snap** | `angkorfetch` | `sudo snap install angkorfetch` | **Planned** | Manifest in `snap/`; Snap Store registration pending |
```

Directly under the table add:

```markdown
Only the **Live** and **Built** commands work today; the others are the target commands once the channel is published. Versions in this table were checked on 2026-11-11.
```

Fixes contradictions 1-4 and 6 (row level). Built rows use the README's local-file commands.

**Section 1, WinGet.**

- Metadata: keep every bullet except **Status** (Name, ID, Moniker, location, installer type,
  URL, and checksum match the three manifests: `InstallerType: zip`,
  `NestedInstallerType: portable`, `RelativeFilePath: angkorfetch.exe`, `Moniker: angkorfetch`).
  Replace the Status bullet with:

  ```markdown
  - **Status:** **Pending**. The first submission, [microsoft/winget-pkgs#409790](https://github.com/microsoft/winget-pkgs/pull/409790) ("New package: AMRSKH.AngkorFetch version 1.1.1"), was rejected over the manifest file names and closed without being merged (2026-08-20). The files were renamed to `AMRSKH.AngkorFetch.*.yaml` (commit `d079c68`); resubmission is pending, so the package is not in the WinGet community repository yet.
  ```

- Manifest Structure: keep.
- Validation & Testing: keep the heading; replace the code block with:

  ```powershell
  # Use the version directory that exists under winget-pkgs/manifests/a/AMRSKH/AngkorFetch/

  # Validate manifest schema
  winget validate --manifest winget-pkgs/manifests/a/AMRSKH/AngkorFetch/<version>

  # Test local manifest install (enable local manifests once, as administrator)
  winget settings --enable LocalManifestFiles
  winget install --manifest winget-pkgs/manifests/a/AMRSKH/AngkorFetch/<version>

  # Verification commands once the package is merged into microsoft/winget-pkgs:
  winget search angkorfetch
  winget show AMRSKH.AngkorFetch
  winget install AMRSKH.AngkorFetch
  winget upgrade AMRSKH.AngkorFetch
  winget uninstall AMRSKH.AngkorFetch
  ```

  Reasons: this branch has only the `1.1.1` directory; `1.2.1` exists only on the unmerged
  `origin/automation/sync-packages-v1.2.1` (`21ac68f`). A hard-coded `1.2.1` path fails today,
  and `1.1.1` goes stale when that branch is merged, so use the placeholder that the Metadata
  bullets already use. `LocalManifestFiles` is a winget admin setting that is `Disabled` by
  default (`winget --info`, winget v1.29.380 on this host); `install --manifest` refuses
  without it. The ID matches README.

- Automation & Publishing: keep the heading; replace the paragraph and its code block with:

  ````markdown
  `.github/workflows/sync-packages.yml` runs `scripts/sync_package_manifests.py` after each release. It renames the version directory and rewrites `PackageVersion`, `InstallerUrl`, and `InstallerSha256` on the branch `automation/sync-packages-v<version>` in this repository, for a pull request into `main`. It does not submit anything to `microsoft/winget-pkgs`. Merge that pull request before submitting, so the manifests match the release.

  Submitting to `microsoft/winget-pkgs` is a manual maintainer step. The package is not in that repository yet, so the resubmission is a new-package pull request made from the manifests in this repository:

  ```powershell
  winget validate --manifest winget-pkgs/manifests/a/AMRSKH/AngkorFetch/<version>
  wingetcreate submit winget-pkgs/manifests/a/AMRSKH/AngkorFetch/<version>
  ```

  After the package has been merged, later versions can use `wingetcreate update`, which starts from the manifest already in `microsoft/winget-pkgs`:

  ```powershell
  wingetcreate update AMRSKH.AngkorFetch --version <version> --urls https://github.com/AMRSKH/angkorfetch/releases/download/v<version>/angkorfetch-windows-x86_64.zip --submit
  ```

  `submit` and `update --submit` open the pull request on GitHub and need a GitHub token (`--token`, or one stored with `wingetcreate token`).
  ````

  Reason (factually wrong step): `wingetcreate update` loads the package's current manifest
  from `microsoft/winget-pkgs`. PR #409790 was closed unmerged, so there is nothing to update.
  The old command also pinned `v1.1.1` and had neither `--version` nor `--submit`.

**Section 2, Homebrew.**

- Metadata: keep `Package Name` and the four architecture lines. Replace the
  `Formula Path`, `Canonical Tap`, `Direct Tap URL`, and `Status` bullets, and relabel
  `Architectures`, so the list reads:

  ```markdown
  - **Package Name:** `angkorfetch`
  - **Formula Path:** `Formula/angkorfetch.rb` (generated; see [Formula/README.md](../Formula/README.md))
  - **Status:** **Planned (homebrew-core)**. The maintainer plans a build-from-source formula in `Homebrew/homebrew-core` (commit `c40983a`); it has not been submitted. `brew install angkorfetch` works only after it is accepted.
  - **Tap:** [`AMRSKH/homebrew-tap`](https://github.com/AMRSKH/homebrew-tap) still exists, but it is no longer the documented install path and CI does not update it. Its formula points to v1.1.1 (checked 2026-11-11), behind the v1.2.1 release.
  - **Architectures (generated formula):**
    - (the four existing lines, unchanged)
  ```

- Audit Compliance: keep the heading and the three bullets. Change the lead sentence
  "The formula is strictly audited against `brew audit --strict`:" to
  "The generated formula is written to pass `brew audit --strict`:". After the bullets add:

  ```markdown
  CI does not run `brew audit`. The `test` job in `release.yml` runs `python scripts/sync_package_manifests.py --check`, which fails if the committed formula is not what the generator produces. This formula installs the prebuilt release archives; the planned homebrew-core formula builds from source, so it will be a different formula.
  ```

  Reason: `release.yml` has no Homebrew step; the sync PR body only says "Run `brew audit --strict`
  on macOS if available". `c40983a` says "Planned for homebrew-core build from source".

- Validation & Testing: keep the heading; replace the code block with:

  ```bash
  # On macOS: put the formula in a local tap, then audit and install it by name
  brew tap-new local/angkorfetch
  cp Formula/angkorfetch.rb "$(brew --repository local/angkorfetch)/Formula/"
  brew audit --strict local/angkorfetch/angkorfetch
  brew install local/angkorfetch/angkorfetch

  # Verification commands once accepted into homebrew-core:
  brew search angkorfetch
  brew info angkorfetch
  brew install angkorfetch
  brew upgrade angkorfetch
  brew uninstall angkorfetch
  ```

  Reason: current Homebrew refuses `brew audit` with a file path and installs formulae only from
  a tap; `--build-from-source` does nothing for a formula without bottles that downloads prebuilt
  archives. This is from Homebrew's behaviour as the Advisor knows it, not run here (no macOS
  host). The local-tap form works whether or not path arguments are still accepted.

**Section 3, Fedora: DNF / RPM.**

- Metadata: keep `Package Name`, `Spec File`, `Build Helper`. Replace `Artifact` and `Status`:

  ```markdown
  - **Artifact:** `angkorfetch-<version>-1.x86_64.rpm` (for example `angkorfetch-1.2.1-1.x86_64.rpm`; no dist suffix, because the release job builds on Ubuntu, where `%{?dist}` is empty)
  - **Status:** **Built**. The `.rpm` is attached to each GitHub Release. No Copr or Fedora repository serves it yet, so `dnf install angkorfetch` by name does not work.
  ```

- Building the RPM: keep the heading; replace the code block and the sentence under it with:

  ````markdown
  The release `.rpm` does not use the files in `linux/rpm/`. On tag pushes, the `packages` job in `.github/workflows/release.yml` writes its own spec, packages the binary it just compiled with `rpmbuild`, and attaches the result to the release.

  `linux/rpm/angkorfetch.spec` and `linux/rpm/build-rpm.sh` build from source, for Copr or a Fedora review. They are not run in CI and need fixes before use:

  - `build-rpm.sh` copies `angkorfetch.spec` by a relative path, so it has to run from `linux/rpm/`:
    ```bash
    cd linux/rpm && bash build-rpm.sh
    ```
  - It writes the source archive as `angkorfetch-<version>.tar.gz`, but `rpmbuild` looks for the file name at the end of the spec's `Source0` URL (`v<version>.tar.gz`).
  - Both files set the version by hand (`VERSION`, `Version:`); keep them in step with `Cargo.toml` (see [RELEASING.md](../RELEASING.md#cutting-a-release), step 1).
  ````

  Reasons: `build-rpm.sh` line 12 `cp "$SPEC"` with `SPEC="angkorfetch.spec"` fails from the
  repository root under `set -e`; line 10 writes `angkorfetch-$VERSION.tar.gz`; spec line 8
  `Source0: %{url}/archive/v%{version}.tar.gz`. `release.yml` lines 257-279 use an inline spec.
  Not run (no `rpmbuild` on this host).

- Target Distribution Workflow: keep, with two edits:
  - Replace ``- Create Copr repository: `copr create amrskh/angkorfetch` `` with
    ``- Create the Copr project `amrskh/angkorfetch` (Copr web UI or `copr-cli create`).``
    Reason: the Copr client command is `copr-cli`; there is no `copr create`.
  - `- User command:` becomes `- User command (once the Copr project exists):`.
- Testing Commands: keep the heading; replace the code block with:

  ```bash
  # Test local package install (file from the GitHub Release)
  sudo dnf install ./angkorfetch-1.2.1-1.x86_64.rpm

  # Upgrade: download the newer .rpm and install it the same way; dnf upgrades the installed package.
  # `dnf upgrade angkorfetch` needs a repository (Copr or Fedora), which does not exist yet.

  # Removal
  sudo dnf remove angkorfetch
  ```

**Section 4, Ubuntu / Debian: APT / DEB.**

- Metadata: keep every bullet except Status:

  ```markdown
  - **Status:** **Built**. The `.deb` is attached to each GitHub Release. No PPA or APT repository serves it yet, so `apt install angkorfetch` by name does not work.
  ```

- Building the Debian Package: keep the heading; replace the code block and sentence with:

  ````markdown
  Run from the repository root (the helper runs `cargo build` and copies `README.md` by relative path):

  ```bash
  bash linux/deb/build-deb.sh
  ```

  The helper sets `VERSION` by hand, so check it matches `Cargo.toml` before building. CI does not use it: on tag pushes, the `packages` job in `.github/workflows/release.yml` builds the release `.deb` with its own inline `control` file (`Section`, `Priority`, `Architecture: amd64`, `Maintainer`, `Description`) and the tag version.
  ````

  Reason: `build-deb.sh` line 4 hard-codes `VERSION`; `release.yml` lines 237-255 build the
  release `.deb` inline.

- Target Distribution Workflow: keep, except `- User command:` becomes
  `- User command (once the repository exists):`.
- Testing Commands: keep the heading; replace the code block with:

  ```bash
  # Test local package install (file from the GitHub Release)
  sudo apt install ./angkorfetch_1.2.1_amd64.deb

  # Upgrade: download the newer .deb and install it the same way; apt upgrades the installed package.
  # `apt upgrade` needs an APT repository, which does not exist yet.

  # Removal
  sudo apt remove angkorfetch
  ```

**Section 5, Arch Linux: Pacman / AUR.**

- Metadata: keep, except the Status bullet becomes
  ``- **Status:** **Planned** (PKGBUILD in `linux/arch/`; not yet on the AUR)`` (README wording;
  "ready" is not accurate while `sha256sums=('SKIP')` and `pkgver` lag the release).
- Building with makepkg: keep the block. Add after it:
  ``The PKGBUILD downloads the GitHub archive for `pkgver`, not the working tree.``
- Target Distribution Workflow:
  1. AUR: replace ``- Push PKGBUILD to `ssh://aur@aur.archlinux.org/angkorfetch.git`.`` with:

     ```markdown
     - Set `pkgver`, replace `sha256sums=('SKIP')` with the archive's checksum (`updpkgsums`, from `pacman-contrib`), generate `.SRCINFO` with `makepkg --printsrcinfo > .SRCINFO`, and push both files to `ssh://aur@aur.archlinux.org/angkorfetch.git`. The AUR rejects a push without `.SRCINFO`.
     ```

     `- Users install via AUR helpers:` becomes `- Users install via AUR helpers (once on the AUR):`;
     keep the `yay`/`paru` block. Reason (factually wrong step): the AUR's server hook requires
     `.SRCINFO`; `docs/MAINTAINING.md` (section 4, Arch Linux) already has the `.SRCINFO` and
     SHA256 steps.
  2. Official Arch Repositories: keep, except `- Native command:` becomes
     `- Native command (once in the official repositories):`.
- Testing Commands: keep the heading; replace the code block with:

  ```bash
  # Inspect the PKGBUILD and the built package with namcap
  namcap linux/arch/PKGBUILD
  namcap linux/arch/angkorfetch-[0-9]*.pkg.tar.zst

  # Install the local build, then remove it
  sudo pacman -U linux/arch/angkorfetch-[0-9]*.pkg.tar.zst
  sudo pacman -R angkorfetch

  # Upgrade once the package is on the AUR
  yay -Syu
  ```

  Reason: `pacman -Syu angkorfetch` upgrades only from configured repositories; an AUR or
  locally built package is upgraded by an AUR helper or `pacman -U`. `[0-9]` keeps an
  `angkorfetch-debug-*` package (built when `debug` is on in `makepkg.conf`) out of the glob;
  `.pkg.tar.zst` is makepkg's default `PKGEXT`.

**Section 6, Universal Linux: Flatpak / Flathub.**

- Metadata: keep every bullet except Status:
  ``- **Status:** **Planned** (manifest and AppStream metainfo in `flatpak/`; Flathub submission pending)``.
- Building Flatpak Locally: keep the block. Add after it:

  ```markdown
  Not run in CI and not verified. The manifest declares no Rust SDK extension (`sdk-extensions`) and no vendored Cargo sources, and it builds the tag in its `sources` entry from GitHub, not the working tree.
  ```

- Flathub Publishing: keep steps 1 and 3. Step 2 becomes:
  ``2. Open a pull request against the `new-pr` branch adding `io.github.AMRSKH.angkorfetch` (manifest and metainfo).``
  Reason: Flathub takes new-app submissions only against `new-pr`.
- Testing Commands: keep.

**Section 7, Universal Linux: Snap.** Keep everything. Add after the build block:
`` `snapcraft.yaml` builds the tag in `source-tag` from GitHub, not the working tree. ``

**Section 8 (new), crates.io.** Append after section 7, preceded by a `---` separator:

```markdown
## 8. Any OS: crates.io

### Metadata
- **Crate:** [`angkorfetch`](https://crates.io/crates/angkorfetch)
- **Install Command:** `cargo install angkorfetch` (builds from source, installs to `~/.cargo/bin`)
- **Status:** **Live**. The latest published version is 1.1.1 (2026-07-30), behind the v1.2.1 GitHub release (checked 2026-11-11).

### Publishing
No workflow publishes the crate: `release.yml` has no `cargo publish` step. Publishing is a manual step from the tagged commit and needs maintainer approval (see [AGENTS.md](../AGENTS.md), section 7).
```

Fixes contradiction 4. `grep -n 'cargo publish' .github/workflows/*.yml` is empty.

### Formula/README.md

Confirmed hand-written: `scripts/sync_package_manifests.py` references only
`HOMEBREW_FORMULA = Path("Formula/angkorfetch.rb")`; `update_homebrew` and `check_homebrew` read
and write only that path; `test_sync_package_manifests.py` uses only `HOMEBREW_FORMULA`;
`sync-packages.yml` commits only what the script changed.

Replace lines 1-22 (title through the `--check` code block) with the text below. Keep
`## Why this directory is named Formula/` and its paragraph (lines 24-31) unchanged; its link
`../RELEASING.md#why-the-formula-lives-in-formula` matches root `RELEASING.md` line 381.

````markdown
# Homebrew formula for AngkorFetch

Homebrew is **Planned (homebrew-core)** (see [Package Status](../README.md#package-status--availability)),
so there is no supported `brew install` command yet. To install today, use a quick-install script
or `cargo install angkorfetch` ([Installation](../README.md#installation)).

## What this formula is

`angkorfetch.rb` installs the prebuilt release archives for macOS and Linux (arm and intel).
After each release, `.github/workflows/sync-packages.yml` regenerates it on the
`automation/sync-packages-v<version>` branch, for a pull request into `main`.

The [AMRSKH/homebrew-tap](https://github.com/AMRSKH/homebrew-tap) repository still exists and
holds a copy of this formula, but it is no longer the documented install path and CI does not
update it. Its formula points to v1.1.1 (checked 2026-11-11), behind the v1.2.1 release.

The planned homebrew-core formula builds from source (commit `c40983a`), so it will not be this file.

This README is hand-written; `scripts/sync_package_manifests.py` neither generates nor checks it.

## Do not edit angkorfetch.rb by hand

`angkorfetch.rb` is generated, and the next release's sync pull request overwrites it. Change the
template in `render_homebrew_formula()` in
[`scripts/sync_package_manifests.py`](../scripts/sync_package_manifests.py) and
update `scripts/testdata/angkorfetch.rb.golden`. CI fails otherwise:

```bash
python scripts/sync_package_manifests.py --check
```
````

Removed on purpose: the `brew install AMRSKH/tap/angkorfetch` block (contradiction 1); "This is a
mirror, not the source of truth" and "kept **byte-identical** to the tap's" (the tap is no longer
the documented channel, and byte identity cannot be checked offline); "so this repository can also
be tapped directly" (reads as an install path); "Both are generated" (CI does not touch the tap).

### Further contradictions found (record only; out of scope for this task)

| # | Where | Contradiction | Source |
|---|---|---|---|
| 1 | `Formula/angkorfetch.rb`, `winget-pkgs/manifests/a/AMRSKH/AngkorFetch/1.1.1/` on `main` (`ebf5823`) and this branch | Still pin v1.1.1. The v1.2.1 update sits on `origin/automation/sync-packages-v1.2.1` (`21ac68f`), not merged. A WinGet resubmission or tap update from `main` today would ship 1.1.1 | `git merge-base --is-ancestor` (false); `git diff --stat HEAD origin/automation/sync-packages-v1.2.1` |
| 2 | Pins beyond the PKGBUILD and snapcraft.yaml already listed | `linux/deb/build-deb.sh` and `linux/rpm/build-rpm.sh` `VERSION="1.1.1"`; `linux/rpm/angkorfetch.spec` `Version: 1.1.1` (no 1.2.1 changelog entry); `flatpak/io.github.AMRSKH.angkorfetch.yml` `tag: v1.1.1`; metainfo newest `<release version="1.1.1">`. Root `RELEASING.md`, Cutting a release, step 1 lists all of them; v1.2.1 skipped that step | The files themselves |
| 3 | `linux/rpm/build-rpm.sh` | Fails as documented: relative `cp angkorfetch.spec` (root CWD), and `SOURCES/angkorfetch-<v>.tar.gz` versus the spec's `Source0` file name `v<v>.tar.gz`. Read only, not run | `build-rpm.sh:10,12`, `angkorfetch.spec:8` |
| 4 | `.github/workflows/release.yml` versus `linux/deb/`, `linux/rpm/` | Release `.deb`/`.rpm` come from an inline control file and spec; the `linux/` helpers are unused by CI (old PACKAGING text implied otherwise; fixed by this plan) | `release.yml:237-279` |
| 5 | `flatpak/io.github.AMRSKH.angkorfetch.yml` | `sdk: org.freedesktop.Sdk` with no Rust SDK extension and no vendored Cargo sources; Flathub builds run offline, so the build most likely needs both. Not verified (no `flatpak-builder`). I am also not sure whether Flathub's inclusion policy accepts a console-only app (metainfo `type="console-application"`); worth checking before a submission | Manifest |
| 6 | `linux/arch/PKGBUILD` | `sha256sums=('SKIP')` for a release tarball; the AUR expects a real checksum (`docs/MAINTAINING.md` already says to set it) | `PKGBUILD:12` |
| 7 | `docs/MAINTAINING.md:120-123`, root `RELEASING.md:149` | WinGet submission via `wingetcreate update`, which needs the package to already be in `microsoft/winget-pkgs` (it is not). `docs/MAINTAINING.md:133` uses `copr-cli build-package ... --clone-url`; as far as I know `--clone-url` belongs to `copr-cli buildscm` / `add-package-scm` (not verified) | Those lines |
| 8 | Root `RELEASING.md:394-396`, `:107`, `:150`; `docs/MAINTAINING.md:127` | Call the tap "the user-facing one" and "canonical" and keep "update the tap" as a release step; contradicts README's Planned (homebrew-core) status and the tap's v1.1.1 state | Those lines |
| 9 | `docs/RELEASING.md:41` | Says the script "enforces `brew audit --strict` standards"; it enforces the generator shape, and CI never runs `brew audit` | `release.yml` (no brew step) |
| 10 | Homebrew channel | The repo has no build-from-source formula yet; the generated formula is binary-only, which homebrew-core does not accept as far as I know. The maintainer's plan (`c40983a`) already says build from source, so this is a to-do, not a conflict | `Formula/angkorfetch.rb`, `c40983a` |

### Files to change

- `docs/PACKAGING.md`: intro paragraph; overview table and the note under it; sections 1-7 as
  listed; new section 8.
- `Formula/README.md`: lines 1-22 replaced; lines 24-31 kept.
- No code, scripts, workflows, manifests, `Formula/angkorfetch.rb`, or SHA256 values.

### Risks and mitigations

| Risk | Mitigation |
|---|---|
| Live facts go stale (tap version, crates.io version, PR state) | Each is dated "(checked 2026-11-11)" in the docs and kept in Background |
| Commands this host cannot run (`brew tap-new`/`audit`, `wingetcreate submit`/`update`, `copr-cli`, `makepkg`/`namcap`, `flatpak-builder`, `snapcraft`) have drifted syntax | Each is a maintainer step on the matching OS; the `winget` commands were checked locally (v1.29.380: `validate --manifest`, `install --manifest`, `settings --enable`, `LocalManifestFiles`). QA records the rest as not run and may check them against each tool's docs |
| The correction turns into a rewrite | Only steps with a stated reason change; everything marked "keep" stays byte-identical |
| PACKAGING drifts from README again | The new intro paragraph says to change both together |
| `<version>` placeholders error if pasted literally (PowerShell treats `<` as reserved, bash as a redirect) | Intended; the first comment line says which directory to use |
| Staging `Formula/README.md` triggers the pre-commit hook (`^(scripts\|Formula\|winget-pkgs)/`) | Expected: script tests and `--check` run and pass, since neither scripts nor the formula change. Never `--no-verify` |
| AGENTS.md calls `Formula/` generated | Only `Formula/README.md` changes, and it now says it is hand-written |
| Out-of-scope docs (`docs/MAINTAINING.md`, root `RELEASING.md`, `docs/RELEASING.md`) still call the tap canonical and use `wingetcreate update` | Recorded above for a follow-up task; the Docs stage records them and does not fix them here |
| The Homebrew path-argument claim comes from memory | The local-tap replacement works either way |

### Test plan

1. `python scripts/sync_package_manifests.py --check`: prints
   `Formula/angkorfetch.rb matches the generated shape (v1.1.1)`, exit 0.
2. `python -m unittest discover -s scripts -p "test_*.py"`: OK.
3. Links: extract every `](...)` target from both files; for targets that are not `http` or a
   bare `#`, strip the anchor, resolve relative to the file's folder, then `test -e`. Expected
   targets: from `docs/` `../README.md`, `../Formula/README.md`, `../RELEASING.md`,
   `../AGENTS.md`; from `Formula/` `../README.md`, `../scripts/sync_package_manifests.py`,
   `../RELEASING.md`. Anchors: `#package-status--availability` = README
   `## Package Status & Availability` (line 50); `#installation` = `## Installation` (line 20);
   `#cutting-a-release` = root `RELEASING.md` `## Cutting a release` (line 112);
   `#why-the-formula-lives-in-formula` = `### Why the formula lives in Formula/` (line 381).
4. Status match: each overview row and each section's `**Status:**` bullet equals the README row
   for the same channel (8 channels: Live, Built, Built, Pending, Planned (homebrew-core),
   Planned, Planned, Planned).
5. Grep:
   `grep -nE 'Published|AMRSKH/tap/|409790|pacman -S|1\.1\.1|install angkorfetch|wingetcreate update|copr create|apt upgrade|dnf upgrade|pacman -Syu angkorfetch|brew audit --strict Formula|--build-from-source|\bready\b' docs/PACKAGING.md Formula/README.md`
   (`\bready\b` so "already" does not match; `--build-from-source` so the "build-from-source
   formula" wording in the Homebrew Status bullet does not match).
   Allowed hits only:
   - `409790`: overview WinGet row, WinGet Status bullet.
   - `1.1.1`: overview crates.io and Homebrew rows, Homebrew Tap bullet, section 8 Status, the PR
     title in the WinGet Status bullet, the tap sentence in `Formula/README.md`.
   - `install angkorfetch`: `cargo install angkorfetch` (Live); `brew install angkorfetch`
     (overview Planned row, Homebrew Status, the "once accepted" block); `sudo snap install
     angkorfetch` (Planned row); `dnf install angkorfetch` / `apt install angkorfetch` in the
     Built Status bullets ("by name does not work") and inside the labelled Copr, Fedora, and PPA
     target steps.
   - `pacman -S`: only the Official Arch Repositories target block (labelled).
   - `wingetcreate update`: only the "After the package has been merged" text and block.
   - `apt upgrade`, `dnf upgrade`: only the comments that say a repository is needed.
   - None at all for `Published`, `AMRSKH/tap/`, `copr create`, `pacman -Syu angkorfetch`,
     `brew audit --strict Formula`, `--build-from-source`, `ready` as a word.
6. Repository facts behind the new text:
   - `grep -nE 'Moniker|InstallerType|RelativeFilePath' winget-pkgs/manifests/a/AMRSKH/AngkorFetch/*/*.yaml`
   - `grep -n 'sdk' flatpak/io.github.AMRSKH.angkorfetch.yml` (no `sdk-extensions`)
   - `grep -nE 'base|confinement|source-tag' snap/snapcraft.yaml`
   - `grep -nE '^VERSION=|cp "\$SPEC"' linux/deb/build-deb.sh linux/rpm/build-rpm.sh`
   - `grep -n Source0 linux/rpm/angkorfetch.spec`
   - `grep -n 'cargo publish' .github/workflows/*.yml` (empty)
   - `grep -n brew .github/workflows/release.yml` (comments only, no step)
   - `release.yml` lines 237-279 (inline control and spec)
7. Optional on this Windows host: `winget validate --manifest winget-pkgs\manifests\a\AMRSKH\AngkorFetch\1.1.1`
   succeeds (as recorded in `d079c68`). Do not run `winget settings --enable` (changes a machine
   admin setting) or `winget install --manifest` (installs software).
8. `git diff --stat`: only `docs/PACKAGING.md` and `Formula/README.md`; `git status --short` also
   shows this task file; `git diff --check` clean; `git ls-files --eol` still `w/crlf`.
9. Not runnable here; record as not verified: `brew`, `wingetcreate`, `copr-cli`, `rpmbuild`,
   `makepkg`/`namcap`, `flatpak-builder`, `snapcraft`.
10. Rust checks are not required (no Rust or `Cargo.*` change). Running them is harmless.

### Approvals required

None for the edits or the local work commit. A push or PR needs maintainer approval. The follow-up
actions this plan points to (merge `automation/sync-packages-v1.2.1`, publish 1.2.1 to crates.io,
update or retire the tap, resubmit to WinGet, Copr/AUR/Flathub/Snap submissions, fix the `linux/`
helpers and pins, align `docs/MAINTAINING.md` and the `RELEASING.md` files) are outside this task
and each needs maintainer approval.

### Implementation (Dev, 2026-11-11)

Ran in the `dev` subagent (Claude Code). Option B applied exactly as written above, to
`docs/PACKAGING.md` and `Formula/README.md` only. Before editing, every repository fact the new
text relies on was re-read: the three WinGet manifests, `linux/deb/build-deb.sh`,
`linux/rpm/build-rpm.sh`, `linux/rpm/angkorfetch.spec`, `linux/arch/PKGBUILD`, the Flatpak
manifest and metainfo, `snap/snapcraft.yaml`, `Formula/angkorfetch.rb` (hash lines not read into
the docs), `release.yml` (`test`, `packages`, `release` jobs), `sync-packages.yml` (branch name,
`gh pr create --base main`), `update_winget` / `_winget_dir` in the sync script, commits `c40983a`
and `d079c68`, README lines 20 and 50-61, root `RELEASING.md` lines 112 and 381.

`docs/PACKAGING.md` (325 lines, was 269; `wc -l`, corrected after QA Round 1):

| Plan item | Done |
|---|---|
| Title, intro | Kept; status-definition paragraph added after line 3 |
| Overview | Heading and header row kept; seven rows replaced by the plan's eight (README order); note added under the table |
| 1. WinGet | Status bullet replaced; Validation block replaced (`<version>`, `LocalManifestFiles`, ID-based verification); Automation & Publishing paragraph and block replaced (sync PR, `wingetcreate submit`, then `wingetcreate update --version --urls --submit`, token note). Other metadata bullets and Manifest Structure kept |
| 2. Homebrew | Formula Path / Canonical Tap / Direct Tap URL / Status replaced by Formula Path, Status, Tap; `Architectures` relabelled, its four lines kept; Audit lead sentence changed, three bullets kept, CI paragraph added; Validation block replaced (local tap) |
| 3. Fedora | Artifact and Status replaced; Building the RPM block and sentence replaced; `copr create` line and `User command:` changed; Testing block replaced (1.2.1, upgrade comment, removal). Official Fedora Submission kept |
| 4. APT / DEB | Status replaced; Building block and sentence replaced; `User command:` changed; Testing block replaced (1.2.1). Debian Official Archive kept |
| 5. Arch | Status replaced; sentence added after the makepkg block; AUR push bullet replaced; both `User command` / `Native command` labels changed; Testing block replaced |
| 6. Flatpak | Status replaced; sentence added after the build block; Flathub step 2 replaced. Steps 1 and 3 and Testing Commands kept |
| 7. Snap | Kept; one sentence added after the build block |
| 8. crates.io | New section after a `---` separator |

`Formula/README.md` (39 lines, was 31): lines 1-22 replaced with the plan's text; `## Why this
directory is named Formula/` and its paragraph kept byte-identical.

Headings, section numbers 1-7, `**Status:**` markers, and `---` separators are unchanged; section 8
is appended. Both files keep CRLF in the working tree (325 and 39 CRLF, no bare LF, no non-ASCII
byte); the task file stays LF. No SHA256 value, manifest, formula, script, or workflow touched.

Deviations from the plan: none. Concern noted, not changed (minor wording; QA to judge):

- Section 3, Building the RPM, plan text "the `packages` job ... packages the binary it just
  compiled with `rpmbuild`, and attaches the result to the release". The `packages` job does
  compile (`cargo build --release`, `release.yml` line 215) and run `rpmbuild` (line 278), but it
  only uploads a workflow artifact (`actions/upload-artifact`, name `packages`, lines 281-286);
  the `release` job downloads it and attaches it (`download-artifact`, then
  `softprops/action-gh-release` with `files: artifacts/*`, lines 318-354). The end result
  (`.rpm` on the release) is accurate; the sentence attributes the attach step to the wrong job.
  If QA wants it exact, a minimal fix is "...with `rpmbuild`; the `release` job attaches the
  result to the release." The `.deb` sentence in section 4 does not make this claim.

Files changed: `docs/PACKAGING.md`, `Formula/README.md` (131 insertions, 67 deletions). This task
file.

### Dev fixes (Round 1 Lows, 2026-11-11, dev subagent)

Ran in the `dev` subagent (Claude Code). Fixed QA Round 1 L1-L5 and the `wingetcreate token`
wording item, in `docs/PACKAGING.md` only. L6 is out of scope (root `RELEASING.md`) and was not
touched. `Formula/README.md` is unchanged in this round. Six lines were edited in place; none were
added or removed (still 325 lines).

| Item | Line | Old | New | Source checked |
|---|---|---|---|---|
| L1 | 129 | "...packages the binary it just compiled with `rpmbuild`, and attaches the result to the release." | "...packages the binary it just compiled with `rpmbuild`; the `release` job attaches the result to the release." | `release.yml:281-286` upload-artifact `packages`; `:318` download-artifact, `:352-354` `softprops/action-gh-release`, `files: artifacts/*` |
| L2 | 125 | "no dist suffix, because the release job builds on Ubuntu" | "no dist suffix, because the `packages` job builds on Ubuntu" | `release.yml:199-202` (`packages`, `runs-on: ubuntu-latest`), `:278` `rpmbuild` |
| L3 | 180 | "(the helper runs `cargo build` and copies `README.md` by relative path)" | "(the helper copies `target/release/angkorfetch` and `README.md` by relative path)" | `build-deb.sh:5` `BIN="angkorfetch"`, `:28` `cp "target/release/$BIN" ...`, `:31` `cp README.md ...`. `cargo build` (`:27`) finds `Cargo.toml` from a subdirectory, but its output lands in the root `target/`, so the relative copy at `:28` is what needs the root |
| L4 | 133 | "...so it has to run from `linux/rpm/`:" | "...so it has to run from `linux/rpm/` (even there, the build fails until the next item is fixed):" | `build-rpm.sh:10` writes `angkorfetch-$VERSION.tar.gz`; `angkorfetch.spec:8` `Source0: %{url}/archive/v%{version}.tar.gz`. The clause sits before the code block, so it is read before the command |
| L5 | 5 | "**Planned**: ...; the manifest, spec, or formula in this repository is the starting point." | "**Planned**: ...; the manifest or spec in this repository is the starting point (Homebrew needs a new build-from-source formula; see section 2)." | PACKAGING `:99`, `Formula/README.md:17`, `c40983a`. QA's wording plus the coordinator's "see section 2" pointer. Consistent with README `:58` (Planned (homebrew-core), direct submission to homebrew-core planned); README not changed |
| Wording | 76 | "or one stored with `wingetcreate token`" | "or one stored with `wingetcreate token --store`" | QA Round 1, Not verified (store flag is `--store`). Not run here: `wingetcreate` is not installed |

Also corrected in this task file: the old line counts in the Implementation entry (269 and 31 by
`wc -l` on `git show HEAD:<file>`, not 270 and 32), per the QA observation.

Re-run results (2026-11-11, same host):

| # | Check | Result |
|---|---|---|
| 1 | `python scripts/sync_package_manifests.py --check` | `Formula/angkorfetch.rb matches the generated shape (v1.1.1)`, exit 0. Pass |
| 2 | `python -m unittest discover -s scripts -p "test_*.py"` | `Ran 35 tests`, `OK`. Pass |
| 3 | Link and anchor check (same scratchpad script) | 0 failures; same 8 relative targets and 4 anchors. Pass |
| 4 | Status match (same scratchpad script) | All 8 channels match README (status, ID, command); `Formula/README.md` has `**Planned (homebrew-core)**`. Pass |
| 5 | Plan grep on both files | 25 hit lines, the same file:line set as in Dev results (PACKAGING 11, 14, 15, 18, 34, 70, 73, 85, 86, 112, 114, 126, 147, 153, 161, 162, 177, 198, 208, 209, 246, 321, 322; Formula/README 5, 15), each allowed as classified there. None of the six edited lines is a hit; "build-from-source formula" on line 5 does not match `--build-from-source`. Pass |
| 6 | `git diff --check` | No output, exit 0. Pass |
| 7 | `git ls-files --eol docs/PACKAGING.md Formula/README.md` | Both `i/lf w/crlf`; byte scan 325 and 39 CRLF, 0 bare LF, 0 non-ASCII, final CRLF. Pass |
| 8 | `git diff --stat`; `git status --short` | `Formula/README.md \| 28`, `docs/PACKAGING.md \| 170`, 2 files, 131 insertions, 67 deletions (unchanged, because the six edits replace lines already in the diff). Status: `M Formula/README.md`, `M docs/PACKAGING.md`, `?? team/Tasks/TASK-009-align-packaging-docs.md`. Nothing staged. Pass |

Not verified: `wingetcreate token --store` and the `build-rpm.sh` / `build-deb.sh` behaviour
(tools not on this host; read from the scripts only).

### Dev fixes (Round 2 Lows, 2026-11-11, dev subagent)

Ran in the `dev` subagent (Claude Code). Fixed QA Round 2 N1 and N2 in `docs/PACKAGING.md` only;
`Formula/README.md` is unchanged. Two lines were edited in place (still 325 lines).

| Item | Line | Old | New |
|---|---|---|---|
| N1 | 133 | "...so it has to run from `linux/rpm/` (even there, the build fails until the next item is fixed):" | "...so it has to run from `linux/rpm/` (even there it fails: from `linux/rpm/`, `git archive ... HEAD` packs only that directory, so the archive has no `Cargo.toml`; see also the next item):" |
| N2 | 129 | "...writes its own spec, packages the binary it just compiled with `rpmbuild`; the `release` job attaches the result to the release." | "...writes its own spec and packages the binary it just compiled with `rpmbuild`; the `release` job attaches the result to the release." |

N1 evidence, checked by Dev with read-only commands (nothing written to disk):

- `git -C linux/rpm archive --format=tar --prefix=x/ HEAD | tar -tf -` lists only `x/`,
  `x/angkorfetch.spec`, `x/build-rpm.sh`.
- The same command from the repository root includes `x/Cargo.toml` and `x/src/main.rs`.
- `build-rpm.sh:10` runs `git archive ... HEAD` with no path argument, so from `linux/rpm/` it
  packs only that subdirectory; `angkorfetch.spec:20` `%setup -q` and `:23` `cargo build --release
  --locked` then have no `Cargo.toml`. For the follow-up task on `linux/rpm/`: the helper fails
  from every directory as written (from the root at `cp "$SPEC"`, `:12`; from `linux/rpm/` because
  of the subtree-only archive plus the `Source0` name mismatch).

Re-run results (2026-11-11, same host):

| # | Check | Result |
|---|---|---|
| 1 | `python scripts/sync_package_manifests.py --check` | `Formula/angkorfetch.rb matches the generated shape (v1.1.1)`, exit 0. Pass |
| 2 | `python -m unittest discover -s scripts -p "test_*.py"` | `Ran 35 tests`, `OK`. Pass |
| 3 | Link and anchor check (same scratchpad script) | 0 failures. Pass |
| 4 | Status match (same scratchpad script) | 0 failures; all 8 channels match README. Pass |
| 5 | Plan grep on both files | 25 hit lines, the same file:line set as before (PACKAGING 11, 14, 15, 18, 34, 70, 73, 85, 86, 112, 114, 126, 147, 153, 161, 162, 177, 198, 208, 209, 246, 321, 322; Formula/README 5, 15), all allowed as classified in Dev results. Neither edited line is a hit. Pass |
| 6 | `git diff --check` | No output, exit 0. Pass |
| 7 | `git ls-files --eol docs/PACKAGING.md Formula/README.md` | Both `i/lf w/crlf`; byte scan 325 and 39 CRLF, 0 bare LF, 0 non-ASCII, final CRLF. Pass |
| 8 | `git diff --stat`; `git status --short`; `git diff --cached --stat` | `Formula/README.md \| 28`, `docs/PACKAGING.md \| 170`, 2 files, 131 insertions, 67 deletions. Status: `M Formula/README.md`, `M docs/PACKAGING.md`, `?? team/Tasks/TASK-009-align-packaging-docs.md`. Nothing staged. Pass |

Not verified: an actual `rpmbuild` run (not installed on this host); the N1 conclusion rests on the
`git archive` listing above and the spec.

## Dependencies

- TASK-008 (README statuses and the contradiction list).
- Sources of truth: README Package Status table (lines 52-61); maintainer commits `c40983a`
  (Homebrew Planned for homebrew-core, build from source; WinGet rejected over manifest naming)
  and `d079c68` (manifest files renamed to `AMRSKH.AngkorFetch.*.yaml`, `winget validate`
  succeeded); live facts in Background (not re-fetched).
- Repository files read: `docs/PACKAGING.md`, `Formula/README.md`, `Formula/angkorfetch.rb`,
  `winget-pkgs/manifests/a/AMRSKH/AngkorFetch/1.1.1/*.yaml`, `linux/arch/PKGBUILD`,
  `linux/arch/build-arch.sh`, `linux/deb/build-deb.sh`, `linux/rpm/angkorfetch.spec`,
  `linux/rpm/build-rpm.sh`, `flatpak/*.yml`, `flatpak/*.metainfo.xml`, `snap/snapcraft.yaml`,
  `.github/workflows/release.yml`, `.github/workflows/sync-packages.yml`,
  `scripts/sync_package_manifests.py`, `scripts/test_sync_package_manifests.py`,
  `.githooks/pre-commit`, `docs/MAINTAINING.md`, `docs/RELEASING.md`, root `RELEASING.md`.
- Git state: `origin/automation/sync-packages-v1.2.1` (`21ac68f`) is not merged into `main`
  (`ebf5823`) or this branch, so `Formula/` and `winget-pkgs/` stay at 1.1.1 here. The docs plan
  does not depend on it, because version-specific manifest paths use `<version>`.
- Local tools: `winget` v1.29.380 present (commands above checked with `--help` and `--info`).
  Not installed: `wingetcreate`, `brew`, `copr-cli`, `rpmbuild`, `makepkg`, `flatpak-builder`,
  `snapcraft`.
- No maintainer decision outstanding for this task.

## Acceptance Criteria

- [x] Every channel status in both files matches README's Package Status table
- [x] Homebrew: homebrew-core shown as Planned; tap described accurately and not as the supported channel
- [x] WinGet: PR #409790 shown as closed unmerged; resubmission pending; ID `AMRSKH.AngkorFetch`
- [x] Arch shown as AUR (Planned); crates.io listed as Live with its version lag
- [x] Built DEB/RPM rows use local-file install commands; no repository-only command shown as working
- [x] Current-version examples use 1.2.1
- [x] Maintainer submission procedures still present and correct (commands for tools not installed on
  this host were checked for syntax only; see QA Rounds 1-3, "Not verified")
- [x] `python scripts/sync_package_manifests.py --check` and script tests still pass
- [x] All relative links resolve
- [x] Independent QA accepted (Round 3, `qa-review` subagent)

## Testing Requirements

- `python scripts/sync_package_manifests.py --check`
- `python -m unittest discover -s scripts -p "test_*.py"`
- Every relative link in both files points to an existing file.
- Grep both files for `Published`, `AMRSKH/tap`, `409790`, `pacman -S`, `1.1.1`,
  `install angkorfetch` and confirm each remaining hit is accurate and labelled.
- `git diff --stat` shows only the files in scope.

### Dev results (2026-11-11, `dev` subagent, Windows 11, Git Bash + PowerShell 5.1, Python 3.14.10)

Plan test items 1-10. The link and status checks used throwaway Python scripts in the session
scratchpad (not in the repository).

| # | Check | Command | Result |
|---|---|---|---|
| 1 | Formula shape | `python scripts/sync_package_manifests.py --check` | `Formula/angkorfetch.rb matches the generated shape (v1.1.1)`, exit 0. Pass |
| 2 | Script tests | `python -m unittest discover -s scripts -p "test_*.py"` | `Ran 35 tests`, `OK`, exit 0. Pass |
| 3 | Links and anchors | Extract every `](...)` target from both files; resolve non-`http` targets relative to the file's folder; compute GitHub heading slugs for anchors | `docs/PACKAGING.md`: `../README.md#package-status--availability`, `../Formula/README.md`, `../RELEASING.md#cutting-a-release`, `../AGENTS.md`. `Formula/README.md`: `../README.md#package-status--availability`, `../README.md#installation`, `../scripts/sync_package_manifests.py`, `../RELEASING.md#why-the-formula-lives-in-formula`. All files exist, all anchors match a heading (README lines 20 and 50; root `RELEASING.md` lines 112 and 381). 0 failures. External links (PR #409790, tap, crates.io) not fetched. Pass |
| 4 | Status match | Parse README's Package Status table, the PACKAGING overview, and each section's first `**Status:**` value | 8 README rows, 8 overview rows. crates.io `**Live**` (overview, sec 8), Fedora `**Built**` (sec 3), Debian `**Built**` (sec 4), WinGet `**Pending**` (sec 1), Homebrew `**Planned (homebrew-core)**` (sec 2), Arch `**Planned**` (sec 5), Flatpak `**Planned**` (sec 6), Snap `**Planned**` (sec 7): all equal to README. Package ID and install command cells also equal README for all 8 rows. `Formula/README.md` contains `**Planned (homebrew-core)**`. Pass |
| 5 | Grep | Plan's `grep -nE 'Published\|AMRSKH/tap/\|409790\|...\|\bready\b'` on both files | 25 hit lines (23 in PACKAGING, 2 in Formula/README), all allowed (list below). No hit for `Published`, `AMRSKH/tap/`, `copr create`, `pacman -Syu angkorfetch`, `brew audit --strict Formula`, `--build-from-source`, or `ready` as a word. Pass |
| 6 | Repository facts | Plan's greps | WinGet: `InstallerType: zip`, `NestedInstallerType: portable`, `RelativeFilePath: angkorfetch.exe`, `Moniker: angkorfetch` (only the `1.1.1` directory exists). Flatpak: only `sdk: org.freedesktop.Sdk`, no `sdk-extensions`. Snap: `base: core22`, `confinement: strict`, `source-tag: v1.1.1`. `build-deb.sh:4` and `build-rpm.sh:4` `VERSION="1.1.1"`; `build-rpm.sh:12` `cp "$SPEC" ...`. Spec line 8 `Source0: %{url}/archive/v%{version}.tar.gz`. `cargo publish` in workflows: no output (exit 1). `brew` in `release.yml`: lines 73 and 75 (comments) and 82 (step name of the `--check` step); no brew command. `release.yml` 237 `Build .deb package`, 255 `dpkg-deb --build`, 257 `Build .rpm package`, 279 `cp rpmbuild/RPMS/x86_64/*.rpm .`. Pass |
| 7 | WinGet schema (optional) | `winget validate --manifest winget-pkgs\manifests\a\AMRSKH\AngkorFetch\1.1.1` (winget v1.29.380) | `Manifest validation succeeded.`, exit 0. Pass. `winget settings --enable` and `winget install --manifest` not run (plan says not to) |
| 8 | Scope and whitespace | `git diff --stat`; `git status --short`; `git diff --check`; `git ls-files --eol` | `Formula/README.md \| 28`, `docs/PACKAGING.md \| 170`, 2 files, 131 insertions, 67 deletions. Status: `M Formula/README.md`, `M docs/PACKAGING.md`, `?? team/Tasks/TASK-009-align-packaging-docs.md`. `git diff --check`: no output, exit 0. Both files `i/lf w/crlf` before and after. No `+`/`-` diff line touches a SHA256 value. Pass |
| 9 | Platform tools | `command -v` (Git Bash) and `Get-Command` (PowerShell) | Not installed: `brew`, `wingetcreate`, `copr-cli`, `rpmbuild`, `makepkg`, `namcap`, `flatpak-builder`, `snapcraft`. **Not verified**: every command in the new text for these tools (`brew tap-new` / `audit` / `install`, `wingetcreate submit` / `update` / `token`, `copr-cli create`, `cd linux/rpm && bash build-rpm.sh`, `makepkg --printsrcinfo`, `updpkgsums`, `namcap`, `pacman -U`, `yay -Syu`, `flatpak-builder`, `snapcraft`). Their syntax is the Advisor's, from each tool's documentation as known to the Advisor; Dev did not check it against live docs |
| 10 | Rust checks | not run | No Rust or `Cargo.*` change (plan item 10) |

Item 5, every hit and its allowed-list entry:

| File:line | Match | Allowed as |
|---|---|---|
| PACKAGING:11 | `cargo install angkorfetch`, `1.1.1` | Live command; overview crates.io row |
| PACKAGING:14 | `409790` | Overview WinGet row |
| PACKAGING:15 | `brew install angkorfetch`, `1.1.1` | Overview Planned row; overview Homebrew row |
| PACKAGING:18 | `sudo snap install angkorfetch` | Overview Planned row |
| PACKAGING:34 | `409790`, `1.1.1` | WinGet Status bullet; PR title in that bullet |
| PACKAGING:70, 73 | `wingetcreate update` | "After the package has been merged" text and block |
| PACKAGING:85 | `brew install angkorfetch` | Homebrew Status bullet |
| PACKAGING:86 | `1.1.1` | Homebrew Tap bullet |
| PACKAGING:112, 114 | `brew install angkorfetch`, `brew uninstall angkorfetch` (substring) | "once accepted into homebrew-core" block |
| PACKAGING:126 | `dnf install angkorfetch` | Built Status bullet ("by name does not work") |
| PACKAGING:147 | `sudo dnf install angkorfetch` | Copr target step, labelled "once the Copr project exists" |
| PACKAGING:153 | `sudo dnf install angkorfetch` | Official Fedora target step, "Target install command once accepted" (kept text) |
| PACKAGING:161 | `dnf upgrades` (substring, prose) | Upgrade comment in the Testing block (plan text). Strictly, the allowed list names only the comment that says a repository is needed; this is the line above it in the same comment pair, not a command |
| PACKAGING:162 | `dnf upgrade angkorfetch` | Comment that says a repository is needed |
| PACKAGING:177 | `apt install angkorfetch` | Built Status bullet ("by name does not work") |
| PACKAGING:198 | `sudo apt install angkorfetch` | PPA target step, labelled "once the repository exists" |
| PACKAGING:208 | `apt upgrades` (substring, prose) | Same as line 161, for APT |
| PACKAGING:209 | `apt upgrade` | Comment that says a repository is needed |
| PACKAGING:246 | `pacman -S` | Official Arch Repositories block, labelled |
| PACKAGING:321 | `cargo install angkorfetch` | Section 8 Install Command (Live) |
| PACKAGING:322 | `1.1.1` | Section 8 Status |
| Formula/README:5 | `cargo install angkorfetch` | Live command |
| Formula/README:15 | `1.1.1` | Tap sentence |

`1.1.1` therefore appears only in the dated live facts (crates.io, tap) and the PR #409790 title,
as the plan requires.

Not verified: rendering on GitHub (not previewed); external links and live facts (no network
checks run by Dev; Background facts were not re-fetched); the platform-tool commands in item 9.

## QA Results

### Round 3 (QA, 2026-11-11, qa-review subagent)

Verdict: Accepted

Independence: this review ran as an independent `qa-review` subagent in **Claude Code**, separate
from the Advisor and Dev agents, and did not write the changes. Host: Windows 11, Git Bash, Python
3.14.10. Focused re-check of `### Dev fixes (Round 2 Lows, ...)` (N1, N2) against
`git diff -- docs/PACKAGING.md Formula/README.md` (nothing staged, HEAD `4daf6b9`). Every check
below was re-run in this round.

No new findings: no Critical, High, Medium, or Low issue on the two changed lines or elsewhere in
the diff.

#### Round 2 Lows

| Item | Status | Evidence (checked in this round) |
|---|---|---|
| N1 `docs/PACKAGING.md:133` | Fixed | The line now reads "...so it has to run from `linux/rpm/` (even there it fails: from `linux/rpm/`, `git archive ... HEAD` packs only that directory, so the archive has no `Cargo.toml`; see also the next item):". `git -C linux/rpm archive --format=tar --prefix=x/ HEAD \| tar -tf -` lists only `x/`, `x/angkorfetch.spec`, `x/build-rpm.sh`. From the root, the same archive has 131 entries (88 files), including `x/Cargo.toml`, `x/Cargo.lock`, `x/src/main.rs`. `build-rpm.sh:10` runs `git archive ... HEAD` with no path argument; `angkorfetch.spec:20` `%setup -q` and `:23` `cargo build --release --locked` need `Cargo.toml`. Together with the next bullet (`:137`, `Source0` name: `angkorfetch.spec:8` against `build-rpm.sh:10`), the section now says correctly that the helper fails as written from any directory. The code block stays under "need fixes before use" (`:131`), so AGENTS.md section 11 still holds. Wording: "from `linux/rpm/`" appears twice in one sentence. That is redundant but does not mislead, so it is not a finding |
| N2 `docs/PACKAGING.md:129` | Fixed | The line now reads "...writes its own spec and packages the binary it just compiled with `rpmbuild`; the `release` job attaches the result to the release." The grammar is correct and the facts hold. The `packages` job (`release.yml:199-202`, `ubuntu-latest`) compiles (`:215`), writes the inline spec (`:263`), runs `rpmbuild` (`:278`), and uploads the artifact `packages` (`:281-283`). The `release` job (`:313`) downloads it (`:318`) and attaches `artifacts/*` with `softprops/action-gh-release` (`:352-354`) |

#### Earlier Lows

| Item | Status | Evidence |
|---|---|---|
| L1 (`:129`), L2 (`:125`), L3 (`:180`), L4 (`:133`), L5 (`:5`), wording (`:76`) | Fixed in Round 2. In this round only `:129` (N2) and `:133` (N1, which completes L4) changed | Command 3: each of these six lines equals the Advisor plan text once only its documented fix is reversed |
| L6 `Formula/README.md:39` | Open, out of scope by design (root `RELEASING.md`, plan contradiction 8) | `RELEASING.md` has no diff (command 10) |

#### Unchanged since Round 2

Command 3 compared all 131 `+` lines of `git diff -U0` with the Advisor plan text. 125 are plan
text verbatim, including every `+` line of `Formula/README.md`. The other six are PACKAGING `:5`,
`:76`, `:125`, `:129`, `:133`, and `:180`. Each one equals the plan once its documented fix (Round 1
fixes table, or the Round 2 N1/N2 table) is reversed, and nothing else. Round 2 recorded the same
six lines and the same word diffs for `:5`, `:76`, `:125`, and `:180`, so only `:129` and `:133`
changed in this round. The rest matches Round 2 as well: HEAD is the same, the diff stat (131
insertions, 67 deletions) is the same, and the Status-bullet, link, and grep-hit line numbers
(commands 4-6) are the same. Limit: there is no byte snapshot of the Round 2 state, so this result
comes from the plan comparison and the matching line positions, not from a byte diff against
Round 2.

Statuses: all 8 channels still match README's Package Status table (command 5). Scope: only the
two docs and this task file. No SHA256, manifest, formula, script, or workflow change (command 10).

#### Commands run (results are from this round)

| # | Command | Result |
|---|---|---|
| 1 | `python scripts/sync_package_manifests.py --check` | `Formula/angkorfetch.rb matches the generated shape (v1.1.1)`, exit 0 |
| 2 | `python -m unittest discover -s scripts -p "test_*.py"` | `Ran 35 tests in 0.168s`, `OK`, exit 0 |
| 3 | Plan comparison (throwaway `python -I` script in the session scratchpad, `qa3/check.py`) | 131 `+` and 67 `-` lines; differs from the plan only at PACKAGING `:5, :76, :125, :129, :133, :180`, each one only by its documented fix; 0 failures |
| 4 | Link and anchor check (same script; fenced code skipped; GitHub heading slugs) | 8 relative targets, 0 failures: PACKAGING `:5, :84, :138, :325`, Formula/README `:3, :5, :25, :39`. Anchors `#package-status--availability`, `#installation`, `#cutting-a-release`, `#why-the-formula-lives-in-formula` match headings. 4 external link uses (PACKAGING `:34, :86, :320`; Formula/README `:13`) not fetched |
| 5 | Status match (same script) | README's 8 rows equal PACKAGING's 8 overview rows (ID, command, status). Section Status bullets: `:34` Pending, `:85` Planned (homebrew-core), `:126` Built, `:177` Built, `:223` Planned, `:273` Planned, `:306` Planned, `:322` Live. Formula/README `:3` Planned (homebrew-core) |
| 6 | Plan grep (`Published\|AMRSKH/tap/\|409790\|...\|\bready\b`) on both files | PACKAGING 23 hit lines (11, 14, 15, 18, 34, 70, 73, 85, 86, 112, 114, 126, 147, 153, 161, 162, 177, 198, 208, 209, 246, 321, 322); Formula/README 2 (5, 15). Same set as Round 2; neither edited line is a hit |
| 7 | `git diff --check` | No output, exit 0 |
| 8 | `git ls-files --eol docs/PACKAGING.md Formula/README.md`; byte scan (same script) | Both `i/lf w/crlf`; 325 and 39 CRLF, 0 bare LF, 0 lone CR, 0 non-ASCII bytes, final CRLF present |
| 9 | `git status --short`; `git diff --cached --name-only`; `git diff --stat` | ` M Formula/README.md`, ` M docs/PACKAGING.md`, `?? team/Tasks/TASK-009-align-packaging-docs.md`; nothing staged; 2 files, 131 insertions, 67 deletions |
| 10 | `git diff HEAD --stat -- Formula/angkorfetch.rb winget-pkgs scripts .github linux flatpak snap README.md RELEASING.md docs/RELEASING.md docs/MAINTAINING.md docs/DEVELOPMENT.md AGENTS.md CONTRIBUTING.md get.sh get.ps1 install.sh install.ps1 Cargo.toml Cargo.lock src`; 64-hex scan of the `+`/`-` diff lines | Empty; 0 hash-like values |
| 11 | `git -C linux/rpm archive --format=tar --prefix=x/ HEAD \| tar -tf -`; the same from the root (entries, files, `grep` for `Cargo.toml`/`src/main.rs`) | 3 entries (2 files) from `linux/rpm/`; 131 entries (88 files, equal to `git ls-tree -r HEAD`) from the root, including `Cargo.toml`. Read-only; output went to a pipe only |
| 12 | Read `linux/rpm/build-rpm.sh`, `linux/rpm/angkorfetch.spec`, and the job and step lines of `release.yml` | As cited in the Round 2 Lows table |

Observation (not a finding): Round 2 row 11 gives "130 from the root". On the same HEAD this round
counts 131 entries (88 files and 43 directories). This does not affect N1.

Not run: `winget settings --enable` and `winget install --manifest` (as instructed); Rust checks (no
Rust or `Cargo.*` change); `bash -n` on the code blocks and installers (no code block or installer
changed in this round); network checks.

#### Not verified

- An actual `build-rpm.sh` / `rpmbuild` run (`rpmbuild` is not on this host). N1 rests on the
  `git archive` listings above and on reading the spec.
- Live facts (PR #409790 state, tap version, crates.io 1.1.1, release assets): not re-fetched in
  this round; taken from Background and Round 1.
- All commands for tools that are not on this host (list in Round 1, Not verified), and GitHub
  rendering of both files.

### Round 2 (QA, 2026-11-11, qa-review subagent)

Verdict: Accepted

Independence: this review ran as an independent `qa-review` subagent in **Claude Code**, separate
from the Advisor and Dev agents, and did not write the changes. Host: Windows 11, Git Bash, Python
3.14.10, git 2.56.0.windows.4. Reviewed the Round 1 Low fixes (`### Dev fixes (Round 1 Lows, ...)`)
and gave `git diff -- docs/PACKAGING.md Formula/README.md` (nothing staged) and both files a fresh
full read. Every check below was re-run in this round.

No Critical, High, or Medium findings. Two new Low findings, N1 and N2, both on lines changed in
this round. I accept both as Low; they do not block the commit. N1 is a factual gap with a one-line
fix, so I recommend fixing it before the commit (a docs-only re-check would cover it).

#### Findings

| # | Severity | File:line | Finding | Evidence | Fix |
|---|---|---|---|---|---|
| N1 | Low | `docs/PACKAGING.md:133` | The new clause "(even there, the build fails until the next item is fixed)" says that fixing the `Source0` file name (`:137`) is enough for `cd linux/rpm && bash build-rpm.sh` to work. It is not. Run from `linux/rpm/`, `git archive ... HEAD` (`build-rpm.sh:10`) packs only that directory, so the source archive has no `Cargo.toml` and `%build` (`angkorfetch.spec:23`, `cargo build`) cannot succeed. As written, the helper fails from every directory: from the root at `cp "$SPEC"` (`:12`), and from `linux/rpm/` because of the subtree-only archive plus the name mismatch. The section still says "need fixes before use", so it does not present the helper as working (AGENTS.md section 11 still met). Round 1, the Advisor (plan contradiction 3), and Dev all missed the subtree problem | `git -C linux/rpm archive --format=tar --prefix=angkorfetch-1.1.1/ HEAD \| tar -tf -` lists only `angkorfetch-1.1.1/`, `angkorfetch-1.1.1/angkorfetch.spec`, `angkorfetch-1.1.1/build-rpm.sh` (2 files; 130 when run from the root) | Replace the parenthetical with "(even there it fails: from `linux/rpm/`, `git archive ... HEAD` packs only that directory, so the archive has no `Cargo.toml`; see also the next item)". Record the subtree problem in the follow-up task for `linux/rpm/` |
| N2 | Low | `docs/PACKAGING.md:129` | Grammar after the L1 fix: "writes its own spec, packages the binary it just compiled with `rpmbuild`; the `release` job ..." lists two actions with no "and". The facts are correct. The wording came from Round 1's own suggested fix | Word diff against the plan line: "`rpmbuild`, and" became "`rpmbuild`; the `release` job" | "...writes its own spec and packages the binary it just compiled with `rpmbuild`; the `release` job attaches the result to the release." |

#### Round 1 Lows

| Item | Status | Evidence (re-read in this round) |
|---|---|---|
| L1 `docs/PACKAGING.md:129` | Fixed (grammar nit: N2) | `packages` only uploads the artifact `packages` (`release.yml:281-286`); `release` downloads it (`:318`) and attaches `artifacts/*` with `softprops/action-gh-release` (`:351-354`) |
| L2 `docs/PACKAGING.md:125` | Fixed | `packages` has `runs-on: ubuntu-latest` (`release.yml:199-202`); inline spec `Release: 1%{?dist}` (`:266`); `rpmbuild` (`:278`) |
| L3 `docs/PACKAGING.md:180` | Fixed | `build-deb.sh:5` `BIN="angkorfetch"`, `:28` `cp "target/release/$BIN"`, `:31` `cp README.md`; `cargo build` at `:27` writes to the root `target/` from any subdirectory |
| L4 `docs/PACKAGING.md:133` | Fixed as requested; the added clause is incomplete (N1) | `build-rpm.sh:10`, `angkorfetch.spec:8` |
| L5 `docs/PACKAGING.md:5` | Fixed | Consistent with `:85`, `:99`, `Formula/README.md:17`, and README `:58` (unchanged) |
| L6 `Formula/README.md:39` | Open, out of scope by design (root `RELEASING.md`) | `RELEASING.md:391-392` still says "byte-identical by design" and "The tap is the user-facing one"; `RELEASING.md` has no diff |
| Wording `docs/PACKAGING.md:76` | Fixed | Now `wingetcreate token --store`; not checked with the tool (not installed) |

Fresh full read: no other problem found. Every overview row and every section `**Status:**` bullet
still matches README for all 8 channels. Nothing new is presented as working. The CI, sync, WinGet,
Flatpak, Snap, PKGBUILD, and formula facts behind the text were re-read and hold:
`release.yml:82-83` (`--check` in the `test` job, which starts at `:38`), `:242-252` (inline
`control`), `:263-277` (inline spec); `sync-packages.yml:191` branch
`automation/sync-packages-${TAG}`, `:258` `--base main`; `sync_package_manifests.py:290-349`
(directory rename, `PackageVersion`, `InstallerUrl`, `InstallerSha256`); `PKGBUILD:11-12`; Flatpak
manifest `sdk: org.freedesktop.Sdk`, no `sdk-extensions`, git source `tag:`; `snapcraft.yaml`
`base: core22`, `confinement: strict`, `source-tag:`; `Formula/angkorfetch.rb` `on_macos` and
`on_linux` with `on_arm` and `on_intel`; root `RELEASING.md` "Cutting a release", step 1 lists the
hand-set versions. No workflow references `linux/`, `build-deb`, `build-rpm`, `homebrew-tap`,
`cargo publish`, or a `brew` command.

#### Commands run (results are from this round)

| # | Command | Result |
|---|---|---|
| 1 | `python scripts/sync_package_manifests.py --check` | `Formula/angkorfetch.rb matches the generated shape (v1.1.1)`, exit 0 |
| 2 | `python -m unittest discover -s scripts -p "test_*.py"` | `Ran 35 tests in 0.169s`, `OK`, exit 0 |
| 3 | Link and anchor check (throwaway `python -I` script in the session scratchpad; fenced code skipped; GitHub heading slugs) | 8 relative targets, 0 failures: `docs/PACKAGING.md:5,84,138,325`, `Formula/README.md:3,5,25,39`; all files exist; anchors `#package-status--availability`, `#installation`, `#cutting-a-release`, `#why-the-formula-lives-in-formula` match headings. 4 external link uses (3 URLs: PR #409790, the tap, crates.io) not fetched |
| 4 | `git diff --check` | No output, exit 0 |
| 5 | `git ls-files --eol docs/PACKAGING.md Formula/README.md` | Both `i/lf w/crlf`. Byte scan: 325 and 39 CRLF, 0 bare LF, 0 non-ASCII bytes, final CRLF present |
| 6 | `git status --short`; `git diff --cached --name-only`; `git diff --stat` | ` M Formula/README.md`, ` M docs/PACKAGING.md`, `?? team/Tasks/TASK-009-align-packaging-docs.md`; nothing staged; 2 files, 131 insertions, 67 deletions |
| 7 | `git diff HEAD --stat -- Formula/angkorfetch.rb winget-pkgs scripts .github linux flatpak snap README.md RELEASING.md docs/RELEASING.md docs/MAINTAINING.md AGENTS.md`; 64-hex scan of `+`/`-` diff lines | Empty; 0 hash-like values. No SHA256, manifest, formula, script, or workflow change |
| 8 | Every `+` line of the diff compared with the Advisor plan text (scratchpad script), then a word diff of the lines that differ | Only `docs/PACKAGING.md:5, 76, 125, 129, 133, 180` differ (plus the `---` at `:315`, which the plan asks for in prose). Each word diff equals the Dev fixes table exactly; no other rewording |
| 9 | Status match (scratchpad script) | README 8 rows vs PACKAGING 8 rows: package ID, install command, status identical. Status bullets `:34` Pending, `:85` Planned (homebrew-core), `:126` Built, `:177` Built, `:223` Planned, `:273` Planned, `:306` Planned, `:322` Live. `Formula/README.md:3` Planned (homebrew-core) |
| 10 | Plan grep (`Published\|AMRSKH/tap/\|409790\|...\|\bready\b`) on both files | 25 hit lines (23 and 2), the same file:line set as Dev and Round 1, each on the plan's allowed list; none of the six edited lines is a hit |
| 11 | `git -C linux/rpm archive --format=tar --prefix=angkorfetch-1.1.1/ HEAD \| tar -tf -`; same from the root with `wc -l` | 2 files from `linux/rpm/`, 130 from the root (evidence for N1). Read-only, written to a pipe only |
| 12 | `command -v` for `wingetcreate brew copr-cli rpmbuild dpkg-deb makepkg namcap flatpak-builder snapcraft` | All absent |

Not run: `winget settings --enable` and `winget install --manifest` (as instructed); Rust checks
(no Rust or `Cargo.*` change); `bash -n` on the code blocks (no code block changed in this round;
Round 1 parsed all of them); network checks.

#### Not verified

- `wingetcreate token --store`: the tool is not installed. As far as I know, `-s, --store` caches a
  token for `submit` and `update --submit`; I am not sure of the exact current option set.
- Runtime behaviour of `build-rpm.sh` and `build-deb.sh` (no `rpmbuild`, no `dpkg-deb`). N1 is based
  on the `git archive` output above plus reading the spec, not on an `rpmbuild` run.
- `%{?dist}` expanding to empty on Ubuntu: relies on the v1.2.1 asset name
  `angkorfetch-1.2.1-1.x86_64.rpm` from Background and Round 1; not re-fetched in this round.
- Live facts (PR #409790 state, tap version, crates.io 1.1.1, release assets): not re-fetched in this
  round; taken from Background and Round 1.
- All commands for tools not on this host (list in Round 1, Not verified); GitHub rendering of both
  files.

### Round 1 (QA, 2026-11-11, qa-review subagent)

Verdict: Accepted

Independence: this review ran as the `qa-review` subagent in **Claude Code**, separate from the
Advisor and Dev agents, and did not write the changes. Host: Windows 11, Git Bash and Windows
PowerShell 5.1, Python 3.14.10, winget v1.29.380. Reviewed `git diff -- docs/PACKAGING.md
Formula/README.md` (nothing staged) and both files in full, and re-ran every check below.

No Critical, High, or Medium findings. The six Low findings do not block the commit and are
accepted as Low. L1 and L2 are one-line wording fixes worth making if the file is touched again;
a docs-only re-check would cover them.

#### Findings

| # | Severity | File:line | Finding | Evidence | Fix |
|---|---|---|---|---|---|
| L1 | Low | `docs/PACKAGING.md:129` | "the `packages` job ... packages the binary ... with `rpmbuild`, and attaches the result to the release" names the wrong job for the attach step. The end result (`.rpm` on the release) is true | `packages` only uploads the workflow artifact `packages` (`release.yml:281-286`); the `release` job downloads it and attaches `artifacts/*` with `softprops/action-gh-release` (`release.yml:313-354`). The DEB sentence (`:186`) only says the `packages` job "builds" the `.deb`, which is accurate, so section 4 does not have this issue | "...packages the binary it just compiled with `rpmbuild`; the `release` job attaches it to the release." |
| L2 | Low | `docs/PACKAGING.md:125` | "because the release job builds on Ubuntu" can be read as the `release` job, which builds nothing | The `.rpm` is built in `packages` (`release.yml:199-202` `runs-on: ubuntu-latest`, `rpmbuild` at `:278`). The no-dist-suffix fact itself holds: the v1.2.1 asset is `angkorfetch-1.2.1-1.x86_64.rpm` | "because the `packages` job builds on Ubuntu" |
| L3 | Low | `docs/PACKAGING.md:180` | The reason in parentheses is incomplete: `cargo build` finds `Cargo.toml` from any subdirectory; what needs the root are the relative copies. The instruction (run from the root) is correct | `linux/deb/build-deb.sh:28` `cp "target/release/$BIN"`, `:31` `cp README.md` | "(the helper copies `target/release/angkorfetch` and `README.md` by relative path)" |
| L4 | Low | `docs/PACKAGING.md:133-137` | The `cd linux/rpm && bash build-rpm.sh` block can read as a working command, but per the next bullet `rpmbuild` still fails (no `v<version>.tar.gz` in `SOURCES`). The lead-in "need fixes before use" labels it, so AGENTS.md section 11 is met | `build-rpm.sh:10`, `angkorfetch.spec:8` | Optional: end the first bullet with "Even from `linux/rpm/`, the build fails until the next item is fixed." |
| L5 | Low | `docs/PACKAGING.md:5` | The **Planned** definition says "the manifest, spec, or formula in this repository is the starting point"; for Homebrew, `:99` and `Formula/README.md:17` say the homebrew-core formula will be a different, build-from-source formula | `c40983a` ("homebrew-core build from source") | "...the manifest or spec in this repository is the starting point (Homebrew needs a new build-from-source formula)." |
| L6 | Low | `Formula/README.md:39` (kept line) | The kept link targets root `RELEASING.md` "Why the formula lives in Formula/", which still says the two copies are "byte-identical by design" and the tap is "the user-facing one" and canonical. Already recorded as plan contradiction 8 (out of scope) | `RELEASING.md:391-392` | Follow-up task; no change in this task |

Observation (not a finding): Dev results give the old sizes as 270 and 32 lines;
`git show HEAD:<file> | wc -l` gives 269 and 31 (both old files end with a newline). The new
sizes (325, 39) and the diff stat match. No effect on the docs.

#### Acceptance criteria

| Criterion | Result | Evidence |
|---|---|---|
| Statuses match README | Pass | README `:54-61` vs PACKAGING `:11-18`: package ID, install command, and status identical for all 8 rows. Section Status bullets `:34` Pending, `:85` Planned (homebrew-core), `:126` Built, `:177` Built, `:223` Planned, `:273` Planned, `:306` Planned, `:322` Live. `Formula/README.md:3` Planned (homebrew-core). README not modified |
| Homebrew | Pass | PACKAGING `:15`, `:85-86`, `:99`; `Formula/README.md:3-5`, `:13-17`. Read-only `gh`: tap not archived, last push 2026-07-30; its four `url` lines point to v1.1.1; its formula is byte-identical to `HEAD:Formula/angkorfetch.rb` (so "holds a copy" is true today); `gh search prs --repo Homebrew/homebrew-core angkorfetch` returned none ("not submitted" consistent). No `homebrew-tap` reference in `.github/workflows/` ("CI does not update it") |
| WinGet | Pass | PACKAGING `:14`, `:34`, `:52-57`. `gh pr view 409790`: `CLOSED`, `mergedAt: null`, closed 2026-08-20, title matches the quoted one. It is the only PR for `AMRSKH.AngkorFetch`; `manifests/a/AMRSKH` does not exist in `microsoft/winget-pkgs` (HTTP 404). `d079c68` renamed the files to `AMRSKH.AngkorFetch.*.yaml` |
| Arch AUR, crates.io Live with lag | Pass | PACKAGING `:16`, `:223`, `:231-247`; `:11`, `:317-325` |
| Built rows use local files; repository-only commands labelled | Pass | `:12-13`, `:126`, `:159`, `:177`, `:206`. Repository-only commands sit under "once the Copr project exists" `:144`, "once accepted" `:151`, "once the repository exists" `:191`, "once on the AUR" `:236`, "once in the official repositories" `:244`, "once the package is on the AUR" `:259`, and the note at `:20` covers the overview. `dnf upgrade` / `apt upgrade` appear only in comments saying a repository is needed (`:162`, `:209`) |
| Current version 1.2.1 | Pass | `:125`, `:159`, `:206`; the v1.2.1 release assets (read-only `gh release view`) are 6 archives, `angkorfetch_1.2.1_amd64.deb`, `angkorfetch-1.2.1-1.x86_64.rpm`, `checksums.txt`. `1.1.1` remains only in the dated tap and crates.io facts and the PR title |
| Maintainer procedures intact and correct | Pass (see not verified) | WinGet `:61-76`, Copr `:141-148`, Fedora `:149-154`, PPA and Debian `:189-201`, AUR and official repositories `:234-247`, Flathub `:282-289`, Snap `:308-313`, crates.io `:324-325` |
| `--check` and script tests pass | Pass | Commands 1-2 below |
| Relative links resolve | Pass | Command 3 below |
| Independent QA | Accepted | This round |

Repository facts behind the new sentences, read directly: WinGet manifests (only `1.1.1/`;
`InstallerType: zip`, `NestedInstallerType: portable`, `RelativeFilePath: angkorfetch.exe`,
`Moniker: angkorfetch`); `_winget_dir` renames the single version directory and `update_winget`
rewrites `PackageVersion`, the `InstallerUrl` tag, and `InstallerSha256`
(`sync_package_manifests.py:290-352`); `sync-packages.yml:191` branch
`automation/sync-packages-${TAG}`, `:257-258` `gh pr create --base main`, nothing pushed to
`microsoft/winget-pkgs`; the script touches only `Formula/angkorfetch.rb` and `winget-pkgs/`
(`Formula/README.md` is hand-written); `release.yml:82-83` runs `--check` in `test`, and no
workflow runs `brew`, `cargo publish`, `flatpak`, `snapcraft`, or the `linux/` helpers;
`release.yml:237-255` inline `control` (Section, Priority, `Architecture: amd64`, Maintainer,
Description), `:257-279` inline spec; `build-rpm.sh:4,10,12`, `angkorfetch.spec:2,8`,
`build-deb.sh:4,28,31`, `PKGBUILD:3,11,12`; Flatpak manifest has `sdk: org.freedesktop.Sdk`, no
`sdk-extensions`, git source `tag:`; `snapcraft.yaml` `source-tag:`, `base: core22`,
`confinement: strict`; root `RELEASING.md:112` "Cutting a release", step 1 lists the hand-set
versions; `origin/automation/sync-packages-v1.2.1` (`21ac68f`) is not merged into HEAD;
`c40983a` and `d079c68` messages match the cited facts.

Scope and "keep" text: a script compared every `+` line of the diff with the Advisor plan: 0 lines
not in the plan. The removed lines are only those the plan names. Markdown headings, section
numbers 1-7, `**Status:**` markers, and `---` separators are unchanged; section 8 is appended;
`Formula/README.md` "Why this directory is named Formula/" has no hunk. No inbound link to a
removed heading (`git grep 'PACKAGING.md#'` empty; only `docs/PACKAGING.md:84` links to
`Formula/README.md`, without an anchor). No 64-hex value on any `+`/`-` line.
`git diff HEAD --stat` on `Formula/angkorfetch.rb winget-pkgs scripts .github linux flatpak snap
README.md` is empty.

#### Commands run (results are from this round)

| # | Command | Result |
|---|---|---|
| 1 | `python scripts/sync_package_manifests.py --check` | `Formula/angkorfetch.rb matches the generated shape (v1.1.1)`, exit 0 |
| 2 | `python -m unittest discover -s scripts -p "test_*.py"` | `Ran 35 tests`, `OK`, exit 0 |
| 3 | Link and anchor check (throwaway `python -I` script in the session scratchpad; fenced code skipped; GitHub heading slugs) | 8 relative targets (4 per file), 0 failures: `../README.md#package-status--availability` (both files), `../README.md#installation`, `../Formula/README.md`, `../RELEASING.md#cutting-a-release`, `../RELEASING.md#why-the-formula-lives-in-formula`, `../AGENTS.md`, `../scripts/sync_package_manifests.py`. 4 external links not fetched as pages (their facts were checked with `gh`, row 9) |
| 4 | `git diff --check` | No output, exit 0 |
| 5 | `git ls-files --eol docs/PACKAGING.md Formula/README.md` | Both `i/lf w/crlf`; byte scan: 325 and 39 CRLF, 0 bare LF, 0 non-ASCII bytes, final newline present |
| 6 | `git status --short`; `git diff --stat`; `git diff --cached --stat` | `M Formula/README.md`, `M docs/PACKAGING.md`, `?? team/Tasks/TASK-009-align-packaging-docs.md`; 2 files, 131 insertions, 67 deletions; nothing staged |
| 7 | Plan grep (`Published\|AMRSKH/tap/\|409790\|...\|\bready\b`) on both files | 25 hit lines, each matching the plan's allowed list; none for `Published`, `AMRSKH/tap/`, `copr create`, `pacman -Syu angkorfetch`, `brew audit --strict Formula`, `--build-from-source`, `ready` |
| 8 | `winget validate --manifest winget-pkgs/manifests/a/AMRSKH/AngkorFetch/1.1.1`; `winget settings --help`; `winget --info`; `winget install --help` | `Manifest validation succeeded.`, exit 0. `settings` has `--enable` for admin settings; `LocalManifestFiles` is listed, state `Disabled`; `install` has `-m,--manifest`. `winget settings --enable` and `winget install --manifest` were **not** run |
| 9 | Read-only `gh`: `pr view 409790 -R microsoft/winget-pkgs`; `search prs` (winget-pkgs, homebrew-core); `release view -R AMRSKH/angkorfetch`; `api repos/AMRSKH/homebrew-tap` and its `Formula/angkorfetch.rb` (only `url` lines printed; bytes compared with `cmp`) | As in the acceptance table |
| 10 | `bash -n` on the 17 `bash` blocks of both files; PowerShell `Parser::ParseFile` on the 3 `powershell` blocks with `<version>` replaced by `1.2.1` | All parse. With the literal `<version>` the PowerShell blocks do not parse; that is the intended placeholder (plan Risks) |
| 11 | `command -v` (Git Bash) and `Get-Command` (PowerShell) for `wingetcreate brew copr-cli rpmbuild makepkg namcap flatpak-builder snapcraft updpkgsums yay`; `wsl -l` | All absent; WSL not installed |

Not run: Rust checks (`cargo fmt --check`, clippy, `cargo test`), because the diff has no Rust or
`Cargo.*` change; installer `bash -n` (no installer change).

#### Not verified

- Every command for a tool that is not on this host: `brew tap-new` / `--repository` / `audit` /
  `install`, `wingetcreate submit` / `update` / `token`, `copr-cli create`, `rpmbuild`
  (`build-rpm.sh`), `build-deb.sh` (`dpkg-deb`), `makepkg`, `makepkg --printsrcinfo`,
  `updpkgsums`, `namcap`, `pacman -U`, `yay -Syu`, `flatpak-builder`, `snapcraft`, `dnf` and `apt`
  local-file installs. Only shell syntax was checked (row 10). As far as I know the options match
  each tool's documentation (`wingetcreate update <id> --version --urls --submit`; `brew audit`
  takes a formula name, not a path; `brew --repository <user/tap>` prints the tap directory;
  `updpkgsums` is in `pacman-contrib`; the AUR rejects pushes without `.SRCINFO`; Flathub takes
  new apps against `new-pr`). Not sure: that `wingetcreate submit` accepts a manifest
  directory (I believe it does), that `brew tap-new` creates `Formula/` in the new tap (I believe
  it does), and that bare `wingetcreate token` stores a token (the store flag is `--store`; the
  text names the subcommand only).
- `winget install --manifest` after enabling `LocalManifestFiles` (not run on purpose).
- crates.io latest version 1.1.1 (2026-07-30): not re-fetched by QA; taken from Background.
- WinGet rejection reason ("manifest file names"): taken from `c40983a`; PR review comments not read.
- GitHub rendering of both files (not previewed).

## Documentation Updates

Docs, 2026-11-11. This stage ran in the `docs` subagent (Claude Code), after QA Round 3 accepted
the change. Input: `git diff -- docs/PACKAGING.md Formula/README.md` (unchanged by this stage),
then `git grep` over every tracked `*.md` outside `team/Tasks/` for the two file names, `Formula/`,
and the facts the diff changed (tap, `wingetcreate`, `brew audit`, Copr, Flathub, crates.io,
`build-rpm`/`build-deb`, `409790`, `Published`).

**Files changed in this stage: none** (only this section of the task file). No further
documentation changes are needed in this task's scope. The checks, file by file:

| File | Checked | Result |
|---|---|---|
| `README.md` | Package Status table `:52-61`, Installation `:20-46`, platform table `:143-150`, Update `:155-161`, Removal `:183-187`, Manual Installation artifact names `:215-216`, Development links `:245` | No contradiction introduced, so no change. All 8 rows equal the PACKAGING overview. "installs a newer crates.io version if there is one" (`:160`) fits the stated 1.1.1 lag. `.rpm`/`.deb` names and remove commands match PACKAGING `:125`, `:176`, `:165`, `:212`. README never mentions the tap |
| `AGENTS.md` | `:12`, `:23-25` ("See docs/PACKAGING.md for the status of each"), `:44`, `:137`, `:200`, `:249` | No change needed. `:44` says "`Formula/` Homebrew formula, GENERATED by scripts/sync_package_manifests.py". It describes the formula, and `Formula/angkorfetch.rb` is still generated. `Formula/README.md` is not a formula, and it was already hand-written in that directory before this task (the old text "Both are generated" meant the formula and the tap copy). This task adds no file and changes no generation, so it is not a contradiction. Optional wording for a later harness task (not required, not edited): `Formula/  Homebrew formula angkorfetch.rb, GENERATED by scripts/sync_package_manifests.py`. Pre-existing and also optional: `:45` calls `winget-pkgs/` "GENERATED", but the script patches it in place (root `RELEASING.md:348`) |
| `CONTRIBUTING.md` | `:113`, `:114`, `:119` | Still accurate: PACKAGING holds the submission processes and distribution statuses |
| `docs/DEVELOPMENT.md` | Tree `:22-23`, `:46`; `:86-88` | Still accurate: the formula is "generated from template", and `--check` is described as a shape check. The tree leaves out `Formula/README.md`, but it did before this task too (optional, not a contradiction) |
| `team/Docs/README.md` | `:41` | Agrees with the new PACKAGING intro ("change both together") |
| `team/Dev/README.md` | `:30`, `:58` | Hook and SHA256 rules, unaffected |
| Wrappers (`.claude/`, `.opencode/`, `.agents/`), `team/` role and command files | grep | No packaging status or channel statement |
| `improve plan.md` | `:11-20`, `:195-215`, `:707-725` | The maintainer's original prompt (commit `5c3fa34`), giving target commands and goals rather than current status. It already says the tap should not be the main path. Not changed |
| `docs/MAINTAINING.md`, `docs/RELEASING.md`, root `RELEASING.md` | Full grep, sections read | Still contradict the new text. Out of scope by design (Out of Scope; QA L6), so they are not fixed here. Consolidated below |

### Follow-up task (consolidated; proposed, not created)

Consolidates plan items 1-10, QA Round 1 L6, the Round 2 N1 note for `linux/rpm/`, the Out of
Scope list, and items this stage found (marked *new*). Line numbers are for HEAD `4daf6b9`.

Docs (one docs task: align `docs/MAINTAINING.md`, `docs/RELEASING.md`, root `RELEASING.md` with
README and `docs/PACKAGING.md`):

1. **Tap presented as canonical or as a release step** (plan 8, QA L6). Root `RELEASING.md:107`,
   `:150`, `:391-393` ("byte-identical by design", "the user-facing one, so it is the canonical
   shape"), which `Formula/README.md:39` links to. Also `docs/MAINTAINING.md:127`. *New*: the
   diagrams at root `RELEASING.md:85` ("Homebrew tap/core") and `docs/MAINTAINING.md:53`
   ("Homebrew tap").
2. **`wingetcreate update` for a package not in `microsoft/winget-pkgs`** (plan 7).
   `docs/MAINTAINING.md:120-123`, root `RELEASING.md:149`. Use the new-package `wingetcreate submit`
   flow from PACKAGING section 1. Not verified: `copr-cli build-package ... --clone-url`
   (`docs/MAINTAINING.md:133`), since `--clone-url` may belong to `buildscm` / `add-package-scm`.
3. ***New*: release steps written as if every channel is already published.** Root
   `RELEASING.md:105-110`, `:148-156` and `docs/MAINTAINING.md:130-146`. They trigger a Copr build
   in a project that does not exist yet (PACKAGING section 3 says to create it first), "update"
   Flathub for an app that is not on Flathub (a first submission goes to `new-pr`, PACKAGING
   section 6), and in step 7 "verify each package manager resolves the new version". Label these
   steps "once published" and point to PACKAGING for first submissions.
4. **`brew audit` presented as enforced** (plan 9). `docs/RELEASING.md:41` ("enforces `brew audit
   --strict` standards"). *New*, softer: root `RELEASING.md:376-379` ("a formula that would fail
   audit blocks the tag"). CI checks only the generator shape; PACKAGING section 2 says CI does not
   run `brew audit`.
5. ***New*: crates.io missing from the release steps.** Root `RELEASING.md` steps 6-7 and
   `docs/MAINTAINING.md` section 4 have no crates.io step, although the channel is Live and lags at
   1.1.1 (PACKAGING section 8: manual, needs maintainer approval).
6. **Stale artifact counts** (Out of Scope). `docs/RELEASING.md:26`, `:35` ("8 verifiable
   artifacts", "all 7 binary artifacts"); root `RELEASING.md:95` ("5 target binaries"). The v1.2.1
   release has 9 assets, including the musl archive.

Repository files (code or packaging task; no SHA256 by hand):

7. **Hand-set pins at 1.1.1** (plan 2, Out of Scope). `linux/arch/PKGBUILD`, `snap/snapcraft.yaml`,
   `linux/deb/build-deb.sh`, `linux/rpm/build-rpm.sh`, `linux/rpm/angkorfetch.spec` (no 1.2.1
   changelog entry), the Flatpak manifest `tag`, and the metainfo `<release>`. Root `RELEASING.md`
   "Cutting a release", step 1, lists them all; v1.2.1 skipped that step.
8. **`linux/rpm/build-rpm.sh` fails from every directory** (plan 3, QA Round 2 N1). From the
   repository root it fails at the relative `cp "$SPEC"` (`:12`). From `linux/rpm/`,
   `git archive ... HEAD` packs only that subtree (no `Cargo.toml`), and the archive name
   `angkorfetch-<v>.tar.gz` does not match the spec's `Source0` name `v<v>.tar.gz`
   (`angkorfetch.spec:8`).
9. **`linux/arch/PKGBUILD` uses `sha256sums=('SKIP')`** (plan 6).
10. **Flatpak manifest has no Rust SDK extension and no vendored Cargo sources** (plan 5). Also
    check whether Flathub's policy accepts a console-only app. Not verified.
11. **No build-from-source Homebrew formula** for homebrew-core (plan 10). The maintainer plans
    one (`c40983a`); this is a to-do, not a conflict.

External actions (each needs maintainer approval):

12. Merge `origin/automation/sync-packages-v1.2.1` (`21ac68f`) so `Formula/` and `winget-pkgs/` move
    off 1.1.1 (plan 1). Do this before any WinGet resubmission or tap update.
13. Publish 1.2.1 to crates.io; resubmit to WinGet; update or retire `AMRSKH/homebrew-tap`; Copr,
    AUR, Flathub, Snap, and homebrew-core submissions.

Optional harness wording (not contradictions): `AGENTS.md:44` and `:45` and the
`docs/DEVELOPMENT.md` tree, as noted in the table above.

Not verified in this stage: no command in the docs was run, because this stage edited no command
text. GitHub rendering was not previewed, and the live facts were not re-fetched (they come from
Background and QA Round 1).

## Final Validation (main session, 2026-11-11)

| Check | Result |
|---|---|
| `cargo fmt --check` | exit 0 (no Rust change; run as a sanity check) |
| `python -m unittest discover -s scripts -p "test_*.py"` | Ran 35 tests, OK, exit 0 |
| `python scripts/sync_package_manifests.py --check` | `Formula/angkorfetch.rb matches the generated shape (v1.1.1)`, exit 0 |
| `bash -n get.sh && bash -n install.sh` | exit 0 (installers untouched) |
| `git diff --check` | clean, exit 0 |
| `git ls-files --eol` | `docs/PACKAGING.md`, `Formula/README.md`: `i/lf w/crlf` |
| `git diff --stat` | `Formula/README.md` 28, `docs/PACKAGING.md` 170; 131 insertions, 67 deletions; nothing staged |
| 64-hex (SHA256-like) values on diff lines | 0 |

Not run: `cargo clippy` and `cargo test` (no Rust or `Cargo.*` change).

## Git Commit

- Commit Message: `docs(packaging): align PACKAGING and Formula README with README statuses [TASK-009]`
- Commit Hash: `c2e201ad931ed865e90dd8b4add97d6a6a00583d`

## Final Report

- `docs/PACKAGING.md`: status-definition intro; 8-row overview matching README (crates.io Live added,
  Built rows use local-file installs, WinGet `AMRSKH.AngkorFetch` Pending with PR #409790 closed
  unmerged, Homebrew Planned (homebrew-core) with the tap described as not the install path, Arch via
  the AUR); sections 1-7 corrected step by step with reasons; new section 8 (crates.io).
- `Formula/README.md`: the `brew install AMRSKH/tap/angkorfetch` instructions removed; Planned
  status, what the generated formula is, and the tap's state described.
- Stages: Advisor, Dev, QA (Rounds 1-3, all Accepted; Lows from Rounds 1 and 2 fixed), and Docs
  ran in subagents; orchestration, final validation, and commits in the main session.
- Open: QA L6 and the consolidated follow-up list under Documentation Updates (out of scope).
- Not verified: commands for `brew`, `wingetcreate`, `copr-cli`, `rpmbuild`, `makepkg`/`namcap`,
  `flatpak-builder`, `snapcraft` (not installed here); GitHub rendering; live facts after
  2026-11-11.

## Notes and Limitations

- Live facts above are as of 2026-11-11 and can change.
- Advisor (2026-11-11): no maintainer decision is needed for this task; status stays
  In Progress. Several replacement commands (`brew tap-new`/`audit`, `wingetcreate submit`/
  `update`, `copr-cli`, `makepkg`/`namcap`, `flatpak-builder`) cannot be run on this Windows host.
  Their syntax comes from each tool's documentation as the Advisor knows it. QA records them as
  not run.
