# Releasing AngkorFetch

This document outlines the release engineering and CI/CD architecture for AngkorFetch. For repository owner operational tasks, refer to [MAINTAINING.md](MAINTAINING.md). For the complete release engineering deep dive and edge case reference, see [../RELEASING.md](../RELEASING.md).

## Release Pipeline Architecture

```text
 tag vX.Y.Z on main
         │
         ▼
 ┌──────────────────────────── release.yml ────────────────────────────┐
 │  test ──► build (5 targets) ──► packages ──► checksums              │
 │                                     └──────────► release            │
 │                                                     │               │
 │                                                     ▼               │
 │                          sync_packages (calls sync-packages.yml)    │
 └─────────────────────────────────────────────────────────────────────┘
                                                       │
                                                       ▼
                          reads checksums.txt from the published release
                          rewrites Homebrew + winget, opens a pull request
```

## Published Release Assets

Every release produces 8 verifiable artifacts:

1. `angkorfetch-linux-x86_64.tar.gz`: Linux x86_64 static binary
2. `angkorfetch-linux-aarch64.tar.gz`: Linux ARM64 binary (cross-compiled)
3. `angkorfetch-macos-x86_64.tar.gz`: macOS Intel binary
4. `angkorfetch-macos-aarch64.tar.gz`: macOS Apple Silicon binary
5. `angkorfetch-windows-x86_64.zip`: Windows x86_64 executable
6. `angkorfetch_<version>_amd64.deb`: Debian/Ubuntu deb package
7. `angkorfetch-<version>-1.x86_64.rpm`: Fedora/RHEL rpm package
8. `checksums.txt`: SHA256 checksums of all 7 binary artifacts

## Workflows Reference

- [`.github/workflows/release.yml`](../.github/workflows/release.yml): Compiles all platform targets, builds deb/rpm packages, computes checksums, verifies all assets are present, creates GitHub release, and invokes package sync.
- [`.github/workflows/sync-packages.yml`](../.github/workflows/sync-packages.yml): Reusable workflow triggered after release. Downloads `checksums.txt`, runs `scripts/sync_package_manifests.py`, updates `Formula/angkorfetch.rb` and `winget-pkgs/`, commits to branch `automation/sync-packages-vX.Y.Z`, and opens a pull request.
- [`scripts/sync_package_manifests.py`](../scripts/sync_package_manifests.py): Deterministic generator/patcher for Homebrew formula and WinGet manifests. Preserves line endings, enforces `brew audit --strict` standards, and fails on missing artifacts.
