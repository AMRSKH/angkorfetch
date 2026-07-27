# AngkorFetch

A fast, cross-platform system-info ("fetch") tool for Windows, Linux, and macOS, written in Rust.

## Quick Install

```bash
# macOS
brew install AMRSKH/tap/angkorfetch

# Windows
irm https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.ps1 | iex

# Linux
curl -fsSL https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.sh | bash

# Any OS (with Rust)
cargo install angkorfetch
```

## Usage

```
angkorfetch              Show the fetch summary
angkorfetch -v            Show version / logo banner
angkorfetch --hinfo       Show detailed hardware info (alias: --hard)
angkorfetch -h             Show help
```

## Features

- **OS** — name, version, build, architecture
- **Host** — device name
- **Model** — manufacturer and product name
- **CPU** — brand, cores, frequency, usage
- **GPU** — model and usage (NVIDIA + Linux DRM)
- **Memory** — used / total with percentage
- **Disk** — used / total with percentage
- **Display** — resolution and refresh rate
- **Battery** — charge level, status, health
- **WiFi** — SSID and signal strength
- **Network** — local IP address
- **Shell, Terminal, DE** — environment detection
- **Packages** — count by manager (winget, dpkg, rpm, pacman, brew, npm, etc.)

### Extended hardware info (`--hinfo`)

```
Motherboard  — vendor and model
BIOS         — vendor and version
Serial       — system serial number
RAM          — type, speed, manufacturer (e.g. DDR4 @ 3200 MHz)
Disk Model   — brand/model (e.g. Samsung SSD 970 EVO Plus)
Disk Type    — NVMe SSD / SATA SSD / HDD
Ports        — USB, video, audio counts
WiFi         — SSID with signal percentage
```

## Notes

Some fields (RAM speed, disk model, port counts) rely on OS-level tools
that may need elevated permissions. On Linux, run with `sudo` for the
most complete output:

```bash
sudo angkorfetch --hinfo
```

## Building

```bash
cargo build --release
./target/release/angkorfetch
```

## License

MIT
