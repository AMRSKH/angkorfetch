# AngkorFetch Maintainer Operations Guide

This guide is designed for the repository owner and core maintainers (**AMRSKH**). It outlines daily operational tasks, branch lifecycle, release procedures, and external package registry synchronization.

> [!NOTE]
> As the repository owner working directly within the repository environment, you do not need to clone the repository or fork. Work directly on feature branches or `dev` and merge into `main`.

---

## 1. Branch Strategy

| Branch | Purpose | Protection |
| --- | --- | --- |
| `main` | Production branch. Releases are tagged exclusively from here. All external scripts (`get.sh`, `get.ps1`) pull from `main`. | Protected. Only fast-forward or clean PR merges. |
| `dev` | Integration branch. Features and bug fixes land here first. | Automated CI builds and tests. |
| `feat/*`, `fix/*` | Short-lived topic branches for development. | Merged into `dev` or `main`. |
| `automation/sync-packages-*` | Created automatically by CI post-release to sync manifests. | Review and merge into `main`. |

### Keeping `dev` Synchronized
After merging any pull request or tag to `main`, fast-forward `dev`:
```bash
git switch dev
git merge --ff-only main
git push origin dev
```

---

## 2. Standard Development Cycle

```text
Make change on dev / topic branch
         │
         ▼
Run local validations (tests, fmt, clippy)
         │
         ▼
Merge to main (via PR or direct merge when tests pass)
         │
         ▼
Tag release vX.Y.Z
         │
         ▼
CI builds 5 targets + .deb + .rpm + checksums.txt
         │
         ▼
sync-packages.yml opens automation PR for Homebrew & WinGet
         │
         ▼
Review & Merge automation PR
         │
         ▼
Submit / Verify external packages (Winget, Homebrew tap, Copr, AUR, Flathub)
         │
         ▼
Update documentation status tables
```

---

## 3. Cutting a New Release (Step-by-Step)

### Step 1: Version Bump
Before tagging, bump the version string across package manifests:
1. `Cargo.toml`: Update `version = "X.Y.Z"`
2. `Cargo.lock`: Run `cargo check` or `cargo build` to refresh lockfile
3. `linux/rpm/angkorfetch.spec`: Update `Version:` and add a `%changelog` entry
4. `linux/rpm/build-rpm.sh`: Update `VERSION="X.Y.Z"`
5. `linux/deb/build-deb.sh`: Update `VERSION="X.Y.Z"`
6. `flatpak/io.github.AMRSKH.angkorfetch.yml`: Update tag `vX.Y.Z`
7. `flatpak/io.github.AMRSKH.angkorfetch.metainfo.xml`: Add `<release version="X.Y.Z" ... />` entry
8. `snap/snapcraft.yaml`: Update `version` and `source-tag`

> [!IMPORTANT]
> Do **not** manually update `Formula/angkorfetch.rb` or `winget-pkgs/` at this stage. Those files pin SHA256 hashes of release artifacts that do not exist until the release is built and uploaded.

### Step 2: Validate Locally
Run the test suites:
```bash
cargo test --locked
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
python -m unittest discover -s scripts -p "test_*.py"
python scripts/sync_package_manifests.py --check
```

### Step 3: Commit and Tag
Commit the version bump to `main`:
```bash
git add -A
git commit -m "chore(release): prepare vX.Y.Z"
git push origin main

# Tag and push release
git tag -a vX.Y.Z -m "AngkorFetch vX.Y.Z"
git push origin vX.Y.Z
```

### Step 4: GitHub Actions Release Pipeline
Pushing the `vX.Y.Z` tag automatically starts `.github/workflows/release.yml`:
1. `test`: Runs tests across Ubuntu, Windows, and macOS.
2. `build`: Compiles 5 release targets (Linux x86_64, Linux ARM64, macOS Intel, macOS Apple Silicon, Windows x64).
3. `packages`: Generates `.deb` and `.rpm` packages.
4. `checksums`: Generates `checksums.txt` containing SHA256 hashes for all 7 assets.
5. `release`: Creates the GitHub Release and attaches all 8 assets.
6. `sync_packages`: Reusable workflow calls `sync-packages.yml` to download `checksums.txt`, patch Homebrew formula and WinGet manifests, and open a PR: `automation/sync-packages-vX.Y.Z`.

### Step 5: Review and Merge Manifest PR
Review the auto-generated pull request opened by `sync-packages.yml`. Verify:
- All SHA256 hashes match `checksums.txt` in the release.
- Formula shape matches generator.
- Merge the PR into `main`.

---

## 4. External Package Manager Submissions

### WinGet (Windows Package Manager)
1. Ensure the manifest under `winget-pkgs/manifests/a/AMRSKH/AngkorFetch/<version>/` is updated.
2. Submit to `microsoft/winget-pkgs` via `wingetcreate`:
   ```powershell
   wingetcreate update AMRSKH.AngkorFetch --version X.Y.Z -u https://github.com/AMRSKH/angkorfetch/releases/download/vX.Y.Z/angkorfetch-windows-x86_64.zip
   ```
3. Monitor pull request at `https://github.com/microsoft/winget-pkgs/pulls`.

### Homebrew (macOS / Linux)
1. The tap at `AMRSKH/homebrew-tap` is updated by copying `Formula/angkorfetch.rb` to the tap repo.
2. Once Homebrew Core criteria are met, submit formula to `Homebrew/homebrew-core`.

### Fedora Copr (DNF)
1. Trigger build in Copr:
   ```bash
   copr-cli build-package amrskh/angkorfetch --clone-url https://github.com/AMRSKH/angkorfetch.git
   ```

### Arch Linux (AUR)
1. Update `linux/arch/PKGBUILD` with new `pkgver` and SHA256.
2. Generate `.SRCINFO`:
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```
3. Commit and push to AUR git repository.

### Flathub
1. Push updated manifest `flatpak/io.github.AMRSKH.angkorfetch.yml` to the Flathub app repository.

---

## 5. Automation Permissions Setup

To ensure `sync-packages.yml` can automatically open pull requests without permission errors:
- In GitHub: **Repository Settings → Actions → General → Workflow permissions**
- Check **"Allow GitHub Actions to create and approve pull requests"**
- Alternatively, add repository secret `PACKAGE_SYNC_TOKEN` with a Personal Access Token (`contents: write`, `pull-requests: write`).
