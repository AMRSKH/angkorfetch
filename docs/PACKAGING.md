# AngkorFetch Packaging and Distribution Guide

This document defines package specifications, manifest paths, validation workflows, and distribution channels for every supported platform.

## Packaging Overview & Status

| Ecosystem | Target Package ID | Native Install Command | Current Status | Distribution Channel |
| --- | --- | --- | --- | --- |
| **WinGet** | `AMRSKH.AngkorFetch` | `winget install angkorfetch` | **Pending** | `microsoft/winget-pkgs` (PR #409790) |
| **Homebrew** | `angkorfetch` | `brew install angkorfetch` | **Published (Tap)** | `AMRSKH/homebrew-tap` (Core planned) |
| **Fedora (DNF)** | `angkorfetch` | `sudo dnf install angkorfetch` | **Built** | GitHub Release RPM (Copr / Fedora repo planned) |
| **Debian/Ubuntu (APT)** | `angkorfetch` | `sudo apt install angkorfetch` | **Built** | GitHub Release DEB (PPA / APT repo planned) |
| **Arch Linux (Pacman)** | `angkorfetch` | `sudo pacman -S angkorfetch` | **Planned** | PKGBUILD ready (AUR / Community planned) |
| **Flatpak** | `io.github.AMRSKH.angkorfetch` | `flatpak install flathub io.github.AMRSKH.angkorfetch` | **Planned** | Flathub manifest ready |
| **Snap** | `angkorfetch` | `sudo snap install angkorfetch` | **Planned** | Snapcraft manifest ready |

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
- **Status:** **Pending** community PR approval at [microsoft/winget-pkgs#409790](https://github.com/microsoft/winget-pkgs/pull/409790)

### Manifest Structure
1. `AngkorFetch.yaml`: Version manifest referencing `en-US` locale.
2. `AngkorFetch.installer.yaml`: Points to zip release binary with matching SHA256.
3. `AngkorFetch.locale.en-US.yaml`: Metadata, description, tags, publisher URL, and moniker.

### Validation & Testing
```powershell
# Validate manifest schema
winget validate --manifest winget-pkgs/manifests/a/AMRSKH/AngkorFetch/1.1.1

# Test local manifest install
winget install --manifest winget-pkgs/manifests/a/AMRSKH/AngkorFetch/1.1.1

# Verification commands once published:
winget search angkorfetch
winget show AMRSKH.AngkorFetch
winget install angkorfetch
winget upgrade angkorfetch
winget uninstall angkorfetch
```

### Automation & Publishing
WinGet manifests are synchronized automatically by `.github/workflows/sync-packages.yml` via `scripts/sync_package_manifests.py`. Maintainers submit updates to `microsoft/winget-pkgs` via `wingetcreate`:
```powershell
wingetcreate update AMRSKH.AngkorFetch -u https://github.com/AMRSKH/angkorfetch/releases/download/v1.1.1/angkorfetch-windows-x86_64.zip
```

---

## 2. macOS: Homebrew

### Metadata
- **Package Name:** `angkorfetch`
- **Formula Path:** `Formula/angkorfetch.rb`
- **Canonical Tap:** `AMRSKH/homebrew-tap`
- **Direct Tap URL:** `https://github.com/AMRSKH/homebrew-tap`
- **Status:** **Published (Tap)** (`brew install AMRSKH/tap/angkorfetch`); Homebrew Core submission is planned.
- **Architectures:**
  - macOS Apple Silicon (ARM64): `angkorfetch-macos-aarch64.tar.gz`
  - macOS Intel (x86_64): `angkorfetch-macos-x86_64.tar.gz`
  - Linux ARM64: `angkorfetch-linux-aarch64.tar.gz`
  - Linux x86_64: `angkorfetch-linux-x86_64.tar.gz`

### Audit Compliance
The formula is strictly audited against `brew audit --strict`:
- No explicit `version` stanza (Homebrew extracts version from URL).
- Uses `on_arm` and `on_intel` conditional blocks rather than `Hardware::CPU.arm?`.
- Template resides in `render_homebrew_formula()` in `scripts/sync_package_manifests.py`.

### Validation & Testing
```bash
# Verify formula syntax and audit compliance
brew audit --strict Formula/angkorfetch.rb

# Validate local installation
brew install --build-from-source Formula/angkorfetch.rb

# Verification commands once published:
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
- **Artifact:** `angkorfetch-<version>-1.x86_64.rpm`
- **Status:** **Built** (Artifact attached in GitHub Release; Fedora Copr / official repository setup pending)

### Building the RPM
```bash
bash linux/rpm/build-rpm.sh
```
Inside GitHub Actions, RPM packages are built automatically on tag pushes using `rpmbuild` and attached to the release.

### Target Distribution Workflow
1. **Fedora Copr Integration (Recommended intermediary):**
   - Create Copr repository: `copr create amrskh/angkorfetch`
   - Point build to GitHub source archive or spec file.
   - User command:
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
# Test local package install
sudo dnf install ./angkorfetch-1.1.1-1.x86_64.rpm

# Upgrade / Removal
sudo dnf upgrade angkorfetch
sudo dnf remove angkorfetch
```

---

## 4. Ubuntu / Debian: APT / DEB

### Metadata
- **Package Name:** `angkorfetch`
- **Packaging Directory:** `linux/deb/`
- **Build Helper:** `linux/deb/build-deb.sh`
- **Artifact:** `angkorfetch_<version>_amd64.deb`
- **Status:** **Built** (Artifact attached in GitHub Release; PPA / APT repository distribution pending)

### Building the Debian Package
```bash
bash linux/deb/build-deb.sh
```
Inside GitHub Actions, the Debian package is created on tag pushes with correct control metadata (`Priority`, `Section`, `Architecture`, `Maintainer`, `Description`).

### Target Distribution Workflow
1. **PPA / Open Build Service (OBS) APT Repository:**
   - Host repository with signed Release and Packages files.
   - User command:
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
# Test local package install
sudo apt install ./angkorfetch_1.1.1_amd64.deb

# Upgrade / Removal
sudo apt update && sudo apt upgrade angkorfetch
sudo apt remove angkorfetch
```

---

## 5. Arch Linux: Pacman / AUR

### Metadata
- **Package Name:** `angkorfetch`
- **PKGBUILD Location:** `linux/arch/PKGBUILD`
- **Build Helper:** `linux/arch/build-arch.sh`
- **Status:** **Planned** (PKGBUILD ready; AUR submission pending)

### Building with makepkg
```bash
cd linux/arch
makepkg -sfc
```

### Target Distribution Workflow
1. **AUR (Arch User Repository):**
   - Push PKGBUILD to `ssh://aur@aur.archlinux.org/angkorfetch.git`.
   - Users install via AUR helpers:
     ```bash
     yay -S angkorfetch
     # or
     paru -S angkorfetch
     ```
2. **Official Arch Repositories (Extra):**
   - Maintained by Arch Trusted Users once community adoption threshold is met.
   - Native command:
     ```bash
     sudo pacman -S angkorfetch
     ```

### Testing Commands
```bash
# Inspect package with namcap
namcap linux/arch/PKGBUILD

# Upgrade / Removal
sudo pacman -Syu angkorfetch
sudo pacman -R angkorfetch
```

---

## 6. Universal Linux: Flatpak / Flathub

### Metadata
- **Application ID:** `io.github.AMRSKH.angkorfetch`
- **Manifest Location:** `flatpak/io.github.AMRSKH.angkorfetch.yml`
- **Metainfo Location:** `flatpak/io.github.AMRSKH.angkorfetch.metainfo.xml`
- **Base Runtime:** `org.freedesktop.Platform` 24.08 / `org.freedesktop.Sdk` 24.08
- **Finish Args:** `--share=network`, `--socket=x11`
- **Status:** **Planned** (Manifest & AppStream metainfo ready; Flathub submission pending)

### Building Flatpak Locally
```bash
flatpak-builder --user --install --force-clean build-dir flatpak/io.github.AMRSKH.angkorfetch.yml
```

### Flathub Publishing
1. Fork `https://github.com/flathub/flathub`.
2. Submit pull request adding `io.github.AMRSKH.angkorfetch`.
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
