អ្នកកំពុងធ្វើការផ្ទាល់លើ repository ដែលមានស្រាប់របស់ខ្ញុំ៖

https://github.com/AMRSKH/angkorfetch

ខ្ញុំជា repository owner និង maintainer។

កុំប្រាប់ខ្ញុំឱ្យ clone repository។
កុំធ្វើ workflow ដូចជា external contributor សម្រាប់ខ្ញុំ។
ត្រូវពិនិត្យ និងកែ repository ដែលមានស្រាប់ដោយផ្ទាល់។

# MAIN GOAL

ផ្លាស់ប្តូរ AngkorFetch ទៅជា package-manager-first installation strategy។

គោលដៅគឺឱ្យអ្នកប្រើអាចដំឡើងដោយ command សាមញ្ញតាម OS របស់ពួកគេ៖

| OS              | Package Manager | Target Install Command                  |
| --------------- | --------------- | --------------------------------------- |
| Windows         | WinGet          | `winget install angkorfetch`            |
| macOS           | Homebrew        | `brew install angkorfetch`              |
| Fedora          | DNF             | `sudo dnf install angkorfetch`          |
| Ubuntu / Debian | APT             | `sudo apt install angkorfetch`          |
| Arch Linux      | Pacman          | `sudo pacman -S angkorfetch`            |
| Universal Linux | Flatpak         | `flatpak install flathub <REAL_APP_ID>` |

សំខាន់៖
គោលដៅមិនមែនគ្រាន់តែកែ README ឱ្យបង្ហាញ command ទាំងនេះទេ។

ត្រូវរៀបចំ package metadata, repository/distribution integration, CI/CD, release artifacts និង automation ដែលចាំបាច់ ដើម្បីឱ្យ command ទាំងនេះអាចក្លាយជា installation methods ពិតប្រាកដ។

កុំបង្កើត fake package support ឬ fake repository។

---

# 1. INSPECT CURRENT REPOSITORY FIRST

មុនកែអ្វី ត្រូវពិនិត្យ repository ទាំងមូល។

ពិនិត្យយ៉ាងហោចណាស់៖

* `README.md`
* `Cargo.toml`
* `Cargo.lock`
* `src/`
* `scripts/`
* `get.sh`
* `get.ps1`
* `install.sh`
* `install.ps1`
* `Formula/`
* `flatpak/`
* `snap/`
* `winget-pkgs/`
* `linux/`
* `.github/workflows/`
* `RELEASING.md`

ពិនិត្យ current package status និង current release workflow ផងដែរ។

កំណត់ឱ្យច្បាស់ថា៖

* អ្វីបាន publish រួច
* អ្វីកំពុង pending
* អ្វីមានតែ manifest
* អ្វីជា local package
* អ្វីមិនទាន់មាន distribution channel

កុំសន្មត់ថា configuration file មួយមានន័យថា package បាន publish រួច។

---

# 2. TARGET INSTALLATION EXPERIENCE

User-facing installation ត្រូវផ្លាស់ទៅ package-manager-first។

README ត្រូវរៀបចំជាបែបនេះ៖

## Installation

### Windows

```powershell
winget install angkorfetch
```

### macOS

```bash
brew install angkorfetch
```

### Fedora

```bash
sudo dnf install angkorfetch
```

### Ubuntu / Debian

```bash
sudo apt install angkorfetch
```

### Arch Linux

```bash
sudo pacman -S angkorfetch
```

### Flatpak

```bash
flatpak install flathub <REAL_APP_ID>
```

កុំបង្ហាញ placeholder នៅ production documentation។

សម្រាប់ Flatpak ត្រូវជ្រើស App ID ពិតប្រាកដ ហើយប្រើ App ID នោះគ្រប់ទីកន្លែង។

---

# 3. IMPORTANT PACKAGE AVAILABILITY RULE

ត្រូវបែងចែកច្បាស់រវាង៖

