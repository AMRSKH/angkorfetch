# AngkorFetch installer for Windows
#
# Usage (from inside the project folder, in PowerShell):
#   .\install.ps1
#
# This single command will:
#   1. Install the Rust toolchain via rustup-init if `cargo` isn't already present
#   2. Build AngkorFetch in release mode
#   3. Install the angkorfetch.exe binary onto your PATH
#
# After it finishes you can run: angkorfetch | angkorfetch -v | angkorfetch --hinfo

$ErrorActionPreference = "Stop"

function Write-Info($msg)  { Write-Host "==> $msg" -ForegroundColor Green }
function Write-Warn($msg)  { Write-Host "==> $msg" -ForegroundColor Yellow }
function Write-ErrorMsg($msg) { Write-Host "==> $msg" -ForegroundColor Red }

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location $ScriptDir

# 1. Ensure Rust/Cargo is available ------------------------------------------
$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargo) {
    Write-Warn "Rust (cargo) not found. Installing it via rustup-init..."
    $rustupExe = Join-Path $env:TEMP "rustup-init.exe"
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustupExe
    & $rustupExe -y --default-toolchain stable
    $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
}

Write-Info "Using $(cargo --version)"

# 2. Build in release mode ----------------------------------------------------
Write-Info "Building AngkorFetch (release)..."
cargo build --release --quiet

$BinName = "angkorfetch.exe"
$BuiltBin = Join-Path "target\release" $BinName

if (-not (Test-Path $BuiltBin)) {
    Write-ErrorMsg "Build finished but $BuiltBin was not found."
    exit 1
}

# 3. Install onto PATH --------------------------------------------------------
$InstallDir = Join-Path $env:USERPROFILE ".cargo\bin"
if (-not (Test-Path $InstallDir)) {
    $InstallDir = Join-Path $env:LOCALAPPDATA "AngkorFetch\bin"
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

Copy-Item $BuiltBin (Join-Path $InstallDir $BinName) -Force

Write-Info "Installed $BinName to $InstallDir"

$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$InstallDir*") {
    Write-Warn "$InstallDir is not on your PATH yet. Adding it..."
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$InstallDir", "User")
    Write-Warn "Open a new terminal window for the PATH change to take effect."
}

Write-Host ""
Write-Info "Done! Try it out:"
Write-Host "    angkorfetch"
Write-Host "    angkorfetch -v"
Write-Host "    angkorfetch --hinfo   (or --hard)"
