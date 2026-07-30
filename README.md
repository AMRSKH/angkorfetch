# AngkorFetch

**ឧបករណ៍បង្ហាញព័ត៌មានប្រព័ន្ធ** សម្រាប់ Windows, Linux និង macOS សរសេរដោយ Rust ។

A fast, cross-platform system-info ("fetch") tool written in Rust.

---

## ដំឡើង / Install

**macOS**
```bash
brew install AMRSKH/tap/angkorfetch
```

**Windows**
```powershell
irm https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.ps1 | iex
```

**Linux**
```bash
curl -fsSL https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.sh | bash
```

**គ្រប់ប្រព័ន្ធ / Any OS (Rust)**
```bash
cargo install angkorfetch
```

---

## ប្រព័ន្ធដែលគាំទ្រ / Supported platforms

មាន binary ស្រេច (prebuilt) សម្រាប់តែ 5 target ខាងក្រោម។
Prebuilt binaries are published for these five targets only:

| OS | Arch | Prebuilt | ដំឡើងតាម / Install via |
|---|---|---|---|
| Windows 10 / 11 | x86_64 | Yes | `get.ps1`, winget, `cargo install` |
| Linux (glibc) | x86_64 | Yes | `get.sh`, Homebrew, `.deb`, `.rpm`, `cargo install` |
| Linux (glibc) | aarch64 | Yes | `get.sh`, Homebrew, `cargo install` |
| macOS (Intel) | x86_64 | Yes | Homebrew, `get.sh`, `cargo install` |
| macOS (Apple Silicon) | aarch64 | Yes | Homebrew, `get.sh`, `cargo install` |

គ្រប់ target ផ្សេងទៀតត្រូវ build ដោយ `cargo install angkorfetch` ។
Every other target must be built from source with `cargo install angkorfetch`.

---

## មិនទាន់គាំទ្រ / Not yet supported

### 1. OS ដែលមិនមាន code path / Operating systems with no code path

កូដមានតែ 3 សាខា៖ `windows`, `linux`, `macos` (`src/main.rs`) ។ OS ផ្សេងទៀតធ្លាក់ចូល
fallback branch ដូច្នេះ field ផ្នែករឹងភាគច្រើនចេញ `Unknown` / `None` ។

The source only branches on `windows`, `linux`, and `macos` (`src/main.rs`). Any other
OS falls through to the fallback branches, so most hardware fields report
`Unknown` / `None`:

| OS | ស្ថានភាព / Status |
|---|---|
| FreeBSD / OpenBSD / NetBSD / DragonFly | មិនគាំទ្រ — no prebuilt binary, no hardware code path. `get.sh` refuses any `uname -s` other than `Linux`/`Darwin`. May compile, output is degraded. |
| Android / Termux | មិនគាំទ្រ — `target_os = "android"` is not `"linux"`, so DMI/GPU/battery/packages fall through. Untested. |
| Solaris / illumos | មិនគាំទ្រ — untested, no code path. |
| Haiku / Redox / others | មិនគាំទ្រ — untested, not built in CI. |
| iOS / iPadOS | មិនអនុវត្ត / Not applicable — no CLI target. |

នៅ OS ទាំងនេះ មានតែ field មូលដ្ឋានពី `sysinfo` ដែលអាចដំណើរការ៖ OS, Host, Uptime,
CPU, Memory, Disk total, Local IP ។
On those systems only the `sysinfo`-backed basics can work: OS, Host, Uptime, CPU,
Memory, Disk totals, Local IP.

### 2. Arch / distro ដែលមិនមាន binary ស្រេច / No prebuilt binary

| Target | ស្ថានភាព / Status |
|---|---|
| Windows on ARM (aarch64) | No native build — `get.ps1` and winget ship x64 only, which Windows runs under x64 emulation. For a native binary use `cargo install angkorfetch`. |
| Linux armv7 / riscv64 / i686 | Source only. |
| Alpine / musl Linux | Source only — the released Linux binaries are `*-linux-gnu` and will not run without glibc. |
| Windows 7 / 8 / 8.1 | Untested. Not covered by CI. |

---

## លុប / Uninstall

