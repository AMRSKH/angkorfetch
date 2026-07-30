# AngkorFetch

[![crates.io](https://img.shields.io/crates/v/angkorfetch.svg)](https://crates.io/crates/angkorfetch)
[![build](https://github.com/AMRSKH/angkorfetch/actions/workflows/release.yml/badge.svg)](https://github.com/AMRSKH/angkorfetch/actions/workflows/release.yml)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

កំណែភាសាអង់គ្លេស មាននៅខាងក្រោម។

An English version follows below.

---
---

# ភាសាខ្មែរ

**ឧបករណ៍បង្ហាញព័ត៌មានប្រព័ន្ធ** ដែលរត់លឿន សរសេរដោយ Rust សម្រាប់ Windows, Linux និង macOS ។

## អំពីគម្រោង

AngkorFetch គឺជាឧបករណ៍បន្ទាត់បញ្ជាដែលឆ្លើយសំណួរតែមួយ៖ **"កុំព្យូទ័រនេះជាអ្វី?"** ។
រត់ command មួយ វាបង្ហាញ OS, CPU, GPU, RAM, Disk, អេក្រង់, ថ្ម, WiFi និងចំនួន
package ក្នុងអេក្រង់តែមួយ។ បើបន្ថែម `--hinfo` វាបង្ហាញព័ត៌មានផ្នែករឹងលម្អិតទៀត៖
Motherboard, BIOS, លេខសម្គាល់ម៉ាស៊ីន, ប្រភេទនិងល្បឿន RAM, ម៉ូដែលនិងប្រភេទ Disk ។

គោលការណ៍រចនា ៤ ចំណុច៖

១. **Binary តែមួយ គ្មាន runtime** — គ្មាន Python, គ្មាន Node, គ្មាន shell framework ។
   Dependency មានតែ ៣ crate ប៉ុណ្ណោះ៖ `sysinfo`, `colored`, `terminal_size` ។

២. **អានពីប្រភពពិតរបស់ប្រព័ន្ធ** — នៅ Windows អានពី Registry និង CIM/WMI, នៅ Linux
   អានពី `/sys` និង DMI, នៅ macOS អានពី `sysctl` និង `system_profiler` ។
   គ្មានការទាយតម្លៃទេ។

៣. **បរាជ័យដោយសុភាព** — បើ field ណាមួយអានមិនបាន វាបង្ហាញ `Unknown` ឬ `N/A`
   សម្រាប់តែ field នោះ ដោយកម្មវិធីមិន panic ហើយ field ដទៃមិនបាត់។

៤. **សម្របតាមទំហំ terminal** — logo មាន ៣ ទ្រង់ (ពេញ, តូច, គ្មាន) និង gradient
   ២ បែប (24-bit ឬ 16 ពណ៌) ជ្រើសរើសតាមលទ្ធភាពរបស់ terminal ។

## ដំឡើង

macOS និង Linux តាម Homebrew៖

```bash
brew install AMRSKH/tap/angkorfetch
```

Windows៖

```powershell
irm https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.ps1 | iex
```

ដំឡើងទៅ `%LOCALAPPDATA%\AngkorFetch\bin` ហើយបន្ថែមទៅ PATH ដោយស្វ័យប្រវត្តិ។

Linux និង macOS តាម script៖

```bash
curl -fsSL https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.sh | bash
```

ដំឡើងទៅ `~/.local/bin/angkorfetch` ។

គ្រប់ប្រព័ន្ធ តាម Rust៖

```bash
cargo install angkorfetch
```

## ស្ថានភាពកញ្ចប់

កំណែបច្ចុប្បន្នគឺ **v1.1.1** ។ តារាងខាងក្រោមបង្ហាញថាឆានែលណាដំណើរការហើយ
និងឆានែលណាមិនទាន់។

| ឆានែល | កំណែ | ស្ថានភាព | កំណត់ចំណាំ |
|---|---|---|---|
| [crates.io](https://crates.io/crates/angkorfetch) | 1.1.1 | ដំណើរការ | `cargo install angkorfetch` — គ្រប់ target ដែល Rust គាំទ្រ |
| GitHub Releases | v1.1.1 | ដំណើរការ | ៥ archive បូក `.deb`, `.rpm` និង `checksums.txt` |
| Homebrew tap `AMRSKH/tap` | 1.1.1 | ដំណើរការ | macOS និង Linux, x86_64 និង aarch64 |
| `get.ps1` សម្រាប់ Windows | v1.1.1 | ដំណើរការ | ទាញ asset ពី release ចុងក្រោយ |
| `get.sh` សម្រាប់ Linux និង macOS | v1.1.1 | ដំណើរការ | ទាញ asset ពី release ចុងក្រោយ |
| `.deb` | 1.1.1 | ទាញដោយដៃ | `amd64` តែមួយ, គ្មាន apt repository |
| `.rpm` | 1.1.1 | ទាញដោយដៃ | `x86_64` តែមួយ, គ្មាន dnf ឬ yum repository |
| winget `AMRSKH.AngkorFetch` | 1.1.1 | រង់ចាំ | PR [microsoft/winget-pkgs#409790](https://github.com/microsoft/winget-pkgs/pull/409790) នៅបើកចំហ។ `winget install` **មិនទាន់ដំណើរការ** រហូតដល់គេ merge |
| Snap | — | មិន publish | `snap/snapcraft.yaml` មានក្នុង repo តែ CI មិន build |
| Flatpak | — | មិន publish | `flatpak/io.github.AMRSKH.angkorfetch.yml` មានក្នុង repo តែ CI មិន build |
| Homebrew core, AUR, nixpkgs, Debian, Fedora, Scoop, Chocolatey | — | មិនទាន់ដាក់ | គ្មានផែនការជាក់លាក់នៅឡើយ |

ដំឡើង `.deb` និង `.rpm`៖

```bash
sudo dpkg -i angkorfetch_1.1.1_amd64.deb     # Debian និង Ubuntu
sudo rpm -i angkorfetch-1.1.1-1.x86_64.rpm   # Fedora, RHEL និង openSUSE
```

Homebrew formula និង winget manifest ត្រូវ update ដោយស្វ័យប្រវត្តិក្រោយ release
ដោយ workflow `sync-packages`, ព្រោះវា pin `sha256` របស់ artifact ដែលមិនអាចដឹងមុន
ពេលបង្កើត tag ។ សូមអាន `RELEASING.md` ។

## ប្រព័ន្ធដែលគាំទ្រ

មាន binary ស្រេច (prebuilt) សម្រាប់តែ ៥ target ខាងក្រោមប៉ុណ្ណោះ។

| ប្រព័ន្ធប្រតិបត្តិការ | Arch | មាន binary ស្រេច | ដំឡើងតាម |
|---|---|---|---|
| Windows 10 និង 11 | x86_64 | មាន | `get.ps1`, `cargo install` (winget នៅរង់ចាំ) |
| Linux ដែលប្រើ glibc | x86_64 | មាន | `get.sh`, Homebrew, `.deb`, `.rpm`, `cargo install` |
| Linux ដែលប្រើ glibc | aarch64 | មាន | `get.sh`, Homebrew, `cargo install` |
| macOS លើ Intel | x86_64 | មាន | Homebrew, `get.sh`, `cargo install` |
| macOS លើ Apple Silicon | aarch64 | មាន | Homebrew, `get.sh`, `cargo install` |

Target ផ្សេងទៀតទាំងអស់ត្រូវ build ពី source ដោយ `cargo install angkorfetch` ។

## មិនទាន់គាំទ្រ

### ១. ប្រព័ន្ធដែលគ្មាន code path

កូដមានតែ ៣ សាខាប៉ុណ្ណោះ៖ `windows`, `linux` និង `macos` (`src/main.rs`) ។
ប្រព័ន្ធផ្សេងទៀតធ្លាក់ចូល fallback branch ដូច្នេះ field ផ្នែករឹងភាគច្រើនចេញ
`Unknown` ឬ `None` ។

| ប្រព័ន្ធប្រតិបត្តិការ | ស្ថានភាព |
|---|---|
| FreeBSD, OpenBSD, NetBSD, DragonFly | មិនគាំទ្រ — គ្មាន binary ស្រេច គ្មាន code path ។ `get.sh` បដិសេធ `uname -s` ក្រៅពី `Linux` និង `Darwin` |
| Android និង Termux | មិនគាំទ្រ — `target_os = "android"` មិនមែន `"linux"` ដូច្នេះ DMI, GPU, ថ្ម និង package ធ្លាក់ចេញ។ មិនបានសាកល្បង |
| Solaris និង illumos | មិនគាំទ្រ — គ្មាន code path មិនបានសាកល្បង |
| Haiku, Redox និងផ្សេងៗ | មិនគាំទ្រ — មិនបានសាកល្បង គ្មានក្នុង CI |
| iOS និង iPadOS | មិនអនុវត្ត — គ្មាន target សម្រាប់ CLI |

នៅលើប្រព័ន្ធទាំងនេះ មានតែ field មូលដ្ឋានដែលមកពី `sysinfo` អាចដំណើរការបាន៖
OS, Host, Uptime, CPU, Memory, ទំហំ Disk សរុប និង Local IP ។

### ២. Arch និង distro ដែលគ្មាន binary ស្រេច

| Target | ស្ថានភាព |
|---|---|
| Windows លើ ARM (aarch64) | គ្មាន native build — `get.ps1` ផ្ដល់ x64 ដែល Windows រត់ក្រោម emulation ។ បើចង់បាន native សូមប្រើ `cargo install angkorfetch` |
| Linux armv7, riscv64, i686 | ត្រូវ build ពី source |
| Alpine និង Linux ដែលប្រើ musl | ត្រូវ build ពី source — binary ដែល release គឺ `*-linux-gnu` រត់មិនបានបើគ្មាន glibc |
| Windows 7, 8, 8.1 | មិនបានសាកល្បង គ្មានក្នុង CI |

## ប្រើប្រាស់

```bash
angkorfetch              # បង្ហាញព័ត៌មានប្រព័ន្ធ
angkorfetch -v           # បង្ហាញកំណែ
angkorfetch --hinfo      # ព័ត៌មានលម្អិតផ្នែករឹង (ប្រើ --hard ក៏បាន)
angkorfetch -h           # ជំនួយ
```

## ព័ត៌មានដែលបង្ហាញ

ធម្មតា៖ OS, Host, Model, Uptime, CPU, CPU Usage, GPU, GPU Usage, Memory, Disk,
Display, Shell, Terminal, DE, Packages, Battery និង Local IP ។

ជាមួយ `--hinfo`៖ Motherboard, BIOS, Serial, ប្រភេទ ល្បឿន និងក្រុមហ៊ុនផលិត RAM,
Disk Model, Disk Type, Ports និង WiFi ។

## តារាងគាំទ្រតាម field

| Field | Windows | Linux | macOS |
|---|---|---|---|
| OS, Host, Uptime, CPU, Memory, Disk, Local IP | បាន | បាន | បាន |
| Model | បាន | បាន | បាន |
| GPU | បាន | ត្រូវការ `lspci` | បាន |
| GPU Usage | NVIDIA តែមួយ | NVIDIA ឬ AMD តាម `gpu_busy_percent` | មិនបាន ចេញ `N/A` |
| Display | បាន | X11 តែមួយ តាម `xrandr` ។ Wayland គ្មាន XWayland ចេញ `Unknown` | បាន |
| Shell, Terminal | បាន | បាន | បាន |
| DE | ថេរ `Windows Explorer` | អានពី `XDG_CURRENT_DESKTOP` | ថេរ `Aqua` |
| Packages | winget, npm, registry apps | dpkg, rpm, pacman, apk, flatpak, snap, npm | brew, npm |
| Battery | ភាគរយ បូកសុខភាពថ្ម | ភាគរយ បូកសុខភាពថ្ម | ភាគរយ បូកចំនួន cycle |
| WiFi | SSID បូកកម្រិតសញ្ញា | ត្រូវការ `iwgetid` ឬ `nmcli` | SSID តែមួយ ហើយពឹងលើ `airport` ដែល Apple ដកចេញនៅ macOS 14.4 ឡើង |
| Motherboard | បាន | បាន | មកពី `hw.model` |
| BIOS | បាន | បាន | មិនបាន ចេញ `Unknown` |
| Serial | បាន | ត្រូវការសិទ្ធិ root | បាន |
| RAM ប្រភេទ ល្បឿន ក្រុមហ៊ុន | បាន | ត្រូវការសិទ្ធិ root តាម `dmidecode` | បាន |
| Disk Model | បាន | បាន អានពី `/sys/block/*/device/model` | បាន |
| Disk Type | បាន | ត្រូវការ `lsblk` | បាន |
| Ports | USB, Video Out, Audio | USB តាម `lsusb`, Video Out, Audio | USB និង Audio គ្មានចំនួន video out |

## ឧទាហរណ៍លទ្ធផល

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

លទ្ធផលរបស់ `--hinfo`៖

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

## របៀបដំណើរការ

| ផ្នែក | Windows | Linux | macOS |
|---|---|---|---|
| មូលដ្ឋាន៖ OS, CPU, RAM, Disk, Network | `sysinfo` | `sysinfo` | `sysinfo` |
| ផ្នែករឹង | Registry និង `Get-CimInstance` | `/sys/class/dmi`, `/sys/block`, `lspci`, `lsusb` | `sysctl`, `system_profiler`, `ioreg` |
| អេក្រង់ | `GetDeviceCaps` របស់ GDI | `xrandr` | `system_profiler` |
| ថ្ម | `GetSystemPowerStatus` | `/sys/class/power_supply` | `pmset` និង `ioreg` |

CPU Usage គណនាពី sample ២ ដង ដោយមាន delay 200 មិល្លីវិនាទីនៅចន្លោះ ដូច្នេះការរត់
មួយដងចំណាយពេលច្រើនជាង 0.2 វិនាទីបន្តិច។ នៅលើ Windows field មួយចំនួនហៅ
`powershell` ដែលបន្ថែមពេលទៅទៀត។

## បង្កើតពី source

```bash
git clone https://github.com/AMRSKH/angkorfetch.git
cd angkorfetch
cargo build --release          # ចេញនៅ target/release/angkorfetch
cargo test --locked            # ៩ test សម្រាប់ logo, gradient និងការកាត់បន្ទាត់
python -m unittest discover -s scripts -p "test_*.py"   # test សម្រាប់ package sync
```

ត្រូវការ Rust stable (edition 2021) ។ គ្មាន build dependency ផ្សេងទេ។

## កំណត់ចំណាំ

នៅលើ Linux សូមរត់ជាមួយ `sudo` ដើម្បីទទួលព័ត៌មានពេញលេញ ដូចជា Serial និងប្រភេទ
ល្បឿន RAM៖

```bash
sudo angkorfetch --hinfo
```

- **WSL** ដំណើរការតាមផ្លូវ Linux ។ Field ផ្នែករឹងដូចជា BIOS, Serial, ថ្ម និងអេក្រង់
  ភាគច្រើនចេញ `Unknown` ។
- **Wayland** — `Display` ពឹងលើ `xrandr` ដូច្នេះបើគ្មាន XWayland វាចេញ `Unknown` ។
- **Snap និង Flatpak** — manifest មានក្នុង repo (`snap/` និង `flatpak/`) តែ release
  workflow មិន publish ទេ។ Snap ប្រើ strict confinement ដូច្នេះ field មួយចំនួន
  អាចត្រូវរារាំង។
- **GPU** បង្ហាញ adapter ដំបូងតែមួយ ហើយ **Display** បង្ហាញអេក្រង់ចម្បងតែមួយ។
- **Local IP** យក IPv4 ដែលមិនមែន loopback ដំបូងគេ មិនបង្ហាញ IPv6 ទេ។

## លុបចេញ

```bash
# ដំឡើងតាម cargo
cargo uninstall angkorfetch

# ដំឡើងតាម get.sh លើ Linux និង macOS
rm ~/.local/bin/angkorfetch

# ដំឡើងតាម get.ps1 លើ Windows
rm $env:LOCALAPPDATA\AngkorFetch\bin\angkorfetch.exe

# ដំឡើងតាម Homebrew
brew uninstall angkorfetch

# ដំឡើងតាម .deb និង .rpm
sudo apt remove angkorfetch
sudo rpm -e angkorfetch
```

## ចូលរួមអភិវឌ្ឍ

សាខា៖ `main` គឺ production ហើយ release ត្រូវ tag ពី `main` តែមួយ។ `dev` គឺសម្រាប់
អភិវឌ្ឍ និងសាកល្បង feature ថ្មី។ សូមបើក PR ទៅ `dev` សម្រាប់ feature ថ្មី និងទៅ
`main` សម្រាប់ការជួសជុលបន្ទាន់ ឬឯកសារ។ ព័ត៌មានលម្អិតនៅក្នុង `RELEASING.md` ។

- ចង់បន្ថែមប្រព័ន្ធថ្មី? សូមបន្ថែម branch ក្នុង `src/main.rs` រួចបើក PR ។
- ចង់ជួយ publish ទៅ AUR, nixpkgs, Scoop ឬ Chocolatey? សូមបើក issue មុន។
- `Formula/angkorfetch.rb` បង្កើតដោយ `scripts/sync_package_manifests.py` ។
  សូមកែ template ក្នុង script កុំកែ formula ដោយដៃ ព្រោះ CI ប្រៀបធៀបរាល់ byte ។

## អាជ្ញាប័ណ្ឌ

MIT ។ សូមអាន [LICENSE](LICENSE) ។

---
---

# English

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
