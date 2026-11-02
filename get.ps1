# AngkorFetch — universal installer for Windows
#
# Usage:
#   irm https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.ps1 | iex
#
# Downloads the latest prebuilt angkorfetch.exe from GitHub Releases and puts
# it on PATH. No Rust toolchain required.
#
# Requires: PowerShell 5.1+ or PowerShell 7+ (pwsh).
# TLS 1.2 is enforced explicitly for PowerShell 5.1 compatibility.

$ErrorActionPreference = "Stop"

# ---- TLS 1.2 for PowerShell 5.1 (Windows 10 / Server 2016) ----------------
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$Repo     = "AMRSKH/angkorfetch"
$BinName  = "angkorfetch.exe"
$BaseUrl  = "https://github.com/$Repo/releases/latest/download"

function Write-Info($msg) { Write-Host "==> $msg" -ForegroundColor Green }
function Write-Warn($msg) { Write-Host "==> $msg" -ForegroundColor Yellow }
function Write-Err($msg)  { Write-Host "==> $msg" -ForegroundColor Red }

# ---- Architecture detection ------------------------------------------------
$Cpu = $env:PROCESSOR_ARCHITECTURE   # AMD64 / ARM64 / x86
switch ($Cpu) {
    "AMD64" { $AssetName = "angkorfetch-windows-x86_64.zip" }
    "ARM64" {
        # No native ARM64 Windows binary yet — fall back to x64 (runs via emulation)
        Write-Warn "ARM64 detected; downloading x86_64 build (runs via Windows x64 emulation)."
        $AssetName = "angkorfetch-windows-x86_64.zip"
    }
    default {
        Write-Err "Unsupported architecture: $Cpu"
        exit 1
    }
}

$DownloadUrl  = "$BaseUrl/$AssetName"
$ChecksumUrl  = "$BaseUrl/checksums.txt"

$TmpDir = Join-Path $env:TEMP "angkorfetch-install-$(Get-Random)"
New-Item -ItemType Directory -Force -Path $TmpDir | Out-Null

$ZipPath      = Join-Path $TmpDir $AssetName
$ChecksumPath = Join-Path $TmpDir "checksums.txt"

# ---- Download binary -------------------------------------------------------
Write-Info "Downloading $AssetName..."
try {
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $ZipPath -UseBasicParsing
} catch {
    Write-Err "Download failed: $DownloadUrl"
    Write-Err "Check https://github.com/$Repo/releases for available downloads."
    exit 1
}

# ---- Download + verify checksum --------------------------------------------
Write-Info "Downloading checksums.txt..."
try {
    Invoke-WebRequest -Uri $ChecksumUrl -OutFile $ChecksumPath -UseBasicParsing
} catch {
    Write-Err "Could not download checksums.txt from $ChecksumUrl"
    exit 1
}

Write-Info "Verifying SHA256 checksum..."
# checksums.txt is plain text: "<hash>  <filename>"  (two spaces, GNU sha256sum format)
$ChecksumLines = Get-Content $ChecksumPath -Encoding UTF8
$Expected = $null
foreach ($line in $ChecksumLines) {
    # Match lines ending with two-space separator then our asset name
    if ($line -match "^([0-9a-fA-F]{64})\s+$([regex]::Escape($AssetName))$") {
        $Expected = $Matches[1].ToLower()
        break
    }
}
if (-not $Expected) {
    Write-Err "Could not find checksum for $AssetName in checksums.txt"
    exit 1
}

$Hash = (Get-FileHash -Path $ZipPath -Algorithm SHA256).Hash.ToLower()
if ($Hash -ne $Expected) {
    Write-Err "Checksum mismatch for $AssetName!"
    Write-Err "  Expected: $Expected"
    Write-Err "  Actual:   $Hash"
    Write-Err "Download may be corrupted or tampered. Aborting."
    Remove-Item -Recurse -Force $TmpDir
    exit 1
}
Write-Info "Checksum OK."

# ---- Extract + install -----------------------------------------------------
Write-Info "Extracting..."
Expand-Archive -Path $ZipPath -DestinationPath $TmpDir -Force

$InstallDir = Join-Path $env:LOCALAPPDATA "AngkorFetch\bin"
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
Copy-Item (Join-Path $TmpDir $BinName) (Join-Path $InstallDir $BinName) -Force
Remove-Item -Recurse -Force $TmpDir

Write-Info "Installed $BinName to $InstallDir"

# ---- PATH update (exact-entry comparison, not substring -like) -------------
$UserPath    = [Environment]::GetEnvironmentVariable("Path", "User")
$PathEntries = $UserPath -split ";" | Where-Object { $_ -ne "" }

if ($PathEntries -notcontains $InstallDir) {
    $NewPath = ($PathEntries + $InstallDir) -join ";"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Warn "Added $InstallDir to your persistent User PATH."
    Write-Warn "Open a new terminal so angkorfetch is recognised."
    Write-Warn "Alternatively, run in the current session:"
    Write-Warn "    `$env:Path += `";$InstallDir`""
} else {
    Write-Info "$InstallDir is already in your User PATH."
}

# Refresh current session PATH so the binary is usable immediately
if (($env:Path -split ";") -notcontains $InstallDir) {
    $env:Path = "$env:Path;$InstallDir"
}

Write-Host ""
Write-Info "Done! Try it out:"
Write-Host "    angkorfetch"
Write-Host "    angkorfetch -v"
Write-Host "    angkorfetch --hinfo   (or --hard)"
Write-Host ""
Write-Warn "If 'angkorfetch' is not recognised, open a new terminal window."