1. Package source code exists
2. Package is built
3. Package is submitted
4. Package is approved
5. Package is published
6. Package is installable by the target command

កុំចាត់ទុក package ជា "Available" ត្រឹមតែមាន manifest ឬ package file ក្នុង repository។

ឧទាហរណ៍៖

```bash
sudo apt install ./angkorfetch.deb
```

មិនមែនជា native APT repository installation។

```bash
sudo dnf install ./angkorfetch.rpm
```

មិនមែនជា native DNF repository installation។

ត្រូវរក្សាភាពខុសគ្នានេះក្នុង documentation។

---

# 4. WINDOWS / WINGET

គោលដៅចុងក្រោយ៖

```powershell
winget install angkorfetch
```

ត្រូវពិនិត្យ existing:

```text
winget-pkgs/manifests/
```

និង current package identity។

ត្រូវធ្វើឱ្យ:

```powershell
winget search angkorfetch
```

អាចរកឃើញ package បាន។

ត្រូវ verify:

```powershell
winget search angkorfetch
winget show angkorfetch
winget install angkorfetch
winget upgrade angkorfetch
winget uninstall angkorfetch
```

Use the real package identity internally.

កុំប្តូរ package ID ទៅជា invented ID។

WinGet community repository ត្រូវការការសរសេរ និង validation នៃ manifest មុន submission, ដូច្នេះ repository ក្នុង AngkorFetch ត្រូវរៀបចំឱ្យស្របនឹង external publishing workflow។

---

# 5. MACOS / HOMEBREW

គោលដៅ៖

```bash
brew install angkorfetch
```

មិនគួរតម្រូវឱ្យ user សរសេរ:

```bash
brew install AMRSKH/tap/angkorfetch
```

ជាវិធីសំខាន់ទេ។

ពិនិត្យ Formula បច្ចុប្បន្ន និងកំណត់ថាតើអាចប្រើ official Homebrew core បានឬអត់។

បើត្រូវការ external submission មុន អនុវត្ត package formula និង automation ឱ្យរួចសិន ហើយ documentation ត្រូវបង្ហាញ status ពិតប្រាកដរហូតដល់ package ត្រូវបាន publish។

Verify:

```bash
brew search angkorfetch
brew info angkorfetch
brew install angkorfetch
brew upgrade angkorfetch
brew uninstall angkorfetch
```

Formula ត្រូវ synchronize ជាមួយ GitHub Releases និង SHA256។

---

# 6. FEDORA / DNF

គោលដៅ៖

```bash
sudo dnf install angkorfetch
```

ត្រូវផ្លាស់ពី current direct RPM installation ទៅជា actual DNF repository/package distribution។

ត្រូវបង្កើត ឬរៀបចំ:

* RPM spec/package metadata
* repository metadata
* signing strategy ប្រសិនបើត្រូវការ
* publishing workflow
* version synchronization
* installation verification
* upgrade verification

កុំប្រើ local RPM command ជំនួស native repository support។

បើត្រូវ external Fedora packaging/submission មុន, document វាជា external publishing step និងកុំ claim "Available" មុន package publish ពិតប្រាកដ។

---

# 7. UBUNTU / DEBIAN / APT

គោលដៅ៖

```bash
sudo apt install angkorfetch
```

ត្រូវបង្កើត actual APT repository/distribution flow។

រៀបចំ៖

* `.deb` package
* package metadata
* repository structure
* package index metadata
* signing/key strategy
* publication
* release synchronization
* update workflow

មិនអនុញ្ញាតឱ្យ README ប្រើ local `.deb` ជា primary installation method ទៀតទេ។

ត្រូវបែងចែក:

### Repository setup

និង

### Install

ឧទាហរណ៍៖

```bash
# repository setup, only if required
...

# installation
sudo apt update
sudo apt install angkorfetch
```

Final user experience ត្រូវឱ្យ install command នៅតែ៖

```bash
sudo apt install angkorfetch
```

---

# 8. ARCH LINUX / PACMAN

