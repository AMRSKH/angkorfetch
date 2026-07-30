# AngkorFetch

**ឧបករណ៍បង្ហាញព័ត៌មានប្រព័ន្ធ** ដែលរត់លឿន សរសេរដោយ Rust សម្រាប់ Windows, Linux និង macOS ។

A fast, cross-platform system-info ("fetch") tool written in Rust.

[![crates.io](https://img.shields.io/crates/v/angkorfetch.svg)](https://crates.io/crates/angkorfetch)
[![build](https://github.com/AMRSKH/angkorfetch/actions/workflows/release.yml/badge.svg)](https://github.com/AMRSKH/angkorfetch/actions/workflows/release.yml)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## អំពីគម្រោង / About

AngkorFetch គឺជា command ១ បន្ទាត់ដែលឆ្លើយសំណួរថា **"កុំព្យូទ័រនេះជាអ្វី?"** ។
វាបង្ហាញ OS, CPU, GPU, RAM, Disk, Display, Battery, WiFi និង Packages ក្នុងរូបភាព
ស្អាតតែមួយអេក្រង់ ហើយ `--hinfo` បន្ថែមព័ត៌មានផ្នែករឹងលម្អិត ដូចជា Motherboard,
BIOS, Serial, ប្រភេទ RAM និងប្រភេទ Disk ។

គោលការណ៍រចនា ៤ ចំណុច៖

1. **Binary តែមួយ គ្មាន runtime** — គ្មាន Python, គ្មាន Node, គ្មាន shell framework ។
   Dependency មានតែ ៣ crate (`sysinfo`, `colored`, `terminal_size`) ។
2. **អានពីប្រភពដើមរបស់ OS** — Windows អានពី Registry និង CIM/WMI, Linux អានពី
   `/sys` និង DMI, macOS អានពី `sysctl` និង `system_profiler` ។ គ្មានការទាយ។
3. **ធ្លាក់ចុះដោយសុភាព (graceful degradation)** — បើ field ណាមួយអានមិនបាន វាបង្ហាញ
   `Unknown` ឬ `N/A` ជំនួស ដោយមិន panic និងមិនបាត់ field ដទៃ។
4. **សម្រួលតាមទំហំ terminal** — logo មាន ៣ ទ្រង់ (ពេញ / តូច / គ្មាន) និង gradient
   ២ បែប (24-bit ឬ 16 color) ជ្រើសតាមលទ្ធភាព terminal ។

AngkorFetch is one command that answers **"what is this machine?"** in a single
screen. It reads OS-native sources rather than guessing: the registry and CIM on
Windows, `/sys` plus DMI on Linux, `sysctl` and `system_profiler` on macOS. Every
field degrades to `Unknown`/`N/A` on its own instead of failing the whole run,
and the banner adapts to the terminal width and color depth. One static binary,
three crates, no runtime dependencies.

---

## ដំឡើង / Install

**macOS និង Linux (Homebrew)**
```bash
brew install AMRSKH/tap/angkorfetch
```

**Windows**
```powershell
irm https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.ps1 | iex
```
ដំឡើងទៅ `%LOCALAPPDATA%\AngkorFetch\bin` ហើយបន្ថែមទៅ PATH ដោយស្វ័យប្រវត្តិ។

**Linux / macOS (script)**
```bash
curl -fsSL https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.sh | bash
```
ដំឡើងទៅ `~/.local/bin/angkorfetch` ។

**គ្រប់ប្រព័ន្ធ / Any OS (Rust)**
```bash
cargo install angkorfetch
```

---

## ស្ថានភាពកញ្ចប់ / Package status

កំណែបច្ចុប្បន្ន **v1.1.1** ។ តារាងនេះបង្ហាញថាឆានែលណាដំណើរការហើយ ឆានែលណាមិនទាន់។
Current version is **v1.1.1**. What is actually shipping, and what is not:

| ឆានែល / Channel | កំណែ / Version | ស្ថានភាព / Status | កំណត់ចំណាំ / Notes |
|---|---|---|---|
| [crates.io](https://crates.io/crates/angkorfetch) | 1.1.1 | ប្រើបាន / Live | `cargo install angkorfetch` — គ្រប់ target ដែល Rust support |
| GitHub Releases | v1.1.1 | ប្រើបាន / Live | 5 archives + `.deb` + `.rpm` + `checksums.txt` |
| Homebrew tap `AMRSKH/tap` | 1.1.1 | ប្រើបាន / Live | macOS និង Linux, x86_64 + aarch64 |
| `get.ps1` (Windows) | v1.1.1 | ប្រើបាន / Live | ទាញ asset ពី release ចុងក្រោយ |
| `get.sh` (Linux/macOS) | v1.1.1 | ប្រើបាន / Live | ទាញ asset ពី release ចុងក្រោយ |
| `.deb` | 1.1.1 | ទាញដោយដៃ / Download only | `amd64` តែមួយ, គ្មាន apt repository |
| `.rpm` | 1.1.1 | ទាញដោយដៃ / Download only | `x86_64` តែមួយ, គ្មាន dnf/yum repository |
| winget `AMRSKH.AngkorFetch` | 1.1.1 | រង់ចាំ / Pending | PR [microsoft/winget-pkgs#409790](https://github.com/microsoft/winget-pkgs/pull/409790) នៅបើកចំហ។ `winget install` **មិនទាន់ដំណើរការ** ទេ រហូតដល់ merge |
| Snap | — | មិន publish / Not published | `snap/snapcraft.yaml` មានក្នុង repo តែ CI មិន build |
| Flatpak | — | មិន publish / Not published | `flatpak/io.github.AMRSKH.angkorfetch.yml` មានក្នុង repo តែ CI មិន build |
| Homebrew core, AUR, nixpkgs, Debian, Fedora, Scoop, Chocolatey | — | មិនទាន់ដាក់ / Not submitted | គ្មានផែនការជាក់លាក់នៅឡើយ |

`.deb` និង `.rpm` ដំឡើងដោយ៖
```bash
sudo dpkg -i angkorfetch_1.1.1_amd64.deb     # Debian / Ubuntu
sudo rpm -i angkorfetch-1.1.1-1.x86_64.rpm   # Fedora / RHEL / openSUSE
```

Homebrew formula និង winget manifest ត្រូវ update ដោយស្វ័យប្រវត្តិក្រោយ release
ដោយ workflow `sync-packages`, ព្រោះវា pin `sha256` របស់ artifact ដែលមិនអាចដឹងមុន
ពេលបង្កើត tag ។ សូមអាន `RELEASING.md` ។

---

## ប្រព័ន្ធដែលគាំទ្រ / Supported platforms

មាន binary ស្រេច (prebuilt) សម្រាប់តែ ៥ target ខាងក្រោម។
Prebuilt binaries are published for these five targets only:

| OS | Arch | Prebuilt | ដំឡើងតាម / Install via |
|---|---|---|---|
| Windows 10 / 11 | x86_64 | Yes | `get.ps1`, `cargo install` (winget នៅរង់ចាំ) |
| Linux (glibc) | x86_64 | Yes | `get.sh`, Homebrew, `.deb`, `.rpm`, `cargo install` |
| Linux (glibc) | aarch64 | Yes | `get.sh`, Homebrew, `cargo install` |
| macOS (Intel) | x86_64 | Yes | Homebrew, `get.sh`, `cargo install` |
| macOS (Apple Silicon) | aarch64 | Yes | Homebrew, `get.sh`, `cargo install` |

គ្រប់ target ផ្សេងទៀតត្រូវ build ដោយ `cargo install angkorfetch` ។
Every other target must be built from source with `cargo install angkorfetch`.

---

## មិនទាន់គាំទ្រ / Not yet supported

### ១. OS ដែលមិនមាន code path / Operating systems with no code path

កូដមានតែ ៣ សាខា៖ `windows`, `linux`, `macos` (`src/main.rs`) ។ OS ផ្សេងទៀតធ្លាក់ចូល
fallback branch ដូច្នេះ field ផ្នែករឹងភាគច្រើនចេញ `Unknown` / `None` ។

The source only branches on `windows`, `linux`, and `macos` (`src/main.rs`). Any other
OS falls through to the fallback branches, so most hardware fields report
`Unknown` / `None`:

| OS | ស្ថានភាព / Status |
|---|---|
| FreeBSD / OpenBSD / NetBSD / DragonFly | មិនគាំទ្រ — គ្មាន binary, គ្មាន code path. `get.sh` បដិសេធ `uname -s` ក្រៅពី `Linux`/`Darwin` |
| Android / Termux | មិនគាំទ្រ — `target_os = "android"` មិនមែន `"linux"` ដូច្នេះ DMI/GPU/battery/packages ធ្លាក់ចេញ។ មិនបានសាកល្បង |
| Solaris / illumos | មិនគាំទ្រ — មិនបានសាកល្បង, គ្មាន code path |
| Haiku / Redox / ផ្សេងៗ | មិនគាំទ្រ — មិនបានសាកល្បង, គ្មានក្នុង CI |
| iOS / iPadOS | មិនអនុវត្ត / Not applicable — គ្មាន CLI target |

នៅ OS ទាំងនេះ មានតែ field មូលដ្ឋានពី `sysinfo` ដែលអាចដំណើរការ៖ OS, Host, Uptime,
CPU, Memory, Disk total, Local IP ។
On those systems only the `sysinfo`-backed basics can work.

### ២. Arch / distro ដែលមិនមាន binary ស្រេច / No prebuilt binary

| Target | ស្ថានភាព / Status |
|---|---|
| Windows on ARM (aarch64) | គ្មាន native build — `get.ps1` ផ្ដល់ x64 ដែល Windows រត់ក្រោម emulation. បើចង់បាន native សូមប្រើ `cargo install angkorfetch` |
| Linux armv7 / riscv64 / i686 | Source only |
| Alpine / musl Linux | Source only — binary ដែល release គឺ `*-linux-gnu` រត់មិនបានបើគ្មាន glibc |
| Windows 7 / 8 / 8.1 | មិនបានសាកល្បង, គ្មានក្នុង CI |

---

## ប្រើប្រាស់ / Usage

```bash
angkorfetch              # បង្ហាញព័ត៌មានប្រព័ន្ធ
angkorfetch -v           # បង្ហាញកំណែ
angkorfetch --hinfo      # ព័ត៌មានលម្អិតផ្នែករឹង (--hard ក៏បាន)
angkorfetch -h           # ជំនួយ
```

---

## ព័ត៌មានបង្ហាញ / What it shows

OS · Host · Model · Uptime · CPU · CPU Usage · GPU · GPU Usage · Memory · Disk ·
Display · Shell · Terminal · DE · Packages · Battery · Local IP

**លម្អិត / Details** (`--hinfo`): Motherboard · BIOS · Serial · RAM type/speed/vendor ·
Disk Model · Disk Type · Ports · WiFi

---

## តារាងគាំទ្រតាម field / Per-field support

| Field | Windows | Linux | macOS |
|---|---|---|---|
| OS, Host, Uptime, CPU, Memory, Disk, Local IP | Yes | Yes | Yes |
| Model | Yes | Yes | Yes |
| GPU | Yes | ត្រូវការ `lspci` | Yes |
| GPU Usage | NVIDIA តែមួយ | NVIDIA, ឬ AMD តាម `gpu_busy_percent` | No (`N/A`) |
| Display | Yes | X11 តែមួយ — `xrandr` ។ Wayland គ្មាន XWayland ចេញ `Unknown` | Yes |
| Shell, Terminal | Yes | Yes | Yes |
| DE | ថេរ `Windows Explorer` | `XDG_CURRENT_DESKTOP` | ថេរ `Aqua` |
| Packages | winget, npm, registry apps | dpkg, rpm, pacman, apk, flatpak, snap, npm | brew, npm |
| Battery | % + health % | % + health % | % + cycle count |
| WiFi | SSID + signal % | ត្រូវការ `iwgetid` / `nmcli` | SSID តែមួយ, ហើយពឹងលើ `airport` ដែល Apple ដកចេញនៅ macOS 14.4+ |
| Motherboard | Yes | Yes | មកពី `hw.model` |
| BIOS | Yes | Yes | No (`Unknown`) |
| Serial | Yes | ត្រូវការ root | Yes |
| RAM type / speed / vendor | Yes | ត្រូវការ root (`dmidecode`) | Yes |
| Disk Model | Yes | Yes — អានពី `/sys/block/*/device/model` | Yes |
| Disk Type | Yes | ត្រូវការ `lsblk` | Yes |
| Ports | USB, Video Out, Audio | USB តាម `lsusb`, Video Out, Audio | USB, Audio — គ្មានចំនួន video-out |

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

## របៀបដំណើរការ / How it works

| ផ្នែក / Area | Windows | Linux | macOS |
|---|---|---|---|
| មូលដ្ឋាន (OS, CPU, RAM, Disk, Net) | `sysinfo` | `sysinfo` | `sysinfo` |
| ផ្នែករឹង / Hardware | Registry + `Get-CimInstance` | `/sys/class/dmi`, `/sys/block`, `lspci`, `lsusb` | `sysctl`, `system_profiler`, `ioreg` |
| Display | `GetDeviceCaps` (GDI) | `xrandr` | `system_profiler` |
| Battery | `GetSystemPowerStatus` | `/sys/class/power_supply` | `pmset`, `ioreg` |

CPU Usage គណនាពី sample ២ ដង ដោយមាន delay 200ms នៅចន្លោះ ដូច្នេះ run មួយចំណាយ
ពេលជាង 0.2 វិនាទីបន្តិច។ លើ Windows field មួយចំនួនហៅ `powershell` ដែលបន្ថែមពេល។

CPU usage needs two samples 200 ms apart, so a run always costs at least that;
on Windows a few fields shell out to `powershell`, which adds more.

---

## បង្កើតពី source / Build from source

```bash
git clone https://github.com/AMRSKH/angkorfetch.git
cd angkorfetch
cargo build --release          # target/release/angkorfetch
cargo test --locked            # 9 tests (logo layout, gradient, wrapping)
python -m unittest discover -s scripts -p "test_*.py"   # package sync tests
```

ត្រូវការ Rust stable (edition 2021) ។ គ្មាន build dependency ផ្សេង។

---

## កំណត់ចំណាំ / Notes

លើ Linux រត់ជាមួយ `sudo` ដើម្បីទទួលព័ត៌មានពេញលេញ (Serial, RAM type/speed) ។
On Linux, run with `sudo` for complete output (Serial, RAM type/speed):

```bash
sudo angkorfetch --hinfo
```

- **WSL** ដំណើរការតាម path Linux ។ Field ផ្នែករឹង (BIOS, Serial, Battery, Display)
  ភាគច្រើនចេញ `Unknown` ។
- **Wayland** — `Display` ពឹងលើ `xrandr`, ដូច្នេះបើគ្មាន XWayland វានឹងចេញ `Unknown` ។
- **Snap / Flatpak** — manifest មានក្នុង repo (`snap/`, `flatpak/`) តែ release workflow
  មិន publish ទេ។ Snap ប្រើ strict confinement ដូច្នេះ field មួយចំនួនអាចត្រូវរារាំង។
- **GPU** បង្ហាញ adapter ដំបូងតែមួយ, និង **Display** បង្ហាញអេក្រង់ចម្បងតែមួយ។
- **Local IP** យក IPv4 មិនមែន loopback ដំបូងគេ, មិនបង្ហាញ IPv6 ។

---

## លុប / Uninstall

```bash
# cargo
cargo uninstall angkorfetch

# get.sh (Linux/macOS)
rm ~/.local/bin/angkorfetch

# get.ps1 (Windows)
rm $env:LOCALAPPDATA\AngkorFetch\bin\angkorfetch.exe

# Homebrew
brew uninstall angkorfetch

# .deb / .rpm
sudo apt remove angkorfetch
sudo rpm -e angkorfetch
```

---

## ចូលរួម / Contributing

- ចង់បន្ថែម OS ថ្មី? សូមបន្ថែម branch ក្នុង `src/main.rs` រួចបើក PR ។
  Want another OS? Add a branch in `src/main.rs` and open a PR.
- ចង់ជួយ publish ទៅ AUR, nixpkgs, Scoop ឬ Chocolatey? សូមបើក issue មុន។
- `Formula/angkorfetch.rb` ត្រូវបង្កើតដោយ `scripts/sync_package_manifests.py` —
  សូមកែ template ក្នុង script មិនមែនកែ formula ដោយដៃ, ព្រោះ CI ប្រៀបធៀប byte ។

---

## អាជ្ញាប័ណ្ឌ / License

MIT — សូមអាន [LICENSE](LICENSE) ។
