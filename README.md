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

## ឧទាហរណ៍ / Example output

```
  █████╗  ███╗   ██╗  ██████╗  ██╗  ██╗  ██████╗  ██████╗
 ██╔══██╗ ████╗  ██║ ██╔════╝ ██║ ██╔╝ ██╔══██╗ ██╔══██╗
 ███████║ ██╔██╗ ██║ ██║  ███╗ █████╔╝  ██║  ██║ ██████╔╝
 ██╔══██║ ██║╚██╗██║ ██║   ██║ ██╔═██╗  ██║  ██║ ██╔══██╗
 ██║  ██║ ██║ ╚████║ ╚██████╔╝ ██║  ██╗ ╚██████╔╝ ██║  ██║
 ╚═╝  ╚═╝ ╚═╝  ╚═══╝  ╚═════╝  ╚═╝  ╚═╝  ╚═════╝  ╚═╝  ╚═╝

╔═══════════════════════════════════════════════════════════════════════╗
║ AngkorFetch v1.0.1  •  Fast Cross-Platform System Fetch  •  by AMSDev ║
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

លើ Linux រត់ជាមួយ `sudo` ដើម្បីទទួលព័ត៌មានពេញលេញ។
On Linux, run with `sudo` for complete output:

```bash
sudo angkorfetch --hinfo
```

---

## អាជ្ញាប័ណ្ឌ / License

MIT