គោលដៅ៖

```bash
sudo pacman -S angkorfetch
```

កុំប្រើ AUR command ជំនួសឱ្យ official Pacman repository។

ត្រូវកំណត់ថាតើ package អាចចូល official Arch repositories បានយ៉ាងដូចម្តេច។

រៀបចំ:

* PKGBUILD/package metadata
* version synchronization
* release update process
* validation
* publishing documentation

បើ package មិនទាន់មាន official Arch repository:

* ត្រូវរៀបចំ packaging ឱ្យរួច
* អាចរៀបចំ AUR compatibility ផង
* ប៉ុន្តែកុំសរសេរ `sudo pacman -S angkorfetch` ថា Available មុនពេល package actually exists in a Pacman-accessible repository

AUR និង official Arch repository ត្រូវបង្ហាញជាពីរ status ខុសគ្នា។

---

# 9. FLATPAK / FLATHUB

គោលដៅ៖

```bash
flatpak install flathub <REAL_APP_ID>
```

ត្រូវពិនិត្យ current:

```text
flatpak/
```

និង existing manifest។

ត្រូវធ្វើឱ្យ Flatpak packaging:

* valid
* reproducible
* version synchronized
* CI validated
* ready for Flathub requirements

ជ្រើស App ID មួយដែលមាន format ត្រឹមត្រូវ។

Example format:

```text
org.example.AngkorFetch
```

នេះគ្រាន់តែជា format example, មិនមែន App ID ពិតប្រាកដសម្រាប់ project ទេ។

ត្រូវកំណត់ real App ID ដោយផ្អែកលើ project/package identity ហើយប្រើវាដូចគ្នានៅគ្រប់ files។

កុំ claim Flathub publication មុន package publish ពិតប្រាកដ។

---

# 10. GITHUB RELEASE ARTIFACTS

Review current release pipeline។

Release artifacts ត្រូវមាន naming convention មានស្ថេរភាព។

ត្រូវ ensure:

* version consistency
* OS consistency
* architecture consistency
* checksums
* package artifacts
* archives
* release metadata

Package managers ត្រូវយក source/artifacts ពី release ដែលមាន version និង checksum ដែលអាច verify បាន។

កុំឱ្យ package formula ឬ manifest សរសេរ version មួយ ខណៈ GitHub Release ជា version មួយផ្សេងទៀត។

---

# 11. VERSION SOURCE OF TRUTH

បង្កើត single reliable version source។

Version ត្រូវ synchronize រវាង:

```text
Cargo.toml
Git tag
GitHub Release
WinGet
Homebrew
APT
RPM
Arch
Flatpak
```

កាត់បន្ថយ manual version duplication តាមដែលអាចធ្វើបាន។

Automation គួរធ្វើ version/checksum updates ដោយស្វ័យប្រវត្តិ។

---

# 12. README FOR USERS

README ត្រូវផ្តោតលើ end users មុនគេ។

Recommended structure:

# AngkorFetch

One-sentence description.

## Features

## Installation

## Quick Start

## Usage

## Options

## Supported Platforms

## Update

## Uninstall

## Troubleshooting

## Manual Installation

## Development

## Contributing

## License

Installation ត្រូវនៅខាងលើ។

User មិនគួរត្រូវអាន technical implementation មុនពេលអាច install បានទេ។

---

# 13. USER INSTALLATION TABLE

បង្កើត table ដែលងាយអាន៖

| OS              | Package Manager | Install                                 |
| --------------- | --------------- | --------------------------------------- |
| Windows         | WinGet          | `winget install angkorfetch`            |
| macOS           | Homebrew        | `brew install angkorfetch`              |
| Fedora          | DNF             | `sudo dnf install angkorfetch`          |
| Ubuntu / Debian | APT             | `sudo apt install angkorfetch`          |
| Arch Linux      | Pacman          | `sudo pacman -S angkorfetch`            |
| Linux           | Flatpak         | `flatpak install flathub <REAL_APP_ID>` |

