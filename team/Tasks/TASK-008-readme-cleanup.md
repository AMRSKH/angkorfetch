Author: AI Agent (Claude Code)
Date: 2026-11-11
Task: TASK-008
Status: Completed

# TASK-008: Clean Up README

## Objective

Tidy `README.md`: remove duplicated sections, correct claims that are inaccurate or present
not-yet-live install methods as working, and redact personal data from the example
output, while keeping the current section order and tone.

## Background

The maintainer asked to "update README to clean" and chose (2026-11-11):

- Scope: **Tidy and fix**, not a concise rewrite. Keep section order and tone; roughly
  220 to 260 lines (currently 342).
- Example output: **Redact and update version**. Replace the hardware serial, local IP,
  and Wi-Fi network name with neutral placeholders; set the banner version to the current
  `Cargo.toml` version (v1.2.1). Everything else in the example stays.

Problems found while reading the README before planning:

- `Manual Installation` repeats the quick-install scripts and `cargo install` from
  `Installation`; `Quick Start` overlaps `Usage`.
- `Update` and `Uninstall` list WinGet, Homebrew, DNF, APT, Pacman, and Flatpak commands as
  if they work, but every one of those channels is `[Pending]` or `[Planned]`, and the
  methods that do work (quick-install scripts, `cargo install`) have no update or
  uninstall instructions.
- `Supported Platforms` says Linux musl has no prebuilt binary, yet `release.yml` builds
  `angkorfetch-linux-x86_64-musl.tar.gz`; it also lists Pending or Planned packages as
  native packages.
- The Arch row in `Package Status` uses `sudo pacman -S` while `Installation` uses an AUR
  helper.

## Requirements

- Every install, update, and uninstall method shown as working is actually live; the rest
  stay clearly marked `[Pending]` or `[Planned]` (AGENTS.md section 11).
- No section duplicates another; links to docs replace repeated detail where useful.
- Facts match the repository: release assets in `.github/workflows/release.yml`, install
  paths in `get.sh` / `get.ps1`, flags in `src/main.rs`, package status in `docs/PACKAGING.md`.
- Example output: serial, local IP, and Wi-Fi name replaced with placeholders; banner shows
  the `Cargo.toml` version. No other example values changed.
- Section order and tone kept; result about 220 to 260 lines.
- All relative links resolve to existing files.

## Scope

- `README.md`.

## Out of Scope

- Any change to install scripts, packaging files, workflows, or code.
- Other docs, unless the Docs stage finds one that now contradicts the README.
- Changing the status of any package channel.

## Assigned Roles

- Advisor: `advisor` subagent (Claude Code).
- Dev: `dev` subagent (Claude Code).
- QA Review: `qa-review` subagent (Claude Code), independent of Dev.
- Docs: `docs` subagent (Claude Code).
- Orchestration, final validation, commits, and the final report: main session.

## Implementation Plan

Advisor, 2026-11-11. Every fact below was checked against the repository, not the old README.

### Approach

| Option | Result | Verdict |
|---|---|---|
| A. Fix facts in place and keep every section, including the Pending per-channel subsections and the package-manager Update/Uninstall blocks marked Pending | About 280 lines. Installation still duplicates Package Status. Non-working commands stay in code blocks. | Rejected |
| **B. Tidy and dedupe in the current order**: merge Quick Start + Usage + Options; replace the six Pending/Planned install subsections with one pointer to Package Status; Update/Uninstall cover only methods that work; Manual Installation keeps only what Installation does not already cover | About 235 to 250 lines. No duplication. No non-working command shown as working. | **Recommended** |
| C. Concise rewrite and reorder | About 150 lines | Excluded by the maintainer's scope choice |

Edit only `README.md`. Keep the badges, the bold status markers, the `---` separators between
sections, and the current wording wherever this plan does not ask for a change. Do not rename the
headings `Package Status & Availability` or `Manual Installation`. New in-page links use the
anchors `#package-status--availability` and `#manual-installation`.

### Section-by-section plan (resulting order)

**1. Title, badges, tagline (lines 1-9): keep unchanged.** Every badge target exists:
`release.yml` and `LICENSE`.

**2. Features: keep, but correct two bullets.**
- Direct OS query: delete "Nothing is guessed." On Windows, `get_display` falls back to 60 Hz when
  GDI returns no refresh rate (`src/main.rs`, `if r > 0 { r } else { 60 }`).
- Terminal-adaptive design: the tiers are `Full`, `Compact`, and `None`, and the tier is chosen
  by terminal width (`logo_tier(term_width())`). There is no "minimal logo". Truecolor is chosen
  from the environment (`COLORTERM`, `WT_SESSION`, `TERM_PROGRAM`, `TERM`), with a 16-colour
  fallback. Suggested wording: "Picks a full, compact, or no logo to fit the terminal width,
  and a 24-bit truecolor or 16-color gradient depending on the terminal."
- Keep the other two bullets. The three runtime crates match `Cargo.toml`.

