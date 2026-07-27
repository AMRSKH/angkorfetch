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

## កំណត់ចំណាំ / Notes

លើ Linux រត់ជាមួយ `sudo` ដើម្បីទទួលព័ត៌មានពេញលេញ។
On Linux, run with `sudo` for complete output:

```bash
sudo angkorfetch --hinfo
```

---

## សាងសង់ / Build

```bash
cargo build --release
./target/release/angkorfetch
```

---

## អាជ្ញាប័ណ្ឌ / License

MIT
