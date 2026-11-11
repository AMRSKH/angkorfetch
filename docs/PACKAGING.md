# AngkorFetch Packaging and Distribution Guide

This document defines package specifications, manifest paths, validation workflows, and distribution channels for every supported platform.

Channel statuses here must match the [Package Status table in the README](../README.md#package-status--availability); change both together. **Live**: installable from the channel today. **Built**: the package file is attached to each GitHub Release, but no repository serves it. **Pending**: a submission was made and must be redone. **Planned**: not yet submitted to the target channel; the manifest or spec in this repository is the starting point (Homebrew needs a new build-from-source formula; see section 2).

## Packaging Overview & Status

| Ecosystem | Target Package ID | Native Install Command | Current Status | Distribution Channel |
| --- | --- | --- | --- | --- |
| **crates.io (Cargo)** | `angkorfetch` | `cargo install angkorfetch` | **Live** | crates.io; latest published version 1.2.1, matching the v1.2.1 release |
| **Fedora (DNF)** | `angkorfetch` | `sudo dnf install ./angkorfetch-*.x86_64.rpm` | **Built** | `.rpm` (x86_64) in GitHub Releases; Copr / Fedora repo planned |
| **Debian/Ubuntu (APT)** | `angkorfetch` | `sudo apt install ./angkorfetch_*_amd64.deb` | **Built** | `.deb` (amd64) in GitHub Releases; PPA / APT repo planned |
| **WinGet** | `AMRSKH.AngkorFetch` | `winget install AMRSKH.AngkorFetch` | **Pending** | `microsoft/winget-pkgs`: first PR #409790 closed unmerged; resubmission pending |
| **Homebrew** | `angkorfetch` | `brew install angkorfetch` | **Planned (homebrew-core)** | `Homebrew/homebrew-core` submission planned; `AMRSKH/homebrew-tap` is not the install path (formula there at v1.1.1) |
| **Arch Linux (AUR)** | `angkorfetch` | `yay -S angkorfetch` | **Planned** | PKGBUILD in `linux/arch/`; not yet on the AUR |
| **Flatpak** | `io.github.AMRSKH.angkorfetch` | `flatpak install flathub io.github.AMRSKH.angkorfetch` | **Planned** | Manifest and AppStream metainfo in `flatpak/`; Flathub submission pending |
| **Snap** | `angkorfetch` | `sudo snap install angkorfetch` | **Planned** | Manifest in `snap/`; Snap Store registration pending |

Only the **Live** and **Built** commands work today; the others are the target commands once the channel is published. Versions in this table were checked on 2026-11-11.

---

## 1. Windows: WinGet

### Metadata
- **Package Name:** AngkorFetch
- **Package Identifier:** `AMRSKH.AngkorFetch`
- **Moniker:** `angkorfetch`
- **Manifest Location:** `winget-pkgs/manifests/a/AMRSKH/AngkorFetch/<version>/`
- **Installer Type:** `zip` containing portable `angkorfetch.exe`
- **Target URL:** `https://github.com/AMRSKH/angkorfetch/releases/download/v<version>/angkorfetch-windows-x86_64.zip`
- **Checksum:** SHA256 of `angkorfetch-windows-x86_64.zip` from `checksums.txt`
- **Status:** **Pending**. The first submission, [microsoft/winget-pkgs#409790](https://github.com/microsoft/winget-pkgs/pull/409790) ("New package: AMRSKH.AngkorFetch version 1.1.1"), was rejected over the manifest file names and closed without being merged (2026-08-20). The files were renamed to `AMRSKH.AngkorFetch.*.yaml` (commit `d079c68`); resubmission is pending, so the package is not in the WinGet community repository yet.

### Manifest Structure
1. `AMRSKH.AngkorFetch.yaml`: Version manifest referencing `en-US` locale.
2. `AMRSKH.AngkorFetch.installer.yaml`: Points to zip release binary with matching SHA256.
3. `AMRSKH.AngkorFetch.locale.en-US.yaml`: Metadata, description, tags, publisher URL, and moniker.

### Validation & Testing
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

### Automation & Publishing
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

`submit` and `update --submit` open the pull request on GitHub and need a GitHub token (`--token`, or one stored with `wingetcreate token --store`).

---

## 2. macOS: Homebrew

### Metadata
- **Package Name:** `angkorfetch`
- **Formula Path:** `Formula/angkorfetch.rb` (generated; see [Formula/README.md](../Formula/README.md))
- **Status:** **Planned (homebrew-core)**. The maintainer plans a build-from-source formula in `Homebrew/homebrew-core` (commit `c40983a`); it has not been submitted. `brew install angkorfetch` works only after it is accepted.
- **Tap:** [`AMRSKH/homebrew-tap`](https://github.com/AMRSKH/homebrew-tap) still exists, but it is no longer the documented install path and CI does not update it. Its formula points to v1.1.1 (checked 2026-11-11), behind the v1.2.1 release.
- **Architectures (generated formula):**
  - macOS Apple Silicon (ARM64): `angkorfetch-macos-aarch64.tar.gz`
  - macOS Intel (x86_64): `angkorfetch-macos-x86_64.tar.gz`
  - Linux ARM64: `angkorfetch-linux-aarch64.tar.gz`
  - Linux x86_64: `angkorfetch-linux-x86_64.tar.gz`

### Audit Compliance
The generated formula is written to pass `brew audit --strict`:
- No explicit `version` stanza (Homebrew extracts version from URL).
- Uses `on_arm` and `on_intel` conditional blocks rather than `Hardware::CPU.arm?`.
- Template resides in `render_homebrew_formula()` in `scripts/sync_package_manifests.py`.

CI does not run `brew audit`. The `test` job in `release.yml` runs `python scripts/sync_package_manifests.py --check`, which fails if the committed formula is not what the generator produces. This formula installs the prebuilt release archives; the planned homebrew-core formula builds from source, so it will be a different formula.

### Validation & Testing
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

---

## 3. Fedora: DNF / RPM

### Metadata
- **Package Name:** `angkorfetch`
- **Spec File:** `linux/rpm/angkorfetch.spec`
- **Build Helper:** `linux/rpm/build-rpm.sh`
- **Artifact:** `angkorfetch-<version>-1.x86_64.rpm` (for example `angkorfetch-1.2.1-1.x86_64.rpm`; no dist suffix, because the `packages` job builds on Ubuntu, where `%{?dist}` is empty)
- **Status:** **Built**. The `.rpm` is attached to each GitHub Release. No Copr or Fedora repository serves it yet, so `dnf install angkorfetch` by name does not work.

### Building the RPM
The release `.rpm` does not use the files in `linux/rpm/`. On tag pushes, the `packages` job in `.github/workflows/release.yml` writes its own spec and packages the binary it just compiled with `rpmbuild`; the `release` job attaches the result to the release.

`linux/rpm/angkorfetch.spec` and `linux/rpm/build-rpm.sh` build from source, for Copr or a Fedora review. They are not run in CI. The helper needs `git`, `rpmbuild`, and the `cargo` RPM, and runs from any directory:

```bash
bash linux/rpm/build-rpm.sh
```

- It archives the whole repository at the committed `HEAD` (uncommitted changes are left out) into `~/rpmbuild/SOURCES/v<version>.tar.gz`, the file name at the end of the spec's `Source0` URL, under the top directory `angkorfetch-<version>/` that `%setup -q` expects. The archive equals the published `v<version>` source only when `HEAD` is that tag.
- A full `rpmbuild` run with these files has not been verified; only the archive step has been tested, with `rpmbuild` stubbed. The spec's `cargo build` downloads crates, so the build needs network access.
- Both files set the version by hand (`VERSION`, `Version:`); keep them in step with `Cargo.toml` (see [RELEASING.md](../RELEASING.md#cutting-a-release), step 1).

### Target Distribution Workflow
1. **Fedora Copr Integration (Recommended intermediary):**
   - Create the Copr project `amrskh/angkorfetch` (Copr web UI or `copr-cli create`).
   - Point build to GitHub source archive or spec file.
   - User command (once the Copr project exists):
     ```bash
     sudo dnf copr enable amrskh/angkorfetch
     sudo dnf install angkorfetch
     ```
2. **Official Fedora Submission:**
   - Submit package review request via Fedora Bugzilla.
   - Target install command once accepted:
     ```bash
     sudo dnf install angkorfetch
     ```

### Testing Commands
```bash
# Test local package install (file from the GitHub Release)
sudo dnf install ./angkorfetch-1.2.1-1.x86_64.rpm

# Upgrade: download the newer .rpm and install it the same way; dnf upgrades the installed package.
# `dnf upgrade angkorfetch` needs a repository (Copr or Fedora), which does not exist yet.

# Removal
sudo dnf remove angkorfetch
```

---

## 4. Ubuntu / Debian: APT / DEB

### Metadata
- **Package Name:** `angkorfetch`
- **Packaging Directory:** `linux/deb/`
- **Build Helper:** `linux/deb/build-deb.sh`
- **Artifact:** `angkorfetch_<version>_amd64.deb`
- **Status:** **Built**. The `.deb` is attached to each GitHub Release. No PPA or APT repository serves it yet, so `apt install angkorfetch` by name does not work.

### Building the Debian Package
Run from the repository root (the helper copies `target/release/angkorfetch` and `README.md` by relative path):

```bash
bash linux/deb/build-deb.sh
```

The helper sets `VERSION` by hand, so check it matches `Cargo.toml` before building. CI does not use it: on tag pushes, the `packages` job in `.github/workflows/release.yml` builds the release `.deb` with its own inline `control` file (`Section`, `Priority`, `Architecture: amd64`, `Maintainer`, `Description`) and the tag version.

### Target Distribution Workflow
1. **PPA / Open Build Service (OBS) APT Repository:**
   - Host repository with signed Release and Packages files.
   - User command (once the repository exists):
     ```bash
     # Repository configuration
     sudo add-apt-repository ppa:amrskh/angkorfetch   # or curl keyring setup
     sudo apt update

     # Native install
     sudo apt install angkorfetch
     ```
2. **Debian Official Archive:**
   - Package via Debian Mentors / RFS.

### Testing Commands
```bash
# Test local package install (file from the GitHub Release)
sudo apt install ./angkorfetch_1.2.1_amd64.deb

# Upgrade: download the newer .deb and install it the same way; apt upgrades the installed package.
# `apt upgrade` needs an APT repository, which does not exist yet.

# Removal
sudo apt remove angkorfetch
```

---

## 5. Arch Linux: Pacman / AUR

### Metadata
- **Package Name:** `angkorfetch`
- **PKGBUILD Location:** `linux/arch/PKGBUILD`
- **Build Helper:** `linux/arch/build-arch.sh`
- **Status:** **Planned** (PKGBUILD in `linux/arch/`; not yet on the AUR)

### Building with makepkg
```bash
cd linux/arch
makepkg -sfc
```

The PKGBUILD downloads the GitHub archive for `pkgver`, not the working tree.

### Target Distribution Workflow
1. **AUR (Arch User Repository):**
   - Set `pkgver`, replace `sha256sums=('SKIP')` with the archive's checksum (`updpkgsums`, from `pacman-contrib`), generate `.SRCINFO` with `makepkg --printsrcinfo > .SRCINFO`, and push both files to `ssh://aur@aur.archlinux.org/angkorfetch.git`. The AUR rejects a push without `.SRCINFO`.
   - Users install via AUR helpers (once on the AUR):
     ```bash
     yay -S angkorfetch
     # or
     paru -S angkorfetch
     ```
2. **Official Arch Repositories (Extra):**
   - Maintained by Arch Trusted Users once community adoption threshold is met.
   - Native command (once in the official repositories):
     ```bash
     sudo pacman -S angkorfetch
     ```

### Testing Commands
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

---

## 6. Universal Linux: Flatpak / Flathub

### Metadata
- **Application ID:** `io.github.AMRSKH.angkorfetch`
- **Manifest Location:** `flatpak/io.github.AMRSKH.angkorfetch.yml`
- **Metainfo Location:** `flatpak/io.github.AMRSKH.angkorfetch.metainfo.xml`
- **Base Runtime:** `org.freedesktop.Platform` 24.08 / `org.freedesktop.Sdk` 24.08
- **Finish Args:** `--share=network`, `--socket=x11`
- **Status:** **Planned** (manifest and AppStream metainfo in `flatpak/`; Flathub submission pending)

### Building Flatpak Locally
```bash
flatpak-builder --user --install --force-clean build-dir flatpak/io.github.AMRSKH.angkorfetch.yml
```

Not run in CI and not verified. The manifest declares no Rust SDK extension (`sdk-extensions`) and no vendored Cargo sources, and it builds the tag in its `sources` entry from GitHub, not the working tree.

### Flathub Publishing
1. Fork `https://github.com/flathub/flathub`.
2. Open a pull request against the `new-pr` branch adding `io.github.AMRSKH.angkorfetch` (manifest and metainfo).
3. Target install command once published:
   ```bash
   flatpak install flathub io.github.AMRSKH.angkorfetch
   flatpak run io.github.AMRSKH.angkorfetch
   ```

### Testing Commands
```bash
flatpak update io.github.AMRSKH.angkorfetch
flatpak uninstall io.github.AMRSKH.angkorfetch
```

---

## 7. Universal Linux: Snap

### Metadata
- **Package Name:** `angkorfetch`
- **Manifest Location:** `snap/snapcraft.yaml`
- **Confinement:** `strict`
- **Base:** `core22`
- **Status:** **Planned** (Manifest maintained in repo; Snap Store registration pending)

### Building Snap
```bash
snapcraft --use-lxd
```

`snapcraft.yaml` builds the tag in `source-tag` from GitHub, not the working tree.

---

## 8. Any OS: crates.io

### Metadata
- **Crate:** [`angkorfetch`](https://crates.io/crates/angkorfetch)
- **Install Command:** `cargo install angkorfetch` (builds from source, installs to `~/.cargo/bin`)
- **Status:** **Live**. The latest published version is 1.2.1 (published 2026-11-11), matching the v1.2.1 GitHub release (checked 2026-11-11). Publishing is manual (see below), so a later release stays ahead of crates.io until it is published.

### Publishing
No workflow publishes the crate: `release.yml` has no `cargo publish` step. Publishing is a manual step from the tagged commit and needs maintainer approval (see [AGENTS.md](../AGENTS.md), section 7).