**3. Installation: rewrite the note, keep the quick-install and Cargo subsections, and replace
the six Pending/Planned subsections.**
- Note under the heading: "The quick-install scripts and `cargo install` work today.
  Package-manager channels are not published yet; their status is in
  [Package Status](#package-status--availability)."
- `### Quick install (recommended — works today, no Rust required)`: keep the heading and both
  code blocks exactly as they are. Replace the two italic notes with these facts:
  - Windows (`get.ps1`): downloads the latest release and checks its SHA256 against
    `checksums.txt`. Installs to `$env:LOCALAPPDATA\AngkorFetch\bin` and adds that folder to
    the **user** PATH. On ARM64 it installs the x86_64 build, which runs under emulation. If
    `angkorfetch` is not recognised, open a new terminal.
  - macOS and Linux (`get.sh`): same SHA256 check. Installs to `~/.local/bin/angkorfetch`. If
    that folder is not on PATH, the script appends an `export PATH=...` line to the shell
    profile. On musl x86_64 systems (for example Alpine) it picks the musl build
    automatically; `bash`, `curl`, and `tar` must be installed first. There is no prebuilt
    binary for musl on aarch64, so the script stops and points to `cargo install angkorfetch`.
- `### Via Cargo (any platform with Rust installed)`: keep the code block. Add: "Builds from
  source and installs to `~/.cargo/bin`."
- Replace the Homebrew, WinGet, DNF, APT, AUR, and Flatpak subsections (lines 43-77) with:
  `### Package managers`, followed by "WinGet, Homebrew, APT, DNF, AUR, Flathub, and Snap
  packages are **`[Pending]`** or **`[Planned]`**. See
  [Package Status](#package-status--availability). Until they are live, use a quick-install
  script, `cargo install`, or the `.deb` / `.rpm` file from
  [Manual Installation](#manual-installation)."
  Each channel-specific note (the WinGet rejection, homebrew-core, the PKGBUILD path, the
  Flatpak manifest) moves into the table's Notes column below. Nothing is lost.

**4. Package Status & Availability: keep the heading and the columns, fix the rows.** Use this
exact table. Order: Live first, then Built, then Pending, then Planned. Status values are
unchanged from today's README; the Snap row is added to match `docs/PACKAGING.md`.

```markdown
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
```

Reasons: the Built rows previously showed `sudo dnf install angkorfetch` and
`sudo apt install angkorfetch`, which do not work without a repository. The Arch row used
`sudo pacman -S`, while Installation used an AUR helper.

**5. Quick Start + Usage + Options: merge into one `## Usage` section, placed where Quick Start
is now.** Content:
- "Run with no arguments to print the system summary:" followed by a `bash` block containing
  only `angkorfetch`.
- Then the Options table, keeping the existing `Flag | Long Flag | Description` layout:
  - `-h` | `--help` | Print usage and the option list, then exit
  - `-v` | `--version` | Print the logo and version banner, then exit
  - (empty) | `--hinfo`, `--hard` | Print the logo, then the detailed hardware report:
    Motherboard, BIOS, Serial, CPU, GPU, Memory, RAM (type, speed, vendor), Disk, Disk Model,
    Disk Type, Display, Ports, WiFi, Battery
- Source: `main()` and `print_help()` in `src/main.rs`. `-v` calls `print_logo_banner()`; it
  prints no build information, so "Display current version and build information" was wrong.
  Do not document `/?` (accepted by `main()`, but not listed in `--help`) or flag precedence.

**6. Example Output: keep, changing exactly four lines inside the code blocks (plus one
optional caption tweak).**

| Line now | Change to |
|---|---|
| `║ AngkorFetch v1.1.1  •  …  ║` | `v1.2.1` (same length; all three box lines stay 73 columns) |
| ` ● Local IP   ❯ 192.168.0.208 (Wi-Fi)` | ` ● Local IP   ❯ 192.168.x.x (Wi-Fi)` (`Wi-Fi` is the interface name from `get_ip`, so keep it) |
| ` ● Serial       ❯ GXCGRV2` | ` ● Serial       ❯ XXXXXXX` |
| ` ● WiFi         ❯ TP-Link_58AC_5G (83%)` | ` ● WiFi         ❯ MyNetwork (83%)` |

Keep everything else, including Host `DELL` and RAM `(0080000080AD)`. The RAM value is the
module's manufacturer code, not personal data. The real `--hinfo` output starts with the logo
banner (`print_hardware_info` calls `print_logo_banner`), so change the caption to "Detailed
hardware output (`angkorfetch --hinfo`, logo omitted):". Do not touch the box-drawing art.

**7. Supported Platforms: keep the section and the columns. Fix the rows.** Change the intro
sentence to "Support is categorized into **Prebuilt Binaries**, **Packages**, and
**Source Builds**:" and rename the column `Native Package` to `Package`. Rows:

| Operating System | Architecture | Prebuilt Binary | Package | Implementation Details |
|---|---|---|---|---|
| **Windows 10 & 11** | x86_64 | Yes | WinGet (Pending) | Registry, CIM/WMI, GDI |
| **Windows on ARM** | aarch64 | x64 build via emulation | No | Native build from source via `cargo install` (untested in CI) |
| **Linux (glibc)** | x86_64 | Yes | `.deb`, `.rpm` in Releases | `/sys`, DMI, `lspci`, `dmidecode` |
| **Linux (glibc)** | aarch64 | Yes | No | `/sys`, DMI, `lspci`, `dmidecode` |
| **Linux (musl / Alpine)** | x86_64 | Yes | No | Same as glibc Linux |
| **Linux (musl / Alpine)** | aarch64 | No | No | Build from source via `cargo install` |
| **macOS (Intel)** | x86_64 | Yes | Homebrew (Planned) | `sysctl`, `system_profiler` |
| **macOS (Apple Silicon)** | aarch64 | Yes | Homebrew (Planned) | `sysctl`, `system_profiler` |
| **BSD (FreeBSD, OpenBSD)** | Any | No | No | Build from source; untested; only `sysinfo` fields populated |

Sources: the `build` matrix and the `Verify all expected artifacts` step in `release.yml`
(six archives, including `angkorfetch-linux-x86_64-musl.tar.gz`); the `packages` job (deb
`amd64` and rpm `x86_64` only); and the `get.sh` and `get.ps1` architecture switches. The Linux
data sources are the same code path on every architecture.

**8. Update: rewrite. Cover only methods that work.**

```markdown
| Installed with | How to update |
| --- | --- |
| Quick-install script | Re-run the same install command. It fetches the latest release and overwrites the binary; the PATH change is not repeated. |
| Cargo | `cargo install angkorfetch` (installs a newer crates.io version if there is one) |
| `.deb` / `.rpm` from Releases | Download the newer file and install it as in [Manual Installation](#manual-installation) |
```

Follow the table with: "Package-manager commands will be listed here once those channels are
live." Delete all the WinGet, Homebrew, DNF, APT, Pacman, and Flatpak commands.
Sources: `get.sh` (`cp` overwrites; the PATH line is guarded by `grep -qF`); `get.ps1`
(`Copy-Item -Force`; PATH uses an exact-entry `-notcontains` check); `cargo install` replaces an
installed crate when a newer version exists.

**9. Uninstall: rewrite. Cover only methods that work.**
- A `powershell` block headed `# Windows, quick-install script` containing
  `Remove-Item -Recurse -Force "$env:LOCALAPPDATA\AngkorFetch"`. Then one sentence: "Then remove
  `%LOCALAPPDATA%\AngkorFetch\bin` from your user `Path` (Start menu: *Edit environment variables
  for your account*)." Use the GUI step, not a scripted PATH rewrite; see Risks.
- A `bash` block containing:
  - `# macOS / Linux, quick-install script` then `rm -f ~/.local/bin/angkorfetch`
  - `# Cargo (any OS)` then `cargo uninstall angkorfetch`
  - `# Debian / Ubuntu (.deb from Releases)` then `sudo apt remove angkorfetch`
  - `# Fedora / RHEL (.rpm from Releases)` then `sudo dnf remove angkorfetch`
- One closing sentence: "`get.sh` may have added an `export PATH=` line for `.local/bin` to
  `~/.bashrc`, `~/.zshrc`, `~/.zprofile`, or `~/.bash_profile`; remove it only if nothing else
  uses `~/.local/bin`."
- The apt and dnf removals are valid because the release `.deb` and `.rpm` register the package
  name `angkorfetch` (see the `packages` job). They do not depend on a Pending repository.

**10. Troubleshooting: keep three bullets and rewrite Wayland.**
- Linux root privileges: keep as is. It is accurate: `get_serial` reads
  `/sys/class/dmi/id/product_serial`, and RAM type, speed, and vendor come from `dmidecode`.
- Wayland: the current claim is wrong. `get_display` tries `xrandr`, then `wlr-randr`, then
  `/sys/class/drm/*/modes`. New text: "Display detection tries `xrandr`, then `wlr-randr`, then
  the kernel DRM mode list in `/sys/class/drm`. Without XWayland or `wlr-randr` the value comes
  from DRM (resolution only, no refresh rate). It reports `Unknown` only if all three fail."
- WSL and Confinement: keep as is.

**11. Manual Installation: keep the heading and the position. Keep only content that is not
in Installation.**
- Intro: "Every [GitHub Release](https://github.com/AMRSKH/angkorfetch/releases/latest)
  includes:" followed by this table:

| File | For |
|---|---|
| `angkorfetch-windows-x86_64.zip` | Windows x86_64 (`angkorfetch.exe`) |
| `angkorfetch-macos-x86_64.tar.gz` | macOS Intel |
| `angkorfetch-macos-aarch64.tar.gz` | macOS Apple Silicon |
| `angkorfetch-linux-x86_64.tar.gz` | Linux x86_64 (glibc) |
| `angkorfetch-linux-x86_64-musl.tar.gz` | Linux x86_64 (musl, e.g. Alpine) |
| `angkorfetch-linux-aarch64.tar.gz` | Linux aarch64 (glibc) |
| `angkorfetch_<version>_amd64.deb` | Debian / Ubuntu x86_64 |
| `angkorfetch-<version>-1.x86_64.rpm` | Fedora / RHEL x86_64 |
| `checksums.txt` | SHA256 of every file above |

- A `bash` block, run from the folder the file was downloaded to:
  - `# Debian / Ubuntu` then `sudo apt install ./angkorfetch_*_amd64.deb`
  - `# Fedora / RHEL` then `sudo dnf install ./angkorfetch-*.x86_64.rpm`
- One sentence for archives: "For an archive, extract it and put `angkorfetch` (or
  `angkorfetch.exe`) in a folder on your PATH. On Linux you can check the download with
  `sha256sum -c --ignore-missing checksums.txt`."
- Delete the `VER=$(curl … | grep -o 'tag/v…')` snippets. They scrape GitHub's HTML, and the
  repository cannot verify that. The globs above need no version lookup and do not depend on
  the rpm `%{?dist}` suffix.
- Delete `### Automated Quick Install Scripts` and `### Via Cargo`; they repeat Installation.
- `### Build From Source`: keep unchanged.

**12. Development, Contributing, License: keep unchanged.** All link targets exist:
`docs/DEVELOPMENT.md`, `docs/PACKAGING.md`, `docs/RELEASING.md`, `docs/MAINTAINING.md`,
`CONTRIBUTING.md`, `LICENSE`.

Expected length is about 235 to 250 lines. If the result exceeds 260, shorten prose notes
first; do not drop facts listed above.

### Contradictions found (README versus repository)

| README claim | Repository fact | Source |
|---|---|---|
| Supported Platforms: musl has no prebuilt binary | `angkorfetch-linux-x86_64-musl.tar.gz` is built and required by the release verify step; v1.2.1 includes it (commit `a15d0f5` is in tag `v1.2.1`, and the bot branch `automation/sync-packages-v1.2.1` shows the release completed) | `release.yml`, `get.sh` |
| Supported Platforms: Linux x86_64 packages "DEB, RPM, Flatpak, Arch"; aarch64 "Arch, Flatpak, Homebrew"; macOS "Homebrew" | Only the `.deb`/`.rpm` release files exist, for x86_64 only. Flatpak, Arch, and Homebrew are Planned. | `release.yml` `packages`, `docs/PACKAGING.md` |
| Update/Uninstall list WinGet, Homebrew, DNF, APT, Pacman, and Flatpak commands as working | None of these channels is live; the quick-install and Cargo paths are not covered | `docs/PACKAGING.md`, `get.sh`, `get.ps1` |
| Package Status Built rows use `sudo dnf/apt install angkorfetch` | No repository exists; only a local-file install works | `docs/PACKAGING.md` |
| Package Status Arch uses `sudo pacman -S`; Installation uses `yay`/`paru` | The planned channel is the AUR | `docs/PACKAGING.md` section 5 |
| `-v` shows "version and build information" | Prints the logo and the version banner only | `main()`, `print_logo_banner()` |
| Wayland: Display reports `Unknown` without XWayland | Falls back to `wlr-randr`, then `/sys/class/drm` | `get_display()` |
| Features: "minimal logos", "Nothing is guessed" | Tiers are Full / Compact / None; Windows refresh rate defaults to 60 Hz | `logo_tier()`, `get_display()` |
| `--hinfo` example has no logo | Real output starts with the logo banner | `print_hardware_info()` |

### Files to change

- `README.md` only. No code, scripts, workflows, manifests, or other docs.

### Risks and mitigations

| Risk | Mitigation |
|---|---|
| Trimming drifts toward a rewrite and changes the tone | Keep the headings, bold status markers, `---` separators, and existing sentences wherever this plan does not ask for a change |
| Broken in-page links | Do not rename the two linked headings; QA checks the anchors (test 2) |
| Banner box misaligned after the version edit | `1.1.1` and `1.2.1` have the same length; QA checks that lines 138-140 (or their new positions) still have equal width |
| Release asset names cannot be checked offline (rpm `%{?dist}`, actual v1.2.1 assets) | Commands use globs; the file table uses the pattern from `release.yml` and `docs/RELEASING.md`. QA may compare with the v1.2.1 Releases page if network access is available. |
| crates.io version may lag behind 1.2.1 (no publish step exists in the repo) | Keep the existing **Live** status; out of scope to verify; recorded in Notes |
| A scripted Windows PATH rewrite would rewrite the user `Path` (and could flatten `REG_EXPAND_SZ` entries) | Document the GUI removal step instead |
| Removing the `.local/bin` PATH line breaks other tools | Wording says to remove it only if nothing else uses `~/.local/bin` |
| README ships inside the `.deb` (`/usr/share/doc/angkorfetch/`) | No impact until the next release; contains no secrets |

### Test plan

1. `wc -l README.md`: between 220 and 260.
2. Links: every relative `](...)` target that is not `http` or `#` exists (`test -e`). The in-page
   anchors `#package-status--availability` and `#manual-installation` match headings
   `## Package Status & Availability` and `## Manual Installation`.
3. Redaction: `grep -nE "GXCGRV2|192\.168\.0\.208|TP-Link|v1\.1\.1" README.md` returns nothing;
   `grep -n "AngkorFetch v1.2.1" README.md` returns the banner line, which matches `Cargo.toml`.
4. Example diff: `git diff README.md` inside the two `text` blocks shows only the four lines in
   section 6 (plus the caption); the three banner box lines have equal character counts.
5. No non-live command shown as working: `grep -nE "winget |brew |pacman|yay |flatpak |snap install|dnf install angkorfetch|apt install angkorfetch" README.md`
   matches only Package Status rows marked Pending or Planned.
6. Flags: `cargo run -- -h` output matches the Usage table; `cargo run -- -v` prints
   `AngkorFetch v1.2.1`.
7. Assets: the Manual Installation table matches the `asset:` lines and the verify list in
   `.github/workflows/release.yml`.
8. Install paths: `get.sh` `INSTALL_DIR="$HOME/.local/bin"` and `get.ps1`
   `Join-Path $env:LOCALAPPDATA "AngkorFetch\bin"` match the README text.
9. `git diff --stat`: only `README.md` (plus this task file).
10. No Rust, script, or workflow file changes, so `cargo fmt`, Clippy, `cargo test`, and the
    script tests are not required. Running them is harmless.

### Approvals required

None for the plan or the local work commit. Push and PR need maintainer approval (AGENTS.md
section 7).

### Implementation (Dev, 2026-11-11)

Option B applied to `README.md` only. Every plan fact was re-checked against `get.sh`,
`get.ps1`, `.github/workflows/release.yml`, `src/main.rs`, and `Cargo.toml` before editing.

| Plan section | Done |
|---|---|
| 1. Title, badges, tagline | Unchanged |
| 2. Features | Deleted "Nothing is guessed."; terminal bullet replaced with the suggested wording (full / compact / no logo by width; truecolor or 16-color). Other bullets unchanged |
| 3. Installation | New blockquote note; quick-install heading and both code blocks unchanged; italic notes replaced by the `get.ps1` / `get.sh` facts; Cargo note added; six Pending/Planned subsections replaced by `### Package managers` with the plan's text |
| 4. Package Status | Exact table from the plan (Live, Built, Pending, Planned; Snap row added) |
| 5. Usage | Quick Start, Usage, and Options merged into one `## Usage` at the Quick Start position: one-line intro, `angkorfetch` block, Options table with the plan's three rows |
| 6. Example Output | Exactly four lines changed (banner `v1.2.1`, `192.168.x.x`, `XXXXXXX`, `MyNetwork`) plus the caption "logo omitted". Box art untouched |
| 7. Supported Platforms | Intro sentence and `Package` column renamed; rows from the plan (one wording change, see Divergences) |
| 8. Update | Plan table plus the closing sentence; all package-manager commands removed |
| 9. Uninstall | PowerShell block, GUI PATH sentence, bash block (quick-install, Cargo, apt, dnf), closing `export PATH` sentence |
| 10. Troubleshooting | Wayland bullet rewritten with the plan's text; other three bullets unchanged |
| 11. Manual Installation | Heading kept; intro + nine-row asset table; `.deb` / `.rpm` glob install block; archive and `sha256sum` sentence; `VER=$(curl ...)` snippets, Tip, quick-install and Cargo repeats deleted; `### Build From Source` unchanged |
| 12. Development, Contributing, License | Unchanged |

Divergence from the plan (fact):

- **BSD row, Implementation Details.** Plan text: "only `sysinfo` fields populated". Source:
  on any non-Windows OS, `get_shell` reads `SHELL`, `get_terminal` reads `TERM_PROGRAM` /
  `TERM`, `get_de` reads `XDG_CURRENT_DESKTOP` / `DESKTOP_SESSION` / `GDMSESSION`, and
  `get_arch` runs `uname -m`, so more than `sysinfo` fields are populated on BSD. Written as
  "Build from source; untested; only cross-platform fields (`sysinfo`, environment
  variables) populated".

Layout note (no fact dropped): a first full pass with an extra `### Options` subheading came to
266 lines. Following the plan's rule ("shorten prose notes first"), the Usage section has no
`### Options` subheading, the `### Direct GitHub Release Packages` subheading was dropped (the
plan's section 11 does not list it), and the archive sentence was merged with the `.deb` / `.rpm`
lead-in into one paragraph placed before the install block (the plan puts the archive sentence
after it). Result: 260 lines.

Files changed: `README.md` (73 insertions, 155 deletions). This task file.

### Dev fixes for QA Round 1 (2026-11-11)

Each finding was checked against the source before the fix. README line numbers below are
after the fix.

| QA finding | Source check | Fix |
|---|---|---|
| Medium, `sudo angkorfetch --hinfo` | `get.sh:130` installs to `~/.local/bin`, Cargo to `~/.cargo/bin`; neither is in the default sudo `secure_path` on Debian/Ubuntu or Fedora/RHEL (from the distributions' default sudoers; not run on Linux here) | Line 196: `For full diagnostics run \`sudo "$(command -v angkorfetch)" --hinfo\`; the full path is needed because \`sudo\` usually does not search \`~/.local/bin\` or \`~/.cargo/bin\`.` The shell expands `command -v` before `sudo` runs, so it also works for `/usr/bin` (deb/rpm). The command is now inline instead of a nested code block (saves 3 lines) |
| Low, line 46 APT/DNF called Pending/Planned | Package Status rows 55-56 say **Built** | "WinGet, Homebrew, AUR, Flathub, and Snap packages are **`[Pending]`** or **`[Planned]`**, and the APT and DNF repositories are not published yet." |
| Low, literal `%LOCALAPPDATA%` | `get.ps1:99` builds `$InstallDir` with `Join-Path $env:LOCALAPPDATA ...` and line 112 writes the expanded string with `SetEnvironmentVariable` | Line 174: "remove the entry ending in `AppData\Local\AngkorFetch\bin` (for example `C:\Users\<you>\AppData\Local\AngkorFetch\bin`)" |
| Low, "Every GitHub Release includes" | `git tag --contains a15d0f5` returns only `v1.2.1`; `x86_64-unknown-linux-musl` appears only in `v1.2.1:.github/workflows/release.yml` | Line 205: "The [latest GitHub Release](...) includes:" |
| Low, `sha256sum -c --ignore-missing` is GNU-only | `--ignore-missing` is a GNU coreutils option; BusyBox `sha256sum` has no such flag | Replaced with `grep " $F\$" checksums.txt \| sha256sum -c -` (the same `grep " <name>$"` lookup `get.sh:102` uses). Tested with GNU `sha256sum` on mock files in `sha256sum -t` format (the Linux runner's default): correct file `OK` exit 0; tampered file `FAILED` exit 1. BusyBox not available here, so not tested |
| Low, no command-line download step | Assets are served from `releases/latest/download/<file>` (`get.sh:15`, `get.ps1:19`); `checksums.txt` lists every artifact, deb and rpm included (`checksums` job runs `sha256sum *` over the merged artifacts) | One copy-paste block (lines 221-228, lead-in at 219): set `URL`, download `checksums.txt`, read the versioned `.deb` name from it (rpm pattern in the comment), download, check, install by exact name. Archives: set `F` to the table name and stop after the check |
| Low, update path reuses globs | Old block used `./angkorfetch_*_amd64.deb` and `./angkorfetch-*.x86_64.rpm` | Manual Installation now installs `"./$F"`, an exact name; Update row (line 161) says the steps "use the exact file name, so an older file in the folder is not picked up". The `grep -oE` lookup returns one name because `checksums.txt` lists one deb and one rpm per release |
| Low, Package column mixes available and planned | `linux/arch/PKGBUILD:6` `arch=('x86_64' 'aarch64')`; `Formula/angkorfetch.rb` `on_linux` `on_arm` / `on_intel` point to the glibc tarballs; `flatpak/` and `snap/` build from source with no architecture restriction, on glibc-based bases (`org.freedesktop.Platform`, `core22`); WinGet installer manifest declares `x64` only | Rule stated in the intro: "**Packages** (release files, plus Pending or Planned channels whose manifest targets that platform)". Linux glibc x86_64: "`.deb`, `.rpm` (Built); AUR, Flathub, Snap, Homebrew (Planned)"; glibc aarch64: "AUR, Flathub, Snap, Homebrew (Planned)". Windows on ARM and both musl rows stay "No" (no manifest targets them). Statuses unchanged |
| Low, BSD list incomplete | `get_arch` runs `uname -m`, `get_gpu_usage` runs `nvidia-smi` on every OS | Line 151: "Build from source; untested; only cross-platform fields populated" (no list of sources) |

Not changed: the Package Status **Built** rows keep the plan's exact glob commands
(`sudo apt install ./angkorfetch_*_amd64.deb`, `sudo dnf install ./angkorfetch-*.x86_64.rpm`).
QA did not flag them, and they are correct for a single downloaded file.

Length: the fixes first came to 263 lines; inlining the Troubleshooting `sudo` command brought
it back to **260**. No fact dropped.

Files changed: `README.md` (77 insertions, 159 deletions against `HEAD`). This task file.

### Dev fixes for QA Round 2 (2026-11-11)

| QA finding | Source check | Fix |
|---|---|---|
| Medium, README.md:226-227, the check did not gate the install | Without `pipefail` the pipeline's status is that of `sha256sum -c -`, which exits non-zero on `FAILED` and also when `grep` finds no line (empty input: "no properly formatted SHA256 checksum lines found") | Check and install are one line: `grep " $F\$" checksums.txt \| sha256sum -c - && sudo apt install "./$F"    # .rpm: ... && sudo dnf install "./$F"` (README.md:226). Lead-in (219) now reads "The install runs only if `sha256sum` prints `OK`". Archive sentence (229): replace the install command after `&&` with an extract command (for example `tar xzf "$F"`), "so it runs only if the check prints `OK`" |
| Low, README.md:147-148, musl rows vs the Package rule | `flatpak/io.github.AMRSKH.angkorfetch.yml` ships its own runtime (`runtime: org.freedesktop.Platform`, `sdk: org.freedesktop.Sdk`) and has no `only-arches`, so the app does not depend on the host libc or one architecture. `snap/snapcraft.yaml` has `base: core22` and needs `snapd` on the host; Alpine does not ship `snapd` (external fact, not verified offline) | Both musl rows: Package "Flathub (Planned)". The rule at line 139 is unchanged. Snap stays off the musl rows. AUR (Arch, glibc) and Homebrew (`on_linux` URLs point to the glibc tarballs) also stay off |

Length: 259 lines (the check and install now share one line).

## Dependencies

- Sources of truth used: `.github/workflows/release.yml` (build matrix, verify step,
  `packages` job), `get.sh`, `get.ps1`, `src/main.rs` (`main`, `print_help`,
  `print_logo_banner`, `print_hardware_info`, `get_display`, `get_ip`, `get_wifi`,
  `get_serial`), `Cargo.toml` (version 1.2.1), `docs/PACKAGING.md`, `Formula/`, `winget-pkgs/`,
  `linux/`, `flatpak/`, `snap/`, and git history (`c40983a`, `d079c68`, tag `v1.2.1`, remote
  branch `automation/sync-packages-v1.2.1` at `21ac68f`).
- README channel statuses follow the maintainer's latest explicit decision (`c40983a`,
  2026-11-02): Homebrew is "Planned (homebrew-core)" and the tap was removed from the
  instructions; WinGet is Pending because the first submission was rejected. Changing a status
  is out of scope, so README keeps these statuses even where `docs/PACKAGING.md` disagrees.
- `docs/PACKAGING.md` inconsistencies, recorded and **not** to be fixed in this task:
  1. Homebrew is "Published (Tap)" with `brew install AMRSKH/tap/angkorfetch` (and
     `Formula/README.md` agrees), while the README has said "Planned (homebrew-core)" since
     `c40983a`. Its own overview row also pairs "Published (Tap)" with the untapped command
     `brew install angkorfetch`, which works only after homebrew-core acceptance.
  2. WinGet says "Pending community PR approval at #409790"; README says the submission was
     rejected and will be resubmitted. The overview uses `winget install angkorfetch` (moniker);
     README uses the ID `AMRSKH.AngkorFetch`.
  3. Arch overview row uses `sudo pacman -S` (official-repo stage) while the planned channel is
     the AUR (`yay`/`paru`, section 5).
  4. crates.io is not listed at all, although README and AGENTS.md treat it as live.
  5. Version-pinned examples use 1.1.1 (`winget validate`, `wingetcreate`, deb/rpm test files).
     The current version is 1.2.1.
  6. DEB/RPM "Upgrade" test commands (`apt upgrade`, `dnf upgrade angkorfetch`) cannot fetch a
     newer version while the package exists only as a release file.
- Other stale docs, recorded only: `docs/RELEASING.md` lists five binaries and "8 artifacts"
  (actually six archives plus deb, rpm, and checksums, so nine) and calls the glibc tarball a
  "static binary"; `docs/MAINTAINING.md` (line 102) and root `RELEASING.md` (line 70) say five
  targets; `linux/arch/PKGBUILD` and `snap/snapcraft.yaml` pin 1.1.1; `Formula/` and
  `winget-pkgs/` on this branch are still 1.1.1 because the v1.2.1 sync branch was never merged.
- No dependency on other tasks.

## Acceptance Criteria

- [x] Duplicate sections removed or merged
- [x] No Pending or Planned channel shown as working in any section
- [x] Working update and uninstall paths documented for the quick-install scripts and Cargo
- [x] `Supported Platforms` and `Package Status` match `release.yml` and the channel statuses the README already uses (maintainer decision `c40983a`; `docs/PACKAGING.md` disagrees in places, see Dependencies)
- [x] Example output redacted (serial, IP, Wi-Fi) and banner version matches `Cargo.toml`
- [x] Section order and tone kept; length about 220 to 260 lines
- [x] All relative links resolve
- [x] Independent QA accepted (code Round 3 `Accepted` after two `Changes requested` rounds; docs Round 4 `Accepted`)

## Testing Requirements

- Every relative link in `README.md` points to an existing file.
- Every command shown as working is checked against its source (script, workflow, code).
- `git diff --stat` shows only the files in scope.

### Dev results (2026-11-11, Windows 11, Git Bash + Python 3.14)

| # | Check | Command | Result |
|---|---|---|---|
| 1 | Length | `wc -l README.md` | **260** (was 342). Pass (limit 260 inclusive). File keeps CRLF in the working tree (260 CR), same as before |
| 2 | Links | Extract every `](...)` target; `test -e` on non-`http`, non-`#` ones | `CONTRIBUTING.md`, `LICENSE`, `docs/DEVELOPMENT.md`, `docs/MAINTAINING.md`, `docs/PACKAGING.md`, `docs/RELEASING.md`: all exist. Anchors used: `#package-status--availability` (2x), `#manual-installation` (2x); headings present at lines 50 and 206 with unchanged text. Pass |
| 3 | Redaction | `grep -nE "GXCGRV2\|192\.168\.0\.208\|TP-Link\|v1\.1\.1" README.md` | No output (exit 1). `grep -n "AngkorFetch v1.2.1"` returns line 94 (banner); `Cargo.toml` line 3 `version = "1.2.1"`. Pass |
| 4 | Example diff | Python: compare the two `text` blocks of `HEAD:README.md` with the working tree | Block line counts unchanged (28 and 14). Exactly 4 lines differ: banner `v1.1.1` to `v1.2.1`, Local IP, Serial, WiFi. Caption changed outside the blocks. Banner box lines 93-95: 73, 73, 73 characters. `git diff --check README.md`: clean. Pass |
| 5 | No non-live command as working | `grep -nE "winget \|brew \|pacman\|yay \|flatpak \|snap install\|dnf install angkorfetch\|apt install angkorfetch" README.md` | Matches lines 57-61 (Package Status rows: WinGet Pending, Homebrew / AUR / Flathub / Snap Planned) and lines 149-150 (`Homebrew (Planned)` in Supported Platforms, a status label that matches `brew `, not a command). Pass |
| 6 | Flags | `cargo run -q -- -h`; `cargo run -q -- -v` | `-h` (exit 0) prints `Usage: ... [OPTIONS]` and the three options `-h, --help`, `-v, --version`, `--hinfo, --hard`, matching the Usage table. `-v` (exit 0) prints the logo and the box with `AngkorFetch v1.2.1`, no build information. `--hinfo` field list checked against `print_hardware_info` entries (14 fields, same order). Pass |
| 7 | Assets | `asset:` lines and verify step in `release.yml` vs Manual Installation table | Six archives identical in both (`linux-x86_64`, `linux-x86_64-musl`, `linux-aarch64`, `macos-x86_64`, `macos-aarch64`, `windows-x86_64.zip`); verify list also requires `checksums.txt`, `*.deb`, `*.rpm`. Deb name `angkorfetch_${VER}_amd64.deb` (line 255); rpm `Release: 1%{?dist}`, copied from `RPMS/x86_64/` (lines 266, 279). Pass (rpm `%{?dist}` suffix not verifiable offline; the commands use globs) |
| 8 | Install paths | `grep` in `get.sh` / `get.ps1` | `get.sh:130 INSTALL_DIR="$HOME/.local/bin"`; `get.ps1:99 $InstallDir = Join-Path $env:LOCALAPPDATA "AngkorFetch\bin"`. README lines 30, 36, 171, 178 match. Pass |
| 9 | Scope | `git diff --stat`; `git status --short` | `README.md \| 228 (73 insertions, 155 deletions)`, 1 file changed; status shows `M README.md` and `?? team/Tasks/TASK-008-readme-cleanup.md` only. Pass |
| 10 | Rust / script checks | not run | No Rust, script, or workflow file changed (plan step 10). `cargo run` in step 6 compiled the debug build without errors |

Other source checks done while editing: `get.sh` musl x86_64 / aarch64 branches (lines 49-63),
SHA256 verification in both scripts, `get.ps1` ARM64 fallback (lines 29-33), PATH guards
(`get.sh` `grep -qF`, `get.ps1` `-notcontains`), `get_display` fallback order (xrandr, xrandr
connected line, `wlr-randr`, `/sys/class/drm/*/modes`), Windows 60 Hz default (`main.rs` line
805), `supports_truecolor` environment checks, `logo_tier` (Full / Compact / None).

Not verified: actual asset names on the v1.2.1 Releases page, crates.io's current version, and
the live state of any package channel (no network checks run). Rendering on GitHub not previewed.

### Dev re-run after QA Round 1 fixes (2026-11-11)

| # | Command | Result |
|---|---|---|
| 1 | `wc -l README.md` | **260**; 260 CR (CRLF working tree, unchanged). Pass |
| 2 | Link extraction + `test -e`; anchor and heading grep | 6 relative targets exist; `#package-status--availability` (2x) and `#manual-installation` (2x); headings at lines 50 and 203. Pass |
| 3 | Redaction grep; `grep -n "AngkorFetch v1.2.1"` | No match (exit 1); banner at line 94. Pass |
| 4 | Python diff of the two `text` blocks vs `HEAD` | Lengths 28 and 14 (unchanged); 4 changed lines; banner 73/73/73 characters. Pass |
| 5 | `grep -nE "winget \|brew \|pacman\|yay \|paru\|flatpak \|snap install\|dnf install angkorfetch\|apt install angkorfetch\|upgrade\|AMRSKH/tap\|ignore-missing\|%LOCALAPPDATA%\|sudo angkorfetch\|Every \[GitHub" README.md` | Lines 57-61 (Pending/Planned Package Status rows) and 145-146, 149-150 (`Homebrew (Planned)` labels in Supported Platforms). No `ignore-missing`, `%LOCALAPPDATA%`, `sudo angkorfetch`, or "Every [GitHub". Pass |
| 6 | Flags | Not re-run; Usage table unchanged since the first run |
| 7 | Asset table vs `release.yml` | Table unchanged; 6 archive rows. Pass |
| 8 | Install-path lines | README 30, 36, 171, 174, 178 match `get.sh:130` and `get.ps1:99`. Pass |
| 9 | `git diff --stat`; `git status --short`; `git diff --check README.md` | `README.md` only (77+, 159-); untracked task file; diff check clean. Pass |
| - | Mock checksum test (scratchpad, GNU `sha256sum -t` format) | deb, rpm, and archive lookups each give `OK` (exit 0); tampered deb gives `FAILED` (exit 1); the deb `grep -oE` returns exactly 1 name |

Not verified: BusyBox `sha256sum -c -` on Alpine, the sudo `secure_path` behaviour on a real
Linux host, and the Snap and Flathub aarch64 builds (the manifests do not restrict the
architecture, but no store build was checked).

### Dev re-run after QA Round 2 fixes (2026-11-11)

The README download block (lines 222-226) was extracted with `awk` into a scratch file
(`block.sh`). Two variants were built from it with exact string replacement: `rpm.sh` (the
commented rpm pattern and `sudo dnf install`) and `archive.sh` (`F=angkorfetch-linux-x86_64.tar.gz`,
`&& tar xzf "$F"`). The mock runs used a fresh release folder per case (8 assets; real `.tar.gz`
for the archive; `checksums.txt` from `sha256sum -t`, the Linux text format), with `curl` and
`sudo` stubbed as shell functions that only print.

| Check | Result |
|---|---|
| `bash -n` on `block.sh`, `rpm.sh`, `archive.sh` | exit 0, 0, 0 |
| deb, good file | `angkorfetch_1.2.1_amd64.deb: OK`, then `INSTALL REACHED: sudo apt install ./angkorfetch_1.2.1_amd64.deb`; exit 0 |
| deb, tampered file | `FAILED`, install line **not** reached; exit 1 |
| rpm, good / tampered | `OK` then `sudo dnf install ./angkorfetch-1.2.1-1.x86_64.rpm` reached, exit 0 / `FAILED`, not reached, exit 1 |
| archive, good / tampered | `OK`, `angkorfetch` extracted, exit 0 / `FAILED`, nothing extracted, exit 1 |
| deb line missing from `checksums.txt` (empty `F`) | `sha256sum`: no properly formatted lines; install not reached; exit 1 |
| `wc -l README.md` | **259** (259 CR, CRLF working tree unchanged) |
| Anchors and headings | `#package-status--availability` (2x), `#manual-installation` (2x); headings at 50 and 203. Pass |
| Redaction grep | No match (exit 1). Pass |
| Example blocks vs `HEAD` | Lengths 28 and 14, 4 changed lines, banner 73/73/73. Pass |
| Step 5 command grep (Round 2 pattern) | Lines 57-61 and 145-146, 149-150 only, as in Round 2. `sudo apt\|dnf install` appears only in the Built rows (55-56) and the gated line 226. Pass |
| `git diff --check README.md`; `git diff --stat`; `git status --short` | Clean; `README.md` only (76+, 159-); untracked task file. Pass |

Not verified: as in the Round 1 re-run, plus whether Flathub builds and lists the app for
Alpine hosts (it is Planned; the claim rests on the manifest's bundled runtime).

## QA Results

### Round 4

Verdict (docs): Accepted

Documentation re-check of the Docs stage diff only (`git diff -- docs/DEVELOPMENT.md
team/Docs/README.md`). Independent review: ran as a separate `qa-review` subagent in Claude Code
(2026-11-11) that did not write the change. No file other than this task file was edited.

| File:line | Change | Check against source | Result |
|---|---|---|---|
| docs/DEVELOPMENT.md:130 | Linux display: `xrandr` (X11), then `wlr-randr` (Wayland), then the kernel DRM mode list in `/sys/class/drm` | `get_display` in `src/main.rs`: `xrandr --current` (822), `xrandr` connected line (840), `wlr-randr` (847), `/sys/class/drm/*/modes` (854), then `Unknown` (857). Same order; matches README line 197 | Accurate |
| team/Docs/README.md:38 | "(Options, Example Output)" to "(Usage options table, Example Output)" | README has no `## Options` heading now; the flag table sits under `## Usage` (lines 65-77) | Accurate |

Findings: none.

Missed stale docs: none found. Checks:

- `git grep -i` over tracked files (excluding `README.md` and `team/Tasks/`) for `wayland`,
  `xwayland`, `minimal logo`, `nothing is guessed`, `build information`, `quick start`,
  `(Options`, `sudo angkorfetch`, `ignore-missing`, `no prebuilt`, `musl`, `manual install`,
  `direct github release`, `automated quick install`, `native package`, `full native support`,
  and the old README's update and uninstall commands. Hits are only `release.yml` and `get.sh`
  (code), `improve plan.md` (historical plan, as Docs recorded), and `docs/PACKAGING.md`
  per-channel "once published" commands, which are maintainer docs inside the existing
  out-of-scope PACKAGING follow-up.
- `git grep` for `README` references in other docs: `RELEASING.md:46` and `:117`,
  `CONTRIBUTING.md:114`, and `docs/DEVELOPMENT.md:50` are still true for the new README.
- `docs/DEVELOPMENT.md:142` (truecolor or 16-color, width-based logo) agrees with the new
  Features wording.
- `docs/MAINTAINING.md:44`/`:104` and `docs/RELEASING.md:35` asset counts were already stale
  against `release.yml` before this task and are recorded under Dependencies; the README change
  did not make them stale.

Commands run:

| Command | Result |
|---|---|
| `git diff -- docs/DEVELOPMENT.md team/Docs/README.md` | One line changed in each file |
| `grep -nE 'xrandr\|wlr-randr\|/sys/class/drm\|Fallback' src/main.rs` | Fallback order as in the table above |
| `git diff --check -- docs/DEVELOPMENT.md team/Docs/README.md` | Clean (exit 0) |
| `git ls-files --eol docs/DEVELOPMENT.md team/Docs/README.md` | Index LF, working tree CRLF (`core.autocrlf=true`), as for README |
| `git status --short` | `M README.md`, `M docs/DEVELOPMENT.md`, `M team/Docs/README.md`, `?? team/Tasks/TASK-008-readme-cleanup.md`; nothing else |

Not verified: Linux display output on a real X11, Wayland, or DRM-only host (Windows host).

### Round 3

Verdict: Accepted

Independent review: ran as a separate `qa-review` subagent in Claude Code (2026-11-11,
Windows 11, Git Bash + Python 3.14) that did not write the change. It reviewed the current
`README.md` against the source and re-ran the affected checks itself. No file other than this
task file was edited.

Round 2 findings, re-checked:

| Round 2 finding | Now | Result |
|---|---|---|
| Medium, README.md:226-227, check did not gate the install | Line 226: `grep " $F\$" checksums.txt \| sha256sum -c - && sudo apt install "./$F"` (rpm variant in the comment). Without `pipefail` the pipeline status is `sha256sum`'s, so `&&` runs the install only on `OK`. Lead-in (219) says "The install runs only if `sha256sum` prints `OK`"; the archive sentence (229) swaps the command after `&&` for `tar xzf "$F"` | Fixed (mock runs below) |
| Low, README.md:147-148, musl rows vs the Package rule | Both musl rows show "Flathub (Planned)". `flatpak/io.github.AMRSKH.angkorfetch.yml` bundles `org.freedesktop.Platform` and has no `only-arches`. Snap (`base: core22`, needs `snapd`), AUR (Arch, glibc), and Homebrew (`on_linux` URLs point to glibc tarballs) stay off those rows, which matches the rule at line 139 | Fixed |

New findings (none block):

| Severity | file:line | Finding |
|---|---|---|
| Low | README.md:229 | The archive sentence mentions `angkorfetch.exe` (the Windows zip) inside a flow scoped to "a Linux terminal", and its example extract `tar xzf "$F"` does not open a `.zip` with GNU tar; the sentence is still correct for every Linux archive. |

Commands run (results are this review's own):

| # | Command | Result |
|---|---|---|
| 1 | `wc -l README.md` | 259. Pass |
| 2 | Link extraction + `test -e`; Python heading-slug and duplicate-heading check | 6 relative targets exist; `#package-status--availability` and `#manual-installation` resolve (headings at 50 and 203); no duplicate headings; H2 order unchanged. Pass |
| 3 | `grep -nE "GXCGRV2\|192\.168\.0\.208\|TP-Link\|v1\.1\.1" README.md`; `grep -n "AngkorFetch v1.2.1"` | No match (exit 1); banner at line 94. Pass |
| 4 | Python diff of the two `text` blocks vs `HEAD` | Lengths unchanged; exactly 4 changed lines; banner 73/73/73 characters. Pass |
| 5 | `grep -nE "winget \|brew \|pacman\|yay \|paru\|flatpak \|snap install\|dnf install\|apt install\|upgrade\|AMRSKH/tap\|ignore-missing\|%LOCALAPPDATA%\|sudo angkorfetch\|Every \[GitHub" README.md` | Built rows 55-56 (local-file installs), Pending/Planned rows 57-61, Planned labels 145-146 and 149-150, and the gated line 226 only. Pass |
| 6 | Flags | Not re-run: Usage (65-77) unchanged since Round 1 |
| - | Extracted README lines 222-226 into a scratch script; built an rpm variant (pattern and `dnf`) and an archive variant (`F=angkorfetch-linux-x86_64.tar.gz`, `&& tar xzf "$F"`); `bash -n` on all three | Exit 0, 0, 0 |
| - | Mock runs: fresh folder per case with 8 assets (real `.tar.gz` for the archive), `checksums.txt` from `sha256sum -t` (Linux text format), `curl` stubbed to do nothing, `sudo` stubbed to print | deb good: `OK`, install reached, rc 0. deb tampered: `FAILED`, install **not** reached, rc 1. rpm good: `OK`, `sudo dnf install ./angkorfetch-1.2.1-1.x86_64.rpm` reached, rc 0. rpm tampered: `FAILED`, not reached, rc 1. archive good: `OK`, `angkorfetch` extracted, rc 0. archive tampered: `FAILED`, nothing extracted, rc 1. deb line missing from `checksums.txt`: "no properly formatted SHA256 checksum lines found", not reached, rc 1. Pass |
| - | `sed -n '159,163p' README.md` | Update row 161 still matches the exact-name flow. Pass |
| 9 | `git diff --stat`; `git diff --cached --stat`; `git status --short`; `git diff --check README.md`; `git ls-files --eol README.md` | `README.md` only (76+, 159-); nothing staged; untracked task file; diff check clean; index LF, working tree CRLF. Pass |

Acceptance criteria: all met. Duplicates merged; no Pending or Planned channel shown as
working; update and uninstall cover quick-install, Cargo, and the release `.deb`/`.rpm`;
Supported Platforms and Package Status match `release.yml` and the `c40983a` statuses; example
redacted with banner `v1.2.1`; order and tone kept, 259 lines; all relative links resolve.

Not verified:

- BusyBox `sha256sum -c -` on Alpine, and sudo `secure_path` on a real Linux host (Windows host).
- Whether Flathub will list the app for Alpine hosts (Planned; rests on the bundled runtime).
- The live v1.2.1 `checksums.txt` and asset names, the rpm `%{?dist}` suffix, and the live
  state of any channel (no network checks).
- GitHub rendering of the README.

### Round 2

Verdict: Changes requested

Independent review: ran as a separate `qa-review` subagent in Claude Code (2026-11-11,
Windows 11, Git Bash + Python 3.14) that did not write the change. It reviewed the current
`README.md` against the source, not against the Dev notes. No file other than this task
file was edited.

Round 1 findings, re-checked against the source:

| Round 1 finding | Now | Result |
|---|---|---|
| Medium, `sudo angkorfetch --hinfo` | README.md:196 `sudo "$(command -v angkorfetch)" --hinfo`; the user's shell expands the path before `sudo` runs, so it works for `~/.local/bin` (`get.sh:130`), `~/.cargo/bin`, and `/usr/bin` (deb/rpm) | Fixed |
| Low, APT/DNF called Pending/Planned | README.md:46 now says the APT and DNF repositories are not published, matching **Built** rows 55-56 | Fixed |
| Low, literal `%LOCALAPPDATA%` | README.md:174 names the expanded entry; `get.ps1:99` and `:112` write the expanded `$InstallDir` | Fixed |
| Low, "Every GitHub Release" | README.md:205 "The latest GitHub Release" | Fixed |
| Low, `--ignore-missing` | README.md:226 `grep " $F\$" checksums.txt \| sha256sum -c -`, the same lookup as `get.sh:102`; mock-tested OK (GNU, Linux text format) | Fixed (BusyBox not tested) |
| Low, no CLI download step | README.md:221-228 download block using `releases/latest/download`, as in `get.sh:15` / `get.ps1:19` | Fixed, but see the new Medium |
| Low, update globs | README.md:161 points to the exact-name flow; `curl -O` overwrites the old `checksums.txt` | Fixed |
| Low, Package column mixes available and planned | README.md:139 states the rule; rows 145-146 match `linux/arch/PKGBUILD:6` (`x86_64`, `aarch64`), the formula's `on_linux` `on_arm`/`on_intel`, and `snap/`/`flatpak/` (source builds, no architecture limit); WinGet manifest `x64` only | Fixed; see the new Low |
| Low, BSD list incomplete | README.md:151 "only cross-platform fields populated" | Fixed |

New findings:

| Severity | file:line | Finding |
|---|---|---|
| Medium | README.md:226-227 | The checksum check does not gate the install: pasted as a block (GitHub's copy button copies all six lines), a `FAILED` check is followed by `sudo apt install` anyway, which contradicts the lead-in "Install only if `sha256sum` prints `OK`" (line 219) and the project rule that install paths abort on checksum failure; join them with `&&` (for example `grep " $F\$" checksums.txt \| sha256sum -c - && sudo apt install "./$F"`) and update the archive sentence at line 230 to match. |
| Low | README.md:147-148 | By the new rule at line 139, the Flatpak manifest (freedesktop runtime, independent of the host libc; Flatpak runs on Alpine) arguably also covers the musl rows, which say "No"; either list Flathub (Planned) there or word the rule so it is clear that glibc-based channels apply to glibc rows only. |

Commands run (results are this review's own):

| # | Command | Result |
|---|---|---|
| 1 | `wc -l README.md` | 260. Pass |
| 2 | Link extraction + `test -e`; Python heading-slug and duplicate-heading check | 6 relative targets exist; `#package-status--availability` (2x) and `#manual-installation` (2x) resolve (Manual Installation now at line 203); no duplicate headings; H2 order unchanged from Round 1. Pass |
| 3 | `grep -nE "GXCGRV2\|192\.168\.0\.208\|TP-Link\|v1\.1\.1" README.md`; `grep -n "AngkorFetch v1.2.1"` | No match (exit 1); banner at line 94. Pass |
| 4 | Python diff of the two `text` blocks vs `HEAD` | Lengths unchanged; exactly 4 changed lines; banner 73/73/73 characters. Pass |
| 5 | `grep -nE "winget \|brew \|pacman\|yay \|paru\|flatpak \|snap install\|dnf install angkorfetch\|apt install angkorfetch\|upgrade\|AMRSKH/tap\|ignore-missing\|%LOCALAPPDATA%\|sudo angkorfetch\|Every \[GitHub" README.md` | Lines 57-61 (Pending/Planned rows) and 145-146, 149-150 (Planned labels in Supported Platforms) only; no Round 1 strings left. Pass |
| 6 | Flags | Not re-run: the Usage section (65-77) is unchanged since Round 1, where `-h`, `-v`, `--hinfo` were run |
| 7 | `diff` of `release.yml` `asset:` names vs the README archive rows | Six archives identical. Pass |
| 8 | Install-path and URL grep in README, `get.sh`, `get.ps1` | README 30, 36, 42, 171, 174, 178, 190, 196, 222 match `get.sh:15`, `:130`, `get.ps1:19`, `:99`, `:112`. Pass |
| 9 | `git diff --stat`; `git diff --cached --stat`; `git status --short`; `git diff --check README.md`; `git ls-files --eol README.md` | `README.md` only (77+, 159-); nothing staged; untracked task file; diff check clean; index LF, working tree CRLF. Pass |
| - | `bash -n` on README lines 222-227 | Exit 0 |
| - | Mock release folder (8 assets, `checksums.txt` from `sha256sum -t`, the Linux runner's text format), README lines 224 and 226 | deb lookup returns exactly `angkorfetch_1.2.1_amd64.deb`, check `OK` exit 0; rpm lookup returns `angkorfetch-1.2.1-1.x86_64.rpm`, `OK` exit 0; `F=angkorfetch-linux-x86_64.tar.gz` matches 1 line (not the musl one), `OK` exit 0. Tampered deb pasted as a block: `FAILED`, then the install line still ran (simulated with `echo`), block exit 0. That is the Medium above |
| - | Note | Git Bash `sha256sum` without `-t` writes `hash *name`, which the lookup does not match; this does not apply to the Ubuntu runner, and `get.sh:102` relies on the same two-space format |

Not verified:

- BusyBox `sha256sum -c -` on Alpine, and the sudo `secure_path` behaviour on a real Linux
  host (Windows host).
- The live `checksums.txt` and asset names of v1.2.1, the rpm `%{?dist}` suffix, and the
  live state of any channel (no network checks).
- Snap Store and Flathub aarch64 builds (the manifests do not restrict the architecture).
- GitHub rendering of the README.

### Round 1

Verdict: Changes requested

Independent review: ran as a separate `qa-review` subagent in Claude Code (2026-11-11,
Windows 11, Git Bash + Python 3.14) that did not write the change. Every claim was checked
against `get.sh`, `get.ps1`, `.github/workflows/release.yml`, `src/main.rs`, `Cargo.toml`,
`linux/`, `flatpak/`, `snap/`, `Formula/`, `winget-pkgs/`, `docs/PACKAGING.md`, and git
history, not against the plan. No file other than this task file was edited.

Findings (one Medium blocks unless the maintainer accepts it; the Lows do not block):

| Severity | file:line | Finding |
|---|---|---|
| Medium | README.md:198 | `sudo angkorfetch --hinfo` fails with "command not found" on Debian, Ubuntu, Fedora, and RHEL for the recommended quick-install (`~/.local/bin`) and for Cargo (`~/.cargo/bin`), because sudo's default `secure_path` does not include those folders; use `sudo "$(command -v angkorfetch)" --hinfo` (pre-existing text, but this task's goal and Testing Requirements cover every command shown as working). |
| Low | README.md:46 | Says APT and DNF packages are `[Pending]`/`[Planned]`, while their Package Status rows (55-56) say **Built**; "APT and DNF repositories" would match the table. |
| Low | README.md:174 | `get.ps1` writes the expanded path (`C:\Users\<name>\AppData\Local\AngkorFetch\bin`) to the user `Path`, so the dialog does not show a literal `%LOCALAPPDATA%\AngkorFetch\bin` entry. |
| Low | README.md:208 | "Every GitHub Release includes" is true only from v1.2.1: the musl archive was added in `a15d0f5`, contained only in tag `v1.2.1` (release.yml at `2de15e7`, the v1.1.1 sync, has no musl target). "The latest release" would be accurate. |
| Low | README.md:222 | `sha256sum -c --ignore-missing` needs GNU coreutils; BusyBox `sha256sum` on Alpine, a target listed at line 216, has no `--ignore-missing`. |
| Low | README.md:222 | Compared with the old README, no command-line download URL is left (the old snippet showed `releases/latest/download/<file>`), so a headless `.deb`/`.rpm`/archive user has no copy-paste download step. |
| Low | README.md:161 | The update path reuses the globs at lines 226 and 229, which match more than one file if an older `.deb`/`.rpm` is still in the download folder. |
| Low | README.md:146 | Linux aarch64 shows Package "No", while Windows and macOS rows list Pending/Planned channels and the planned AUR PKGBUILD (`arch=('x86_64' 'aarch64')`) and Homebrew formula (Linux aarch64 URL) both cover aarch64; the column mixes "available" and "planned". |
| Low | README.md:151 | The BSD wording is more accurate than the plan's, but the "(`sysinfo`, environment variables)" list is still incomplete: `get_arch` runs `uname -m` and `get_gpu_usage` runs `nvidia-smi` on every OS. |

Dev divergences judged:

- BSD row wording: accepted as a correct fix of the plan (the plan's "only `sysinfo` fields" is
  wrong); see the Low above for the remaining gap.
- Layout (no `### Options` subheading, `### Direct GitHub Release Packages` dropped, archive
  sentence before the install block): accepted. No fact from plan sections 5 or 11 is missing.

Acceptance criteria:

| Criterion | Result |
|---|---|
| Duplicate sections removed or merged | Pass. 13 H2 headings, no duplicate headings; Quick Start, Usage, and Options merged; Manual Installation no longer repeats quick-install or Cargo |
| No Pending/Planned channel shown as working | Pass. Pending/Planned commands appear only in Package Status rows with their status (57-61); Supported Platforms says "Homebrew (Planned)" |
| Working update and uninstall paths for quick-install and Cargo | Pass. Matches `cp` / `Copy-Item -Force`, the `grep -qF` and `-notcontains` PATH guards, deb `Package: angkorfetch`, rpm `Name: angkorfetch` |
| Supported Platforms and Package Status match `release.yml` and the README's statuses | Pass. Six archives, deb amd64, rpm x86_64; statuses follow `c40983a` (verified: maintainer commit, 2026-11-02); WinGet "files renamed" verified by `d079c68` |
| Example output redacted; banner version matches `Cargo.toml` | Pass |
| Section order and tone kept; 220 to 260 lines | Pass. Order unchanged apart from the merge; 260 lines |
| All relative links resolve | Pass |
| Independent QA accepted | Not yet: the Medium above |

Commands run (results are this review's own):

| # | Command | Result |
|---|---|---|
| 1 | `wc -l README.md`; `git show HEAD:README.md \| wc -l` | 260 (was 342). Pass |
| 2 | Extract every `](...)` target; `test -e` on non-`http`, non-`#` targets; Python GitHub-slug check of headings outside code fences | `CONTRIBUTING.md`, `LICENSE`, `docs/DEVELOPMENT.md`, `docs/MAINTAINING.md`, `docs/PACKAGING.md`, `docs/RELEASING.md` exist; `#package-status--availability` and `#manual-installation` resolve; no duplicate headings. Repo-wide grep finds no links to removed README anchors. Pass |
| 3 | `grep -nE "GXCGRV2\|192\.168\.0\.208\|TP-Link\|v1\.1\.1" README.md`; `grep -n "AngkorFetch v1.2.1"` | No match (exit 1); banner at line 94; `Cargo.toml` `version = "1.2.1"`. Pass |
| 4 | Python diff of the two `text` blocks, HEAD vs working tree | Block lengths unchanged; exactly 4 lines differ (banner, Local IP, Serial, WiFi); banner lines 93-95 are 73, 73, 73 characters; caption changed outside the block. Pass |
| 5 | `grep -nE "winget \|brew \|pacman\|yay \|paru\|flatpak \|snap install\|dnf install angkorfetch\|apt install angkorfetch\|upgrade\|AMRSKH/tap" README.md` | Lines 57-61 (Pending/Planned rows) and 149-150 (`Homebrew (Planned)` labels) only. Pass |
| 6 | `cargo run -q --locked -- -h`; `-- -v`; `-- --hinfo` (labels only recorded) | `-h` exit 0, three options match the Usage table; `-v` exit 0, logo plus box `AngkorFetch v1.2.1`, identical to README lines 86-95, no build info; `--hinfo` exit 0, starts with the logo, 14 labels in the README's order. Pass |
| 7 | Read `release.yml` matrix (lines 138-157), verify step (332-346), `packages` job | Six archive names match README lines 212-217; deb `angkorfetch_${VER}_amd64.deb`, rpm `Release: 1%{?dist}` from `RPMS/x86_64/`; `checksums.txt` is `sha256sum *` over all artifacts. Pass |
| 8 | `grep` install paths | `get.sh:130` `INSTALL_DIR="$HOME/.local/bin"`; `get.ps1:99` `Join-Path $env:LOCALAPPDATA "AngkorFetch\bin"`; README 30, 36, 171, 178 match. musl branches (`get.sh:49-63`), ARM64 fallback (`get.ps1:29-33`), and SHA256 checks in both scripts match lines 30 and 36. Pass |
| 9 | `git diff --stat`; `git diff --cached --stat`; `git status --short` | `README.md` only (73+, 155-); nothing staged; untracked: this task file. Pass |
| 10 | `cargo fmt --check`; `bash -n get.sh && bash -n install.sh`; `git diff --check README.md` | All exit 0 (optional; no Rust or script change). `git ls-files --eol`: index LF, working tree CRLF via `core.autocrlf=true`, unchanged |

Not verified:

- The live state of any channel: the crates.io version, the v1.2.1 Releases page (the musl
  asset is inferred from the verify step plus `origin/automation/sync-packages-v1.2.1`, which
  runs only after `release`), WinGet, the Homebrew tap. No network checks were run.
- The actual rpm file name (`%{?dist}` on the Ubuntu runner); the README uses globs.
- The sudo `secure_path` finding comes from the distributions' default sudoers files, not
  from running it on Linux (Windows host).
- macOS, Linux, and BSD output, and GitHub rendering of the README.

## Documentation Updates

Docs, 2026-11-11. Source: the QA-accepted `git diff README.md` (Round 3). README.md itself not
edited.

| File | Change | Reason |
|---|---|---|
| `team/Docs/README.md` (line 38) | "`README.md` (Options, Example Output)" to "`README.md` (Usage options table, Example Output)" | README no longer has an `## Options` section; the flag table now sits under `## Usage` |
| `docs/DEVELOPMENT.md` (line 130) | Linux display: "`xrandr` (X11)" to "`xrandr` (X11), then `wlr-randr` (Wayland), then the kernel DRM mode list in `/sys/class/drm`" | It repeated the old README's xrandr-only claim, which the README now corrects; order checked against `get_display` in `src/main.rs` (lines 818-857) |

Checked, no change needed:

- Links to removed or renamed README headings (Quick Start, Options, Direct GitHub Release
  Packages, Automated Quick Install Scripts, the per-channel install subsections): a search of
  every tracked file finds no `README.md#...` link and no reference to those headings outside
  this task file and `improve plan.md`. The README's own anchors (`#package-status--availability`,
  `#manual-installation`) were verified by QA.
- `improve plan.md`: the original maintainer prompt from `5c3fa34` that proposed the README
  layout (Quick Start, Options). It is a historical plan, not documentation of the current
  README, so it is left as is.
- `RELEASING.md:44-46` (quick-install one-liners from `main` are "published in the README"):
  still true (README lines 28 and 34). `RELEASING.md:117` ("`README.md` version tables and
  sample banners"): the sample banner still exists; no version table existed before or after
  this change, so the wording is unchanged by this task.
- `AGENTS.md` (sections 1, 6, 11), `CONTRIBUTING.md`, `team/` role and command files, and the
  tool wrappers in `.claude/`, `.opencode/`, `.codex/`, `.agents/`: no README section names,
  flags, or install, update, or uninstall text that the change affects.
- Package descriptions (`Cargo.toml`, `flatpak/` metainfo, `snap/snapcraft.yaml`, `linux/rpm`
  spec, WinGet locale manifest): generic summaries; none repeats the changed Features wording.
- Not changed, out of scope (Dependencies): `docs/PACKAGING.md`, `Formula/README.md`,
  `docs/RELEASING.md`, `docs/MAINTAINING.md`, root `RELEASING.md`, PKGBUILD, and snapcraft
  inconsistencies. One more of the same kind, recorded only: the `docs/PACKAGING.md` overview
  still shows `sudo dnf install angkorfetch` and `sudo apt install angkorfetch` for the
  **Built** rows (target repository commands), while the README Package Status now shows the
  local-file commands. It belongs to the separate PACKAGING alignment follow-up in Notes.

## Git Commit

- Commit Message: `docs(readme): remove duplicates and correct install, update, and platform facts [TASK-008]`
- Commit Hash: `8cd04280e5dd9171a32a044efb7d006a92c8e94e`

## Final Report

| Item | Content |
|---|---|
| Task | TASK-008: Clean Up README |
| What changed | README tidied in the existing section order (342 to 259 lines): Quick Start, Usage, and Options merged; six Pending/Planned install subsections replaced by one pointer; Package Status and Supported Platforms corrected (musl prebuilt, local-file deb/rpm commands, AUR); Update and Uninstall cover only working methods; Manual Installation gets an asset table and a checksum-gated download; Troubleshooting fixed (`sudo` path, Wayland); example output redacted and banner set to v1.2.1. Docs stage aligned `docs/DEVELOPMENT.md` (display fallbacks) and `team/Docs/README.md` (section name) |
| Files | `README.md`, `docs/DEVELOPMENT.md`, `team/Docs/README.md`, this file |
| Stages | Advisor, Dev, QA Review, and Docs each ran in its own Claude Code subagent (`advisor`, `dev`, `qa-review`, `docs`); orchestration, final validation, commits, and this report ran in the main session |
| QA | Independent `qa-review` subagent: Round 1 and Round 2 `Changes requested` (one Medium each, all findings fixed by Dev); Round 3 `Accepted`; docs Round 4 `Accepted`, no findings |
| Validation | Final check in the main session (2026-11-11): 259 lines; all 6 relative links exist; both in-page anchors match headings; redaction grep empty; `git diff --check` clean; only the four in-scope files changed. No Rust, script, or workflow change, so fmt, Clippy, and tests were not required |
| Limitations | No network checks (live v1.2.1 assets, crates.io version, channel states); BusyBox `sha256sum -c -` and sudo `secure_path` not run on Linux; GitHub rendering not previewed |

## Notes and Limitations

- Advisor (2026-11-11): not blocked. The task file already answers every decision this plan
  needs: scope, example handling, and "do not change channel status".
- Not verified offline: the actual asset names on the v1.2.1 Releases page (inferred from
  `release.yml` plus the bot sync branch), the version currently on crates.io (the repository
  has no publish step), the state of WinGet PR #409790, and whether `AMRSKH/homebrew-tap` is
  still maintained.
- Follow-up for the maintainer (does not block this task): `docs/PACKAGING.md` and
  `Formula/README.md` still present the Homebrew tap as published, and the WinGet entry still
  points to PR #409790. Decide whether to align them with the README in a separate task. The
  list is under Dependencies.
- `install.sh` / `install.ps1` (local source installers) are not mentioned in the README, and
  this plan does not add them.
- Known Low, accepted without a fix (QA Round 3): README.md:229 mentions `angkorfetch.exe` in a
  flow labelled "From a Linux terminal", and the `tar xzf` example does not open the Windows
  `.zip`. Correct for every Linux archive; left as is to avoid a fourth code round.
- Docs stage also noted that the `docs/PACKAGING.md` overview still shows
  `sudo dnf install angkorfetch` / `sudo apt install angkorfetch` for the Built rows; part of
  the PACKAGING alignment follow-up above.
