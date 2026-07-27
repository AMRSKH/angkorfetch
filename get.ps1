# AngkorFetch — universal installer for Windows
#
# Usage:
#   irm https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.ps1 | iex
#
# Downloads the latest prebuilt angkorfetch.exe from GitHub Releases and puts
# it on PATH. No Rust toolchain required.

$ErrorActionPreference = "Stop"

$Repo = "AMRSKH/angkorfetch"
$BinName = "angkorfetch.exe"
$AssetName = "angkorfetch-windows-x86_64.zip"

function Write-Info($msg) { Write-Host "==> $msg" -ForegroundColor Green }
function Write-Warn($msg) { Write-Host "==> $msg" -ForegroundColor Yellow }

Write-Info "Looking up the latest release of $Repo..."
$release = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases/latest"
$relAsset = $release.assets | Where-Object { $_.name -eq $AssetName }

if (-not $relAsset) {
    Write-Host "==> Could not find $AssetName in the latest release of $Repo." -ForegroundColor Red
    Write-Host "==> Check https://github.com/$Repo/releases for available downloads." -ForegroundColor Red
    exit 1
}

$tmpDir = Join-Path $env:TEMP "angkorfetch-install"
New-Item -ItemType Directory -Force -Path $tmpDir | Out-Null
$zipPath = Join-Path $tmpDir $AssetName

Write-Info "Downloading $AssetName..."
Invoke-WebRequest -Uri $relAsset.browser_download_url -OutFile $zipPath

Write-Info "Extracting..."
Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force

$InstallDir = Join-Path $env:LOCALAPPDATA "AngkorFetch\bin"
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Copy-Item (Join-Path $tmpDir $BinName) (Join-Path $InstallDir $BinName) -Force

Write-Info "Installed $BinName to $InstallDir"

$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$InstallDir", "User")
    Write-Warn "Added $InstallDir to PATH. Open a new terminal window for it to take effect."
}

Write-Host ""
Write-Info "Done! Try it out:"
Write-Host "    angkorfetch"
Write-Host "    angkorfetch -v"
Write-Host "    angkorfetch --hinfo   (or --hard)"
