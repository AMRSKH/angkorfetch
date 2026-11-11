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
CI builds 6 targets + .deb + .rpm + checksums.txt
         │
         ▼
sync-packages.yml opens automation PR for Homebrew & WinGet
         │
         ▼
Review & Merge automation PR
         │
         ▼
Submit / Verify external packages (crates.io; once each is published: WinGet, homebrew-core, Copr, AUR, Flathub)
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
1. `test`: Checks formatting on Ubuntu, then runs Clippy and the tests on Ubuntu, Windows, and macOS.
2. `build`: Compiles 6 release targets (Linux x86_64, Linux x86_64 musl, Linux ARM64, macOS Intel, macOS Apple Silicon, Windows x64).
3. `packages`: Generates `.deb` and `.rpm` packages.
4. `checksums`: Generates `checksums.txt` containing SHA256 hashes for the 8 other assets.
5. `release`: Creates the GitHub Release and attaches all 9 assets.
6. `sync_packages`: Reusable workflow calls `sync-packages.yml` to download `checksums.txt`, patch Homebrew formula and WinGet manifests, and open a PR: `automation/sync-packages-vX.Y.Z`.

### Step 5: Review and Merge Manifest PR
Review the auto-generated pull request opened by `sync-packages.yml`. Verify:
- All SHA256 hashes match `checksums.txt` in the release.
- Formula shape matches generator.
- Merge the PR into `main`.

---

## 4. External Package Manager Submissions

Channel statuses are in the [Package Status table in the README](../README.md#package-status--availability). Of the channels below, only crates.io is published today. The `.deb` and `.rpm` files need no step here, because each release attaches them. Every other step applies once its channel is published; first submissions follow [PACKAGING.md](PACKAGING.md).

### WinGet (Windows Package Manager)
**Status: Pending.** The package is not in `microsoft/winget-pkgs` yet, so the next submission is a new-package pull request: follow [PACKAGING.md](PACKAGING.md), section 1. Step 2 applies only after the package has been merged.

1. Ensure the manifest under `winget-pkgs/manifests/a/AMRSKH/AngkorFetch/<version>/` is updated.
2. Once the package is in `microsoft/winget-pkgs`, submit each later version via `wingetcreate`:
   ```powershell
   wingetcreate update AMRSKH.AngkorFetch --version X.Y.Z --urls https://github.com/AMRSKH/angkorfetch/releases/download/vX.Y.Z/angkorfetch-windows-x86_64.zip --submit
   ```
3. Monitor pull request at `https://github.com/microsoft/winget-pkgs/pulls`.

### Homebrew (macOS / Linux)
**Status: Planned (homebrew-core).** There is no Homebrew release step yet. The planned formula builds from source and goes to `Homebrew/homebrew-core`, so it is not the generated `Formula/angkorfetch.rb`. See [PACKAGING.md](PACKAGING.md), section 2.

`AMRSKH/homebrew-tap` still exists, but it is not the documented install path, CI does not update it, and updating it is not part of a release.

### Fedora Copr (DNF)
**Status: Built.** The `.rpm` is attached to each GitHub Release, and no Copr project serves it yet. To create the project, follow [PACKAGING.md](PACKAGING.md), section 3. Once it exists, trigger a Copr build for each release. The exact `copr-cli` command is not given here because it has not been verified; check `copr-cli --help` first.

### Arch Linux (AUR)
**Status: Planned.** Not yet on the AUR; the first submission follows [PACKAGING.md](PACKAGING.md), section 5. The steps below apply to each release after that.

1. Update `linux/arch/PKGBUILD` with new `pkgver` and SHA256.
2. Generate `.SRCINFO`:
   ```bash
   makepkg --printsrcinfo > .SRCINFO
   ```
3. Commit and push to AUR git repository.

### Flathub
**Status: Planned.** Not yet on Flathub; the first submission follows [PACKAGING.md](PACKAGING.md), section 6. The step below applies to each release after that.

1. Push updated manifest `flatpak/io.github.AMRSKH.angkorfetch.yml` to the Flathub app repository.

### crates.io (Cargo)
**Status: Live.** No workflow publishes the crate, so each release needs a manual publish from the tagged commit, with maintainer approval and a crates.io token. The published version can lag the release; see [PACKAGING.md](PACKAGING.md), section 8.
```bash
git switch --detach vX.Y.Z
cargo publish --locked
```

---

## 5. Automation Permissions Setup

To ensure `sync-packages.yml` can automatically open pull requests without permission errors:
- In GitHub: **Repository Settings → Actions → General → Workflow permissions**
- Check **"Allow GitHub Actions to create and approve pull requests"**
- Alternatively, add repository secret `PACKAGE_SYNC_TOKEN` with a Personal Access Token (`contents: write`, `pull-requests: write`).