បើ package មួយនៅ pending external publication ត្រូវបង្ហាញ status ជាក់លាក់ក្នុង package-status documentation, មិនមែនផ្តល់ command ក្លែងក្លាយថា working ទេ។

---

# 14. QUICK START

បន្ទាប់ពី install user ត្រូវអាចរត់:

```bash
angkorfetch
```

Documentation ត្រូវប្រើ actual CLI behavior របស់ AngkorFetch។

ពិនិត្យ source code មុនសរសេរ command examples។

Current valid examples គួរត្រូវផ្អែកលើ actual CLI options ដូចជា:

```bash
angkorfetch
angkorfetch -v
angkorfetch --hinfo
angkorfetch -h
```

កុំបង្កើត undocumented options។

---

# 15. UPDATE / UNINSTALL

បង្ហាញ update និង uninstall តាម package manager។

Examples:

```bash
winget upgrade angkorfetch
winget uninstall angkorfetch
```

```bash
brew upgrade angkorfetch
brew uninstall angkorfetch
```

```bash
sudo dnf upgrade angkorfetch
sudo dnf remove angkorfetch
```

```bash
sudo apt update
sudo apt upgrade angkorfetch
sudo apt remove angkorfetch
```

```bash
sudo pacman -Syu angkorfetch
sudo pacman -R angkorfetch
```

Flatpak commands ត្រូវប្រើ actual App ID។

កុំប្រើ package-manager uninstall command សម្រាប់ package ដែល user installed manually, លុះត្រាតែ package manager ជាអ្នក install វា។

---

# 16. MANUAL INSTALLATION

រក្សា manual installation methods ទុក ប៉ុន្តែផ្លាស់ទៅ fallback section៖

## Manual Installation

អាចរួមមាន:

* GitHub Release binary
* `get.sh`
* `get.ps1`
* Cargo
* source build

Package manager ត្រូវនៅជាវិធីសំខាន់។

Manual installation មិនត្រូវធ្វើឱ្យ README មើលទៅដូចជា primary method ទៀតទេ។

---

# 17. CONTRIBUTING.md

បង្កើត ឬធ្វើឱ្យ `CONTRIBUTING.md` ល្អសម្រាប់ external contributors។

វាត្រូវពន្យល់:

## About the project

AngkorFetch ជាអ្វី។

## Development requirements

* Rust stable
* Cargo
* Git

ប្រើ dependencies ពិតប្រាកដរបស់ project។

## Getting started

External contributor អាច clone repository បាននៅទីនេះ។

```bash
git clone https://github.com/AMRSKH/angkorfetch.git
cd angkorfetch
```

## Build

```bash
cargo build
```

ឬ release build:

```bash
cargo build --release
```

## Test

ប្រើ actual test commands របស់ project។

```bash
cargo test --locked
```

## Format

```bash
cargo fmt --check
```

## Lint

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

ប្រើ command នេះតែបើ compatible ជាមួយ current project។

## Coding guidelines

* idiomatic Rust
* simple cross-platform code
* avoid unnecessary dependencies
* preserve graceful failure behavior
* preserve existing platform behavior
* avoid unrelated refactoring

## Pull Requests

Contributor ត្រូវ:

* ពន្យល់ what changed
* ពន្យល់ why
* បញ្ជាក់ affected platforms
* បង្ហាញ tests
* update documentation if behavior changes
* keep PR focused

## Adding a new OS

ពន្យល់ថាតើ platform-specific logic ត្រូវដាក់នៅឯណា។

## Adding a package manager

ពន្យល់:

```text
Package metadata
→ Release integration
→ Validation
→ Publishing
→ Documentation
```

កុំ allow contributor បញ្ចូល installation command ទៅ README មុន package distribution exists។

---

# 18. CONTRIBUTOR DOCUMENTATION STRUCTURE