```bash
# cargo
cargo uninstall angkorfetch

# get.sh (Linux/macOS)
rm ~/.local/bin/angkorfetch

# get.ps1 (Windows)
rm $env:LOCALAPPDATA\AngkorFetch\bin\angkorfetch.exe

# Homebrew (macOS)
brew uninstall angkorfetch
```

---

## ប្រើប្រាស់ / Usage

```bash
angkorfetch              # បង្ហាញព័ត៌មានប្រព័ន្ធ
angkorfetch -v            # បង្ហាញកំណែ
angkorfetch --hinfo       # ព័ត៌មានលម្អិតផ្នែករឹង
angkorfetch -h            # ជំនួយ
```

---

## ព័ត៌មានបង្ហាញ / What it shows

OS · Host · Model · CPU · GPU · Memory · Disk · Display · Battery · WiFi · Network · Shell · Terminal · DE · Packages

**លម្អិត / Details** (`--hinfo`): Motherboard · BIOS · Serial · RAM type/speed · Disk Model/Type · Ports · WiFi signal

---

## តារាងគាំទ្រតាម field / Per-field support

| Field | Windows | Linux | macOS |
|---|---|---|---|
| OS, Host, Uptime, CPU, Memory, Disk, Local IP | Yes | Yes | Yes |
| Model | Yes | Yes | Yes |
| GPU | Yes | Needs `lspci` | Yes |
| GPU Usage | NVIDIA only | NVIDIA, or AMD via `gpu_busy_percent` | No (`N/A`) |
| Display | Yes | X11 only — `xrandr`. Wayland without XWayland gives `Unknown` | Yes |
| Shell, Terminal | Yes | Yes | Yes |
| DE | Fixed `Windows Explorer` | `XDG_CURRENT_DESKTOP` | Fixed `Aqua` |
| Packages | winget, npm, registry apps | dpkg, rpm, pacman, apk, flatpak, snap, npm | brew, npm |
| Battery | % + health % | % + health % | % + cycle count |
| WiFi | SSID + signal % | Needs `iwgetid` / `nmcli` | SSID only, and relies on the legacy `airport` tool Apple removed in macOS 14.4+ |
| Motherboard | Yes | Yes | Derived from `hw.model` |
| BIOS | Yes | Yes | No (`Unknown`) |
| Serial | Yes | Needs root | Yes |
| RAM type / speed / vendor | Yes | Needs root (`dmidecode`) | Yes |
| Disk Model | Yes | Yes — read from `/sys/block/*/device/model` | Yes |
| Disk Type | Yes | Needs `lsblk` | Yes |
| Ports | USB, Video Out, Audio | USB via `lsusb`, Video Out, Audio | USB, Audio — no video-out count |

---

## ឧទាហរណ៍ / Example output

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

**ព័ត៌មានលម្អិត / Hardware info** (`--hinfo`):

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

---

## កំណត់ចំណាំ / Notes

លើ Linux រត់ជាមួយ `sudo` ដើម្បីទទួលព័ត៌មានពេញលេញ (Serial, RAM type/speed) ។
On Linux, run with `sudo` for complete output (Serial, RAM type/speed):

```bash
sudo angkorfetch --hinfo
```

- **WSL** ដំណើរការជា Linux ។ Field ផ្នែករឹង (BIOS, Serial, Battery, Display) ភាគច្រើនចេញ `Unknown` ។
  WSL runs through the Linux path; most bare-metal fields read `Unknown`.
- **Wayland** — `Display` ពឹងលើ `xrandr`, ដូច្នេះបើគ្មាន XWayland វានឹងចេញ `Unknown` ។
- **Snap / Flatpak** — មាន manifest ក្នុង repo (`snap/`, `flatpak/`) តែ CI មិន publish ទេ, ត្រូវ build ខ្លួនឯង។
  The `snap/` and `flatpak/` definitions are in-tree but the release workflow does not
  publish them; only the tarballs, the Windows zip, `.deb` and `.rpm` are released.
  Snap also uses strict confinement, so some fields can be blocked.
- ចង់បន្ថែម OS ថ្មី? សូមបន្ថែម branch ក្នុង `src/main.rs` ។
  Want another OS? Add a branch in `src/main.rs` and open a PR.

---

## អាជ្ញាប័ណ្ឌ / License

MIT
