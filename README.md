# AngkorFetch

[![crates.io](https://img.shields.io/crates/v/angkorfetch.svg)](https://crates.io/crates/angkorfetch)
[![build](https://github.com/AMRSKH/angkorfetch/actions/workflows/release.yml/badge.svg)](https://github.com/AMRSKH/angkorfetch/actions/workflows/release.yml)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**A fast, cross-platform system-info ("fetch") tool** written in Rust for Windows, macOS, and Linux.

---

## Features

* **Instant & lightweight**: Written in pure Rust with only three dependencies (`sysinfo`, `colored`, `terminal_size`). No Python, Node, or shell framework required.
* **Direct OS query**: Reads actual system metrics from Windows Registry & CIM/WMI, Linux `/sys` & DMI, and macOS `sysctl` & `system_profiler`.
* **Graceful degradation**: Missing or restricted fields report `Unknown` or `N/A` cleanly without crashing or aborting the run.
* **Terminal-adaptive design**: Picks a full, compact, or no logo to fit the terminal width, and a 24-bit truecolor or 16-color gradient depending on the terminal.

---

## Installation

> The quick-install scripts and `cargo install` work today. Package-manager channels are not published yet; their status is in [Package Status](#package-status--availability).

### Quick install (recommended — works today, no Rust required)

#### Windows (PowerShell 5.1+)
```powershell
irm https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.ps1 | iex
```
Downloads the latest release and checks its SHA256 against `checksums.txt`. Installs to `$env:LOCALAPPDATA\AngkorFetch\bin` and adds that folder to your **user** PATH. On ARM64 it installs the x86_64 build, which runs under emulation. If `angkorfetch` is not recognised, open a new terminal.

#### macOS & Linux (bash/zsh)
```bash
curl -fsSL https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.sh | bash
```
Same SHA256 check. Installs to `~/.local/bin/angkorfetch`; if that folder is not on PATH, the script appends an `export PATH=...` line to your shell profile. On musl x86_64 systems (for example Alpine) it picks the musl build automatically; install `bash`, `curl`, and `tar` first. There is no prebuilt binary for musl on aarch64, so the script stops and points to `cargo install angkorfetch`.

### Via Cargo (any platform with Rust installed)
```bash
cargo install angkorfetch
```
Builds from source and installs to `~/.cargo/bin`.

### Package managers

WinGet, Homebrew, AUR, Flathub, and Snap packages are **`[Pending]`** or **`[Planned]`**, and the APT and DNF repositories are not published yet. See [Package Status](#package-status--availability). Until they are live, use a quick-install script, `cargo install`, or the `.deb` / `.rpm` file from [Manual Installation](#manual-installation).

---

## Package Status & Availability

| OS / Ecosystem | Package Identifier | Install Command | Status | Notes |
| --- | --- | --- | --- | --- |
| **Any OS (Crates.io)** | `angkorfetch` | `cargo install angkorfetch` | **Live** | Builds from source |
| **Fedora / RHEL** | `angkorfetch` | `sudo dnf install ./angkorfetch-*.x86_64.rpm` | **Built** | `.rpm` (x86_64) in Releases; Copr repo pending |
| **Ubuntu / Debian** | `angkorfetch` | `sudo apt install ./angkorfetch_*_amd64.deb` | **Built** | `.deb` (amd64) in Releases; APT repo pending |
| **Windows** | `AMRSKH.AngkorFetch` | `winget install AMRSKH.AngkorFetch` | **Pending** | First submission rejected over manifest file names; files renamed, resubmission pending |
| **macOS** | `angkorfetch` | `brew install angkorfetch` | **Planned (homebrew-core)** | Direct submission to homebrew-core planned |
| **Arch Linux** | `angkorfetch` | `yay -S angkorfetch` | **Planned** | PKGBUILD in `linux/arch/`; not yet on the AUR |
| **Universal Linux** | `io.github.AMRSKH.angkorfetch` | `flatpak install flathub io.github.AMRSKH.angkorfetch` | **Planned** | Manifest and AppStream metainfo in `flatpak/` |
| **Universal Linux** | `angkorfetch` | `sudo snap install angkorfetch` | **Planned** | Manifest in `snap/`; Snap Store registration pending |

---

## Usage

Run with no arguments to print the system summary:

```bash
angkorfetch
```

| Flag | Long Flag | Description |
| --- | --- | --- |
| `-h` | `--help` | Print usage and the option list, then exit |
| `-v` | `--version` | Print the logo and version banner, then exit |
| | `--hinfo`, `--hard` | Print the logo, then the detailed hardware report: Motherboard, BIOS, Serial, CPU, GPU, Memory, RAM (type, speed, vendor), Disk, Disk Model, Disk Type, Display, Ports, WiFi, Battery |

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
║ AngkorFetch v1.2.1  •  Fast Cross-Platform System Fetch  •  by AMSDev ║
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
 ● Local IP   ❯ 192.168.x.x (Wi-Fi)
```

Detailed hardware output (`angkorfetch --hinfo`, logo omitted):

```text
 ● Motherboard  ❯ Dell Inc. 08NJ82
 ● BIOS         ❯ Dell Inc. 1.41.0
 ● Serial       ❯ XXXXXXX
 ● CPU          ❯ Intel® Core™ i5-8250U @ 1.60GHz (8 cores) @ 1.60 GHz
 ● GPU          ❯ Intel(R) UHD Graphics 620
 ● Memory       ❯ 8.0 GiB / 15.9 GiB (51%)
 ● RAM          ❯ DDR4 @ 2667 MHz (0080000080AD)
 ● Disk         ❯ 113.4 GiB / 255.0 GiB (44%)
 ● Disk Model   ❯ PM981 NVMe Samsung 256GB
 ● Disk Type    ❯ NVMe SSD
 ● Display      ❯ 1920x1080 @ 60Hz
 ● Ports        ❯ USB x25, Video Out x1, Audio x2
 ● WiFi         ❯ MyNetwork (83%)
 ● Battery      ❯ 63% [Discharging]
```

---

## Supported Platforms

Support is categorized into **Prebuilt Binaries**, **Packages** (release files, plus Pending or Planned channels whose manifest targets that platform), and **Source Builds**:

| Operating System | Architecture | Prebuilt Binary | Package | Implementation Details |
| --- | --- | --- | --- | --- |
| **Windows 10 & 11** | x86_64 | Yes | WinGet (Pending) | Registry, CIM/WMI, GDI |
| **Windows on ARM** | aarch64 | x64 build via emulation | No | Native build from source via `cargo install` (untested in CI) |
| **Linux (glibc)** | x86_64 | Yes | `.deb`, `.rpm` (Built); AUR, Flathub, Snap, Homebrew (Planned) | `/sys`, DMI, `lspci`, `dmidecode` |
| **Linux (glibc)** | aarch64 | Yes | AUR, Flathub, Snap, Homebrew (Planned) | `/sys`, DMI, `lspci`, `dmidecode` |
| **Linux (musl / Alpine)** | x86_64 | Yes | Flathub (Planned) | Same as glibc Linux |
| **Linux (musl / Alpine)** | aarch64 | No | Flathub (Planned) | Build from source via `cargo install` |
| **macOS (Intel)** | x86_64 | Yes | Homebrew (Planned) | `sysctl`, `system_profiler` |
| **macOS (Apple Silicon)** | aarch64 | Yes | Homebrew (Planned) | `sysctl`, `system_profiler` |
| **BSD (FreeBSD, OpenBSD)** | Any | No | No | Build from source; untested; only cross-platform fields populated |

---

## Update

| Installed with | How to update |
| --- | --- |
| Quick-install script | Re-run the same install command. It fetches the latest release and overwrites the binary; the PATH change is not repeated. |
| Cargo | `cargo install angkorfetch` (installs a newer crates.io version if there is one) |
| `.deb` / `.rpm` from Releases | Re-run the download and install steps in [Manual Installation](#manual-installation); they use the exact file name, so an older file in the folder is not picked up |

Package-manager commands will be listed here once those channels are live.

---

## Uninstall

```powershell
# Windows, quick-install script
Remove-Item -Recurse -Force "$env:LOCALAPPDATA\AngkorFetch"
```

Then remove the entry ending in `AppData\Local\AngkorFetch\bin` (for example `C:\Users\<you>\AppData\Local\AngkorFetch\bin`) from your user `Path` (Start menu: *Edit environment variables for your account*).

```bash
# macOS / Linux, quick-install script
rm -f ~/.local/bin/angkorfetch

# Cargo (any OS)
cargo uninstall angkorfetch

# Debian / Ubuntu (.deb from Releases)
sudo apt remove angkorfetch

# Fedora / RHEL (.rpm from Releases)
sudo dnf remove angkorfetch
```

`get.sh` may have added an `export PATH=` line for `.local/bin` to `~/.bashrc`, `~/.zshrc`, `~/.zprofile`, or `~/.bash_profile`; remove it only if nothing else uses `~/.local/bin`.

---

## Troubleshooting

* **Linux Root Privileges**: On Linux, hardware serial numbers and RAM vendor/speed metrics require root privileges to read `/sys/class/dmi` and `dmidecode`. For full diagnostics run `sudo "$(command -v angkorfetch)" --hinfo`; the full path is needed because `sudo` usually does not search `~/.local/bin` or `~/.cargo/bin`.
* **Wayland Displays**: Display detection tries `xrandr`, then `wlr-randr`, then the kernel DRM mode list in `/sys/class/drm`. Without XWayland or `wlr-randr` the value comes from DRM (resolution only, no refresh rate). It reports `Unknown` only if all three fail.
* **WSL (Windows Subsystem for Linux)**: WSL operates in a virtualized container. Host-level hardware fields (Motherboard BIOS, Serial, Battery) will report `Unknown`.
* **Confinement**: Strict sandbox environments (such as Snap confinement) may restrict access to `/sys/class/dmi`.

---

## Manual Installation

The [latest GitHub Release](https://github.com/AMRSKH/angkorfetch/releases/latest) includes:

| File | For |
| --- | --- |
| `angkorfetch-windows-x86_64.zip` | Windows x86_64 (`angkorfetch.exe`) |
| `angkorfetch-macos-x86_64.tar.gz` | macOS Intel |
| `angkorfetch-macos-aarch64.tar.gz` | macOS Apple Silicon |
| `angkorfetch-linux-x86_64.tar.gz` | Linux x86_64 (glibc) |
| `angkorfetch-linux-x86_64-musl.tar.gz` | Linux x86_64 (musl, e.g. Alpine) |
| `angkorfetch-linux-aarch64.tar.gz` | Linux aarch64 (glibc) |
| `angkorfetch_<version>_amd64.deb` | Debian / Ubuntu x86_64 |
| `angkorfetch-<version>-1.x86_64.rpm` | Fedora / RHEL x86_64 |
| `checksums.txt` | SHA256 of every file above |

From a Linux terminal, download a package and `checksums.txt`, check it, and install it by its exact name (the `.deb` and `.rpm` names include the version, so they are read from `checksums.txt`). The install runs only if `sha256sum` prints `OK`:

```bash
URL=https://github.com/AMRSKH/angkorfetch/releases/latest/download
curl -fsSLO "$URL/checksums.txt"
F=$(grep -oE 'angkorfetch_[^ ]+_amd64\.deb$' checksums.txt)    # .rpm: 'angkorfetch-[^ ]+\.x86_64\.rpm$'
curl -fsSLO "$URL/$F"
grep " $F\$" checksums.txt | sha256sum -c - && sudo apt install "./$F"    # .rpm: ... && sudo dnf install "./$F"
```

For an archive, set `F` to its name from the table (for example `F=angkorfetch-linux-x86_64.tar.gz`) and replace the install command after `&&` with an extract command (for example `tar xzf "$F"`), so it runs only if the check prints `OK`; then put `angkorfetch` (or `angkorfetch.exe`) in a folder on your PATH.

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
