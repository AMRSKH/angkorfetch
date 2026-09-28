# AngkorFetch Development Guide

This guide covers building, testing, linting, and contributing to the AngkorFetch core codebase.

## Prerequisites

To develop AngkorFetch locally, you will need:

* **Rust stable** (edition 2021): Install via [rustup.rs](https://rustup.rs/)
* **Cargo**: Included with standard Rust toolchain
* **Git**: Required for version control
* **Python 3.x**: Required for running the package manifest sync tests and verification checks

## Project Structure

```text
angkorfetch/
├── src/
│   └── main.rs                  # Core CLI logic, OS detection, hardware queries, banner rendering
├── Cargo.toml                   # Project metadata and release profile configuration
├── Cargo.lock                   # Pinned dependency graph
├── Formula/
│   └── angkorfetch.rb           # Homebrew formula (generated from template)
├── flatpak/
│   ├── io.github.AMRSKH.angkorfetch.yml           # Flatpak build manifest
│   └── io.github.AMRSKH.angkorfetch.metainfo.xml  # AppStream metainfo
├── linux/
│   ├── arch/
│   │   ├── PKGBUILD             # Arch Linux / AUR package specification
│   │   └── build-arch.sh        # Arch Linux makepkg helper
│   ├── deb/
│   │   └── build-deb.sh         # Debian/Ubuntu .deb package generation script
│   └── rpm/
│       ├── angkorfetch.spec     # RPM package specification
│       └── build-rpm.sh         # Fedora/RHEL .rpm build helper
├── snap/
│   └── snapcraft.yaml           # Snapcraft manifest
├── winget-pkgs/
│   └── manifests/a/AMRSKH/AngkorFetch/
│       └── <version>/           # WinGet YAML manifests (version, installer, locale)
├── scripts/
│   ├── sync_package_manifests.py       # Syncs package checksums and manifests post-release
│   └── test_sync_package_manifests.py  # Unit test suite for manifest automation
├── docs/
│   ├── DEVELOPMENT.md           # This development guide
│   ├── PACKAGING.md             # Package management & distribution guide
│   ├── RELEASING.md             # Release procedures and CI architecture
│   └── MAINTAINING.md           # Repository owner operational guide
├── CONTRIBUTING.md               # External contributor guidelines
└── README.md                    # Primary user-facing documentation
```

## Build

### Debug Build
```bash
cargo build
```
The binary will be located at `target/debug/angkorfetch` (`target/debug/angkorfetch.exe` on Windows).

### Optimized Release Build
```bash
cargo build --release
```
The optimized binary will be located at `target/release/angkorfetch` (`target/release/angkorfetch.exe` on Windows).
The release profile activates:
- `opt-level = 3` (maximum speed)
- `lto = true` (link-time optimization across all crates)
- `codegen-units = 1` (maximum whole-program optimization)
- `strip = true` (removes debug symbols to minimize binary size)

## Testing

AngkorFetch maintains automated unit tests covering logo layout, ANSI color gradient calculations, terminal width wrapping, and packaging synchronizations.

### Run Rust Test Suite
```bash
cargo test --locked
```

### Run Package Manifest Automation Tests
```bash
python -m unittest discover -s scripts -p "test_*.py"
```

### Validate Homebrew Formula Shape
```bash
python scripts/sync_package_manifests.py --check
```

## Formatting & Code Quality

Before opening a pull request, verify that the code satisfies formatting and static analysis rules:

### Code Formatting
```bash
cargo fmt --check
```
To reformat automatically:
```bash
cargo fmt
```

### Clippy Linting
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Platform-Specific Architecture

AngkorFetch reads native OS APIs directly rather than querying external shells or interpreters whenever possible:

1. **Windows**:
   - Query OS metrics via Windows Registry (`SOFTWARE\Microsoft\Windows NT\CurrentVersion`).
   - Query hardware, motherboard, BIOS, and GPU through Windows CIM / WMI calls (`Get-CimInstance`).
   - Screen resolution determined via GDI `GetDeviceCaps`.
   - Power state determined via Win32 `GetSystemPowerStatus`.

2. **Linux**:
   - Query `/sys/class/dmi/id/` for BIOS, product name, board vendor, and chassis data.
   - Query `/sys/class/power_supply/` for battery status and capacity.
   - Query `/sys/block/` for physical disks and NVMe/SATA models.
   - Display resolution queried via `xrandr` (X11).
   - Packages queried across native package managers (`dpkg`, `rpm`, `pacman`, `apk`, `flatpak`, `snap`).

3. **macOS**:
   - Query sysctl (`hw.model`, `machdep.cpu.brand_string`).
   - Query `system_profiler` for graphics display, memory topology, and hardware IDs.
   - Query `pmset` and `ioreg` for battery cycles, power status, and Wi-Fi interface.

## Core Design Principles

- **Zero runtime overhead**: Only 3 dependencies allowed: `sysinfo`, `colored`, and `terminal_size`. Do not add heavy crates.
- **Graceful degradation**: Never crash or panic on missing hardware or permission denials. If a field cannot be determined, output `Unknown` or `N/A`.
- **Preserve terminal adaptability**: Maintain the adaptive banner logic that selects 24-bit truecolor or 16-color gradients and adjusts ASCII width based on terminal size.
