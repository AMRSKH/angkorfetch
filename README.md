# AngkorFetch

[![crates.io](https://img.shields.io/crates/v/angkorfetch.svg)](https://crates.io/crates/angkorfetch)
[![build](https://github.com/AMRSKH/angkorfetch/actions/workflows/release.yml/badge.svg)](https://github.com/AMRSKH/angkorfetch/actions/workflows/release.yml)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**A fast, cross-platform system-info ("fetch") tool** written in Rust for Windows, macOS, and Linux.

---

## Features

* **Instant & lightweight**: Written in pure Rust with only three dependencies (`sysinfo`, `colored`, `terminal_size`). No Python, Node, or shell framework required.
* **Direct OS query**: Reads actual system metrics from Windows Registry & CIM/WMI, Linux `/sys` & DMI, and macOS `sysctl` & `system_profiler`. Nothing is guessed.
* **Graceful degradation**: Missing or restricted fields report `Unknown` or `N/A` cleanly without crashing or aborting the run.
* **Terminal-adaptive design**: Automatically detects terminal capabilities to select between full, compact, or minimal logos and 24-bit truecolor or 16-color gradients.

---

## Installation

> **Working methods listed first.** Methods marked **`[Pending]`** or **`[Planned]`** are not yet in the respective package repository — use the quick-install scripts or `cargo install` in the meantime.

### Quick install (recommended — works today, no Rust required)

#### Windows (PowerShell 5.1+)
```powershell
irm https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.ps1 | iex
```
*(Installs to `$env:LOCALAPPDATA\AngkorFetch\bin`. If `angkorfetch` is not recognised after install, open a new terminal window.)*

#### macOS & Linux (bash/zsh)
```bash
curl -fsSL https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.sh | bash
```
*(Installs to `~/.local/bin/angkorfetch`)*

### Via Cargo (any platform with Rust installed)
```bash
cargo install angkorfetch
```

### macOS — Homebrew **`[Planned: homebrew-core]`**
```bash
brew install angkorfetch
```
*(Planned for submission to homebrew-core built from source.)*

### Windows — WinGet **`[Pending]`**
```powershell
winget install AMRSKH.AngkorFetch
```
*(Not yet available: initial submission was rejected due to manifest file naming; will resubmit with corrected filenames `AMRSKH.AngkorFetch.*.yaml`.)*

### Fedora — DNF **`[Pending]`**
```bash
sudo dnf install angkorfetch
```
*(Copr repository not yet published. Use the `.rpm` from [GitHub Releases](https://github.com/AMRSKH/angkorfetch/releases/latest) — see Manual Install below.)*

### Ubuntu / Debian — APT **`[Pending]`**
```bash
sudo apt install angkorfetch
```
*(APT repository not yet published. Use the `.deb` from [GitHub Releases](https://github.com/AMRSKH/angkorfetch/releases/latest) — see Manual Install below.)*

### Arch Linux — AUR **`[Planned]`**
```bash
yay -S angkorfetch    # or: paru -S angkorfetch
```
*(AUR package not yet published. PKGBUILD is in `linux/arch/PKGBUILD`.)*

### Flatpak — Flathub **`[Planned]`**
```bash
flatpak install flathub io.github.AMRSKH.angkorfetch
```
*(Not yet submitted to Flathub. Manifest is in `flatpak/`.)*

---

## Package Status & Availability

| OS / Ecosystem | Package Identifier | Install Command | Status | Notes |
| --- | --- | --- | --- | --- |
| **Windows** | `AMRSKH.AngkorFetch` | `winget install AMRSKH.AngkorFetch` | **Pending** | Rejected due to manifest file naming; will resubmit |
| **macOS** | `angkorfetch` | `brew install angkorfetch` | **Planned (homebrew-core)** | Direct submission to homebrew-core planned |
| **Fedora** | `angkorfetch` | `sudo dnf install angkorfetch` | **Built** | `.rpm` package in Releases; Copr repo pending |
| **Ubuntu / Debian** | `angkorfetch` | `sudo apt install angkorfetch` | **Built** | `.deb` package in Releases; APT repo pending |
| **Arch Linux** | `angkorfetch` | `sudo pacman -S angkorfetch` | **Planned** | PKGBUILD ready in repo; AUR package planned |
| **Universal Linux** | `io.github.AMRSKH.angkorfetch` | `flatpak install flathub io.github.AMRSKH.angkorfetch` | **Planned** | Manifest & AppStream metainfo ready |
| **Any OS (Crates.io)**| `angkorfetch` | `cargo install angkorfetch` | **Live** | Multi-target source distribution |

---

## Quick Start

Run AngkorFetch with no arguments to print your system summary:

```bash
angkorfetch
```

---

## Usage

```bash
angkorfetch              # show basic system summary
angkorfetch -v           # print version information
angkorfetch --hinfo      # detailed hardware diagnostics (--hard also supported)
angkorfetch -h           # print help and options
```

---

## Options

| Flag | Long Flag | Description |
| --- | --- | --- |
| `-v` | `--version` | Display current version and build information |
| `-h` | `--help` | Display help menu and command options |
| | `--hinfo`, `--hard` | Display detailed hardware diagnostics (Motherboard, BIOS, Serial, RAM speed/type, Disk models, Ports, WiFi) |

---

## Example Output

Default system summary:

```text
  █████╗  ███╗   ██╗  ██████╗  ██╗  ██╗  ██████╗  ██████╗ 
 ██╔══██╗ ████╗  ██║ ██╔════╝  ██║ ██╔╝ ██╔═══██╗ ██╔══██╗
 ███████║ ██╔██╗ ██║ ██║  ███╗ █████╔╝  ██║   ██║ ██████╔╝
 ██╔══██║ ██║╚██╗██║ ██║   ██║ ██╔═██╗  ██║   ██║ ██╔══██╗
 ██║  ██║ ██║ ╚████║ ╚██████╔╝ ██║  ██╗ ╚██████╔╝ ██║  ██║
 ╚═╝  ╚═╝ ╚═╝  ╚═══╝  ╚═════╝  ╚═╝  ╚═╝  ╚═════╝  ╚═╝  ╚═╝

╔═══════════════════════════════════════════════════════════════════════╗
║ AngkorFetch v1.1.1  •  Fast Cross-Platform System Fetch  •  by AMSDev ║
╚═══════════════════════════════════════════════════════════════════════╝

 ● OS         ❯ Windows 11 Pro - 25H2 [x86_64]
 ● Host       ❯ DELL
 ● Model      ❯ Dell Inc. Latitude 5490
 ● Uptime     ❯ 1d 6h 29m
 ● CPU        ❯ Intel® Core™ i5-8250U @ 1.60GHz (8 cores) @ 1.60 GHz
 ● CPU Usage  ❯ 24.1%
 ● GPU        ❯ Intel(R) UHD Graphics 620
 ● GPU Usage  ❯ N/A
 ● Memory     ❯ 8.0 GiB / 15.9 GiB (51%)
 ● Disk       ❯ 113.4 GiB / 255.0 GiB (44%)
 ● Display    ❯ 1920x1080 @ 60Hz
 ● Shell      ❯ PowerShell
 ● Terminal   ❯ Windows Terminal
 ● DE         ❯ Windows Explorer
 ● Packages   ❯ 83(winget), 44(apps)
 ● Battery    ❯ 63% [Discharging]
 ● Local IP   ❯ 192.168.0.208 (Wi-Fi)
```

Detailed hardware output (`angkorfetch --hinfo`):

```text
 ● Motherboard  ❯ Dell Inc. 08NJ82
 ● BIOS         ❯ Dell Inc. 1.41.0
 ● Serial       ❯ GXCGRV2
 ● CPU          ❯ Intel® Core™ i5-8250U @ 1.60GHz (8 cores) @ 1.60 GHz
 ● GPU          ❯ Intel(R) UHD Graphics 620
 ● Memory       ❯ 8.0 GiB / 15.9 GiB (51%)
 ● RAM          ❯ DDR4 @ 2667 MHz (0080000080AD)
 ● Disk         ❯ 113.4 GiB / 255.0 GiB (44%)
 ● Disk Model   ❯ PM981 NVMe Samsung 256GB
 ● Disk Type    ❯ NVMe SSD
 ● Display      ❯ 1920x1080 @ 60Hz
 ● Ports        ❯ USB x25, Video Out x1, Audio x2
 ● WiFi         ❯ TP-Link_58AC_5G (83%)
 ● Battery      ❯ 63% [Discharging]
```

---

## Supported Platforms

Support is strictly categorized into **Native Packages**, **Prebuilt Binaries**, and **Source Builds**:

| Operating System | Architecture | Prebuilt Binary | Native Package | Implementation Details |
| --- | --- | --- | --- | --- |
| **Windows 10 & 11** | x86_64 | Yes | WinGet (Pending) | Full native support (Registry, CIM/WMI, GDI) |
| **Linux (glibc)** | x86_64 | Yes | DEB, RPM, Flatpak, Arch | Full native support (`/sys`, DMI, lspci) |
| **Linux (glibc)** | aarch64 | Yes | Arch, Flatpak, Homebrew | Full native support (`/sys`, DMI) |
| **macOS (Intel)** | x86_64 | Yes | Homebrew | Full native support (sysctl, system_profiler) |
| **macOS (Apple Silicon)** | aarch64 | Yes | Homebrew | Full native support (sysctl, system_profiler) |
| **Linux (musl / Alpine)** | Any | No | No | Build from source via `cargo install` |
| **Windows on ARM** | aarch64 | Emulated x64 | No | Native build from source via `cargo install` |
| **BSD (FreeBSD, OpenBSD)**| Any | No | No | Fallback sysinfo metrics only |

---

## Update

Update AngkorFetch using your system's package manager:

```powershell
# Windows (WinGet)
winget upgrade angkorfetch
```

```bash
# macOS (Homebrew)
brew upgrade angkorfetch

# Fedora (DNF)
sudo dnf upgrade angkorfetch

# Ubuntu / Debian (APT)
sudo apt update && sudo apt upgrade angkorfetch

# Arch Linux (Pacman)
sudo pacman -Syu angkorfetch

# Flatpak
flatpak update io.github.AMRSKH.angkorfetch
```

---

## Uninstall

Remove AngkorFetch cleanly via your package manager:

```powershell
# Windows (WinGet)
winget uninstall angkorfetch
```

```bash
# macOS (Homebrew)
brew uninstall angkorfetch

# Fedora (DNF)
sudo dnf remove angkorfetch

# Ubuntu / Debian (APT)
sudo apt remove angkorfetch

# Arch Linux (Pacman)
sudo pacman -R angkorfetch

# Flatpak
flatpak uninstall io.github.AMRSKH.angkorfetch
```

---

## Troubleshooting

* **Linux Root Privileges**: On Linux, hardware serial numbers and RAM vendor/speed metrics require root privileges to read `/sys/class/dmi` and `dmidecode`. Run with `sudo` for full diagnostics:
  ```bash
  sudo angkorfetch --hinfo
  ```
* **Wayland Displays**: Screen resolution detection relies on `xrandr`. Under pure Wayland sessions without XWayland, the Display field reports `Unknown`.
* **WSL (Windows Subsystem for Linux)**: WSL operates in a virtualized container. Host-level hardware fields (Motherboard BIOS, Serial, Battery) will report `Unknown`.
* **Confinement**: Strict sandbox environments (such as Snap confinement) may restrict access to `/sys/class/dmi`.

---

## Manual Installation

If your platform does not yet have a published package manager repository, use one of the following fallback installation methods:

### Direct GitHub Release Packages

Download the standalone package or archive directly from [GitHub Releases](https://github.com/AMRSKH/angkorfetch/releases/latest):

```bash
# The filenames include the version number; use this snippet to resolve it automatically.

# Debian / Ubuntu (.deb)
VER=$(curl -fsSL https://github.com/AMRSKH/angkorfetch/releases/latest \
      | grep -o 'tag/v[^"]*' | head -1 | cut -d/ -f2)
DEB="angkorfetch_${VER#v}_amd64.deb"
curl -fsSL "https://github.com/AMRSKH/angkorfetch/releases/latest/download/${DEB}" -o "$DEB"
sudo apt install "./$DEB"

# Fedora / RHEL (.rpm)
VER=$(curl -fsSL https://github.com/AMRSKH/angkorfetch/releases/latest \
      | grep -o 'tag/v[^"]*' | head -1 | cut -d/ -f2)
RPM="angkorfetch-${VER#v}-1.x86_64.rpm"
sudo dnf install "https://github.com/AMRSKH/angkorfetch/releases/latest/download/${RPM}"
```

> **Tip**: Or simply visit [GitHub Releases](https://github.com/AMRSKH/angkorfetch/releases/latest)
> and download the file that matches your distro directly.

### Automated Quick Install Scripts

#### Windows (PowerShell)
```powershell
irm https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.ps1 | iex
```
*(Installs executable to `$env:LOCALAPPDATA\AngkorFetch\bin`)*

#### macOS & Linux (Shell)
```bash
curl -fsSL https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.sh | bash
```
*(Installs executable to `~/.local/bin/angkorfetch`)*

### Via Cargo (Any Platform with Rust)
```bash
cargo install angkorfetch
```

### Build From Source
```bash
git clone https://github.com/AMRSKH/angkorfetch.git
cd angkorfetch
cargo build --release
```
The compiled binary will be placed at `target/release/angkorfetch`.

---

## Development

For architecture overview, build instructions, and testing details, see [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md).

For package maintenance and distribution workflows, see [docs/PACKAGING.md](docs/PACKAGING.md) and [docs/RELEASING.md](docs/RELEASING.md).

Repository owner operational guide: [docs/MAINTAINING.md](docs/MAINTAINING.md).

---

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) for coding guidelines, test verification steps, and pull request procedures.

---

## License

This project is licensed under the [MIT License](LICENSE).
