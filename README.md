# AngkorFetch

[![crates.io](https://img.shields.io/crates/v/angkorfetch.svg)](https://crates.io/crates/angkorfetch)
[![build](https://github.com/AMRSKH/angkorfetch/actions/workflows/release.yml/badge.svg)](https://github.com/AMRSKH/angkorfetch/actions/workflows/release.yml)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**A fast, cross-platform system-info ("fetch") tool** written in Rust for Windows,
Linux and macOS.

## About

AngkorFetch is one command that answers a single question: **"what is this
machine?"** One run prints OS, CPU, GPU, RAM, disk, display, battery, WiFi and
package counts on a single screen. Adding `--hinfo` prints detailed hardware on
top of that: motherboard, BIOS, serial number, RAM type and speed, disk model
and type.

Four design rules:

1. **One binary, no runtime.** No Python, no Node, no shell framework. Three
   crates only: `sysinfo`, `colored`, `terminal_size`.

2. **Read what the OS actually reports.** The registry and CIM/WMI on Windows,
   `/sys` plus DMI on Linux, `sysctl` and `system_profiler` on macOS. Nothing is
   guessed.

3. **Fail gracefully.** A field that cannot be read prints `Unknown` or `N/A` by
   itself; the process does not panic and no other field is lost.

4. **Adapt to the terminal.** Three logo tiers (full, compact, none) and two
   gradients (24-bit or 16 colour), chosen from what the terminal supports.

## Install

macOS and Linux, via Homebrew:

```bash
brew install AMRSKH/tap/angkorfetch
```

Windows:

```powershell
irm https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.ps1 | iex
```

Installs to `%LOCALAPPDATA%\AngkorFetch\bin` and adds it to PATH automatically.

Linux and macOS, via script:

```bash
curl -fsSL https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.sh | bash
```

Installs to `~/.local/bin/angkorfetch`.

Any OS, via Rust:

```bash
cargo install angkorfetch
```

## Package status

Current version is **v1.1.1**. What is actually shipping, and what is not:

| Channel | Version | Status | Notes |
|---|---|---|---|
| [crates.io](https://crates.io/crates/angkorfetch) | 1.1.1 | Live | `cargo install angkorfetch` — every target Rust supports |
| GitHub Releases | v1.1.1 | Live | 5 archives plus `.deb`, `.rpm` and `checksums.txt` |
| Homebrew tap `AMRSKH/tap` | 1.1.1 | Live | macOS and Linux, x86_64 and aarch64 |
| `get.ps1` for Windows | v1.1.1 | Live | Pulls the asset from the latest release |
| `get.sh` for Linux and macOS | v1.1.1 | Live | Pulls the asset from the latest release |
| `.deb` | 1.1.1 | Download only | `amd64` only, no apt repository |
| `.rpm` | 1.1.1 | Download only | `x86_64` only, no dnf or yum repository |
| winget `AMRSKH.AngkorFetch` | 1.1.1 | Pending | PR [microsoft/winget-pkgs#409790](https://github.com/microsoft/winget-pkgs/pull/409790) is still open. `winget install` **does not work yet**, not until it merges |
| Snap | — | Not published | `snap/snapcraft.yaml` is in the repo but CI never builds it |
| Flatpak | — | Not published | `flatpak/io.github.AMRSKH.angkorfetch.yml` is in the repo but CI never builds it |
| Homebrew core, AUR, nixpkgs, Debian, Fedora, Scoop, Chocolatey | — | Not submitted | No concrete plans yet |

Installing the `.deb` and `.rpm`:

```bash
sudo dpkg -i angkorfetch_1.1.1_amd64.deb     # Debian and Ubuntu
sudo rpm -i angkorfetch-1.1.1-1.x86_64.rpm   # Fedora, RHEL and openSUSE
```

The Homebrew formula and the winget manifests are updated automatically after a
release by the `sync-packages` workflow, because they pin a `sha256` of artifacts
that do not exist until the tag is built. See `RELEASING.md`.

## Supported platforms

Prebuilt binaries are published for these five targets only.

| Operating system | Arch | Prebuilt | Install via |
|---|---|---|---|
| Windows 10 and 11 | x86_64 | Yes | `get.ps1`, `cargo install` (winget pending) |
| Linux with glibc | x86_64 | Yes | `get.sh`, Homebrew, `.deb`, `.rpm`, `cargo install` |
| Linux with glibc | aarch64 | Yes | `get.sh`, Homebrew, `cargo install` |
| macOS on Intel | x86_64 | Yes | Homebrew, `get.sh`, `cargo install` |
| macOS on Apple Silicon | aarch64 | Yes | Homebrew, `get.sh`, `cargo install` |

Every other target has to be built from source with `cargo install angkorfetch`.

## Not yet supported

### 1. Operating systems with no code path

The source branches on three systems only: `windows`, `linux` and `macos`
(`src/main.rs`). Anything else falls through to the fallback branches, so most
hardware fields report `Unknown` or `None`.

| Operating system | Status |
|---|---|
| FreeBSD, OpenBSD, NetBSD, DragonFly | Unsupported — no prebuilt binary, no hardware code path. `get.sh` refuses any `uname -s` other than `Linux` and `Darwin` |
| Android and Termux | Unsupported — `target_os = "android"` is not `"linux"`, so DMI, GPU, battery and packages fall through. Untested |
| Solaris and illumos | Unsupported — no code path, untested |
| Haiku, Redox and others | Unsupported — untested, not covered by CI |
| iOS and iPadOS | Not applicable — no CLI target |

On those systems only the `sysinfo`-backed basics can work: OS, Host, Uptime,
CPU, Memory, total disk size and Local IP.

### 2. Architectures and distros with no prebuilt binary

| Target | Status |
|---|---|
| Windows on ARM (aarch64) | No native build — `get.ps1` ships x64, which Windows runs under emulation. For a native binary use `cargo install angkorfetch` |
| Linux armv7, riscv64, i686 | Build from source |
| Alpine and other musl Linux | Build from source — the released binaries are `*-linux-gnu` and will not run without glibc |
| Windows 7, 8, 8.1 | Untested, not covered by CI |

## Usage

```bash
angkorfetch              # show system information
angkorfetch -v           # show the version
angkorfetch --hinfo      # detailed hardware information (--hard also works)
angkorfetch -h           # help
```

## What it shows

By default: OS, Host, Model, Uptime, CPU, CPU Usage, GPU, GPU Usage, Memory,
Disk, Display, Shell, Terminal, DE, Packages, Battery and Local IP.

With `--hinfo`: Motherboard, BIOS, Serial, RAM type, speed and vendor, Disk
Model, Disk Type, Ports and WiFi.

## Per-field support

| Field | Windows | Linux | macOS |
|---|---|---|---|
| OS, Host, Uptime, CPU, Memory, Disk, Local IP | Yes | Yes | Yes |
| Model | Yes | Yes | Yes |
| GPU | Yes | Needs `lspci` | Yes |
| GPU Usage | NVIDIA only | NVIDIA, or AMD via `gpu_busy_percent` | No, prints `N/A` |
| Display | Yes | X11 only, via `xrandr`. Wayland without XWayland prints `Unknown` | Yes |
| Shell, Terminal | Yes | Yes | Yes |
| DE | Fixed `Windows Explorer` | From `XDG_CURRENT_DESKTOP` | Fixed `Aqua` |
| Packages | winget, npm, registry apps | dpkg, rpm, pacman, apk, flatpak, snap, npm | brew, npm |
| Battery | Percent plus health | Percent plus health | Percent plus cycle count |
| WiFi | SSID plus signal strength | Needs `iwgetid` or `nmcli` | SSID only, and relies on `airport`, which Apple removed in macOS 14.4 |
| Motherboard | Yes | Yes | Derived from `hw.model` |
| BIOS | Yes | Yes | No, prints `Unknown` |
| Serial | Yes | Needs root | Yes |
| RAM type, speed, vendor | Yes | Needs root, via `dmidecode` | Yes |
| Disk Model | Yes | Yes, read from `/sys/block/*/device/model` | Yes |
| Disk Type | Yes | Needs `lsblk` | Yes |
| Ports | USB, Video Out, Audio | USB via `lsusb`, Video Out, Audio | USB and Audio, no video-out count |

## Example output

```
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

Output of `--hinfo`:

```
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

## How it works

| Area | Windows | Linux | macOS |
|---|---|---|---|
| Basics: OS, CPU, RAM, disk, network | `sysinfo` | `sysinfo` | `sysinfo` |
| Hardware | Registry and `Get-CimInstance` | `/sys/class/dmi`, `/sys/block`, `lspci`, `lsusb` | `sysctl`, `system_profiler`, `ioreg` |
| Display | `GetDeviceCaps` from GDI | `xrandr` | `system_profiler` |
| Battery | `GetSystemPowerStatus` | `/sys/class/power_supply` | `pmset` and `ioreg` |

CPU usage is computed from two samples 200 ms apart, so a run always costs a
little over 0.2 seconds. On Windows a few fields shell out to `powershell`, which
adds more.

## Build from source

```bash
git clone https://github.com/AMRSKH/angkorfetch.git
cd angkorfetch
cargo build --release          # produces target/release/angkorfetch
cargo test --locked            # 9 tests covering logo layout, gradient, wrapping
python -m unittest discover -s scripts -p "test_*.py"   # package sync tests
```

Requires Rust stable (edition 2021). No other build dependency.

## Notes

On Linux, run with `sudo` for complete output, specifically Serial and the RAM
type and speed:

```bash
sudo angkorfetch --hinfo
```

- **WSL** runs through the Linux path. Bare-metal fields such as BIOS, Serial,
  battery and display mostly read `Unknown`.
- **Wayland** — `Display` relies on `xrandr`, so without XWayland it reads
  `Unknown`.
- **Snap and Flatpak** — the manifests are in the repo (`snap/` and `flatpak/`)
  but the release workflow does not publish them. Snap also uses strict
  confinement, so some fields can be blocked.
- **GPU** reports the first adapter only, and **Display** the primary screen only.
- **Local IP** takes the first non-loopback IPv4 address and never shows IPv6.

## Uninstall

```bash
# installed with cargo
cargo uninstall angkorfetch

# installed with get.sh on Linux and macOS
rm ~/.local/bin/angkorfetch

# installed with get.ps1 on Windows
rm $env:LOCALAPPDATA\AngkorFetch\bin\angkorfetch.exe

# installed with Homebrew
brew uninstall angkorfetch

# installed from .deb or .rpm
sudo apt remove angkorfetch
sudo rpm -e angkorfetch
```

## Contributing

Branches: `main` is production and the only branch releases are tagged from.
`dev` is where new features and testing land. Send feature pull requests to
`dev`, and urgent fixes or documentation to `main`. Details are in
`RELEASING.md`.

- Want another operating system? Add a branch in `src/main.rs` and open a PR.
- Want to help publish to AUR, nixpkgs, Scoop or Chocolatey? Open an issue first.
- `Formula/angkorfetch.rb` is generated by `scripts/sync_package_manifests.py`.
  Edit the template in the script rather than the formula by hand, because CI
  compares it byte for byte.

## License

MIT. See [LICENSE](LICENSE).