CONTRIBUTING.md មិនគួររួមបញ្ចូល release details ច្រើនពេក។

បង្កើត links ទៅ:

```text
docs/DEVELOPMENT.md
docs/PACKAGING.md
docs/RELEASING.md
```

បើឯកសារណាមួយមានរួចហើយ ត្រូវ update វាជំនួសការបង្កើត duplicate។

---

# 19. DEVELOPMENT.md

បង្កើត:

```text
docs/DEVELOPMENT.md
```

សម្រាប់ technical development។

រួមមាន:

* prerequisites
* project structure
* build
* test
* formatting
* linting
* platform-specific development
* debugging
* release build
* coding conventions

កុំដាក់ package publishing workflow នៅទីនេះច្រើនពេក។

---

# 20. PACKAGING.md

បង្កើត:

```text
docs/PACKAGING.md
```

សម្រាប់ package maintainers។

មាន sections:

* WinGet
* Homebrew
* DNF/RPM
* APT/DEB
* Arch/Pacman
* Flatpak

សម្រាប់ package នីមួយៗ បង្ហាញ:

* package name
* package ID
* source
* manifest
* version
* checksum
* release dependency
* publishing process
* validation
* install test
* upgrade test
* uninstall test

បែងចែក status:

```text
Published
Pending
Planned
Unsupported
```

---

# 21. RELEASING.md

Update current `RELEASING.md` ឱ្យមាន workflow ច្បាស់៖

```text
Code complete
→ Tests
→ Version bump
→ Release build
→ Checksums
→ Git tag
→ GitHub Release
→ Package updates
→ External package publication
→ Installation verification
→ Documentation verification
```

ត្រូវបែងចែក:

### Automated

អ្វី CI ធ្វើដោយស្វ័យប្រវត្តិ។

### Maintainer action

អ្វី owner ត្រូវ trigger/review។

### External publication

អ្វីត្រូវ submit ទៅ ecosystem ខាងក្រៅ។

កុំអះអាងថា external publication គឺ automated ប្រសិនបើវាមិនមែន។

---

# 22. MAINTAINING.md

បង្កើត:

```text
docs/MAINTAINING.md
```

សម្រាប់ repository owner/maintainer។

ខ្ញុំជា owner ដូច្នេះ workflow ត្រូវផ្តោតលើ:

```text
Make change
→ Test
→ Merge
→ Release
→ Publish packages
→ Verify package managers
→ Update docs
```

កុំប្រាប់ owner ឱ្យ clone repository។

---

# 23. PACKAGE STATUS

README ឬ dedicated documentation ត្រូវមាន package status table ដែល accurate។

Example:

| Ecosystem | Package       | Status | Notes |
| --------- | ------------- | ------ | ----- |
| WinGet    | `angkorfetch` | ...    | ...   |
| Homebrew  | `angkorfetch` | ...    | ...   |
| DNF       | `angkorfetch` | ...    | ...   |
| APT       | `angkorfetch` | ...    | ...   |
| Pacman    | `angkorfetch` | ...    | ...   |
| Flatpak   | `<APP_ID>`    | ...    | ...   |

Status ត្រូវ reflect actual external publication state។

---

# 24. CI/CD

Inspect existing workflows before modifying them។

រក្សា current working automation។

កុំ rewrite CI ដោយគ្មានហេតុផល។

Improve only where needed for:

* release builds
* checksums
* package synchronization
* package validation
* documentation consistency
* version synchronization

Prefer automation over repeated manual editing.

---

# 25. DOCUMENTATION QUALITY

User documentation ត្រូវ:

* simple
* direct
* command-first
* beginner friendly
* technically accurate

Contributor documentation ត្រូវ:

* technical
* reproducible
* focused
* PR-friendly

Maintainer documentation ត្រូវ:

* operational
* release-focused
* package-focused
* automation-focused

កុំ mix these audiences unnecessarily។

---

# 26. REMOVE CONTRADICTORY INSTALLATION DOCUMENTATION

Search entire repository for:

```text
winget install
brew install
apt install
dnf install
pacman
yay
paru
flatpak install
cargo install
get.sh
get.ps1
```

ពិនិត្យគ្រប់ occurrence។

លុប ឬ update instructions ដែល outdated។

ជាពិសេស ត្រូវរក:

* old local `.deb` instructions
* old direct `.rpm` instructions
* old AUR-only instructions
* old tap-only instructions
* obsolete installer commands

Manual methods អាចនៅបាន ប៉ុន្តែត្រូវស្ថិតក្នុង fallback/manual section។

---

# 27. SUPPORTED PLATFORM DOCUMENTATION

Update platform documentation ឱ្យបែងចែកច្បាស់:

### Native package available

### Prebuilt binary available

### Source build available

ឧទាហរណ៍:

```text
Source support
≠
Prebuilt binary support
≠
Package-manager support
```

កុំប្រើ "all Linux distributions" ប្រសិនបើ actual binaries/CI/package repositories មានកំណត់។

---

# 28. SECURITY

Installation scripts និង package repositories ត្រូវប្រុងប្រយ័ត្នជាមួយ:

* SHA256 verification
* HTTPS
* signed repositories where applicable
* trusted release sources
* no arbitrary destructive commands

កុំបន្ថែម installer behavior ដែលមិនចាំបាច់។

---

# 29. ACCEPTANCE CRITERIA

Work is complete only when the repository is prepared for this final installation model:

```text
Windows
winget install angkorfetch
```

```text
macOS
brew install angkorfetch
```

```text
Fedora
sudo dnf install angkorfetch
```

```text
Ubuntu / Debian
sudo apt install angkorfetch
```

```text
Arch Linux
sudo pacman -S angkorfetch
```

```text
Flatpak
flatpak install flathub <REAL_APP_ID>
```

For any command that depends on external registry approval/publication, clearly identify that external step and do not falsely mark it as completed.

---

# 30. FINAL VALIDATION

Run appropriate project checks:

```bash
cargo test --locked
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

Run package-specific validation where possible.

Validate:

* Markdown
* internal links
* package metadata
* release version
* checksums
* CI workflows
* installer scripts
* package commands
* uninstall commands
* upgrade commands
* documentation consistency

Do not claim a command was successfully tested if the current environment cannot test it.

---

# 31. FINAL REPORT

After implementation, provide:

## Changed Files

List every created and modified file.

## User Documentation

Explain what changed for users.

## Contributor Documentation

Explain what changed for contributors.

## Maintainer Documentation

Explain what changed for repository owner/maintainers.

## Package Manager Status

Use:

| OS | Package Manager | Command | Status |
| -- | --------------- | ------- | ------ |

## Package IDs

List actual:

* WinGet ID
* Homebrew formula
* Debian package name
* RPM package name
* Arch package name
* Flatpak App ID

## CI/CD

Explain what was automated.

## Validation

List actual tests/checks performed.

## External Publishing Steps

Clearly separate repository implementation from external approval/publication steps.

## Remaining Work

Only list genuinely incomplete work.

Do not give generic recommendations.

# FINAL RULES

* I am the repository owner.
* Work directly on the existing repository.
* Do not tell me to clone.
* Clone instructions may exist only for external contributors.
* Do not fabricate package availability.
* Do not fabricate package IDs.
* Do not fabricate repositories.
* Do not claim publication before publication.
* Do not replace native package-manager support with local package installation.
* Do not unnecessarily rewrite working Rust code.
* Do not break CI/CD.
* Do not remove useful existing installers unless obsolete.
* Keep user docs simple.
* Keep contributor docs technical.
* Keep maintainer docs operational.
* Keep package docs accurate.
* Keep release docs reproducible.
* No TODO placeholders.
* No fake commands in public documentation.
* No duplicate/conflicting documentation.
* Actually inspect the repository before editing.
* Actually modify the repository, do not only describe what should be changed.
