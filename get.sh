#!/usr/bin/env bash
# AngkorFetch — universal installer
#
# Usage (once published on GitHub):
#   curl -fsSL https://raw.githubusercontent.com/<YOUR_GH_USER>/angkorfetch/main/get.sh | bash
#
# Detects OS + architecture, downloads the matching prebuilt binary from the
# latest GitHub Release, and installs it onto the user's PATH. No Rust
# toolchain required on the user's machine.

set -euo pipefail

REPO="AMRSKH/angkorfetch"
BIN_NAME="angkorfetch"

GREEN="\033[1;32m"; YELLOW="\033[1;33m"; RED="\033[1;31m"; RESET="\033[0m"
info()  { printf "${GREEN}==>${RESET} %s\n" "$1"; }
warn()  { printf "${YELLOW}==>${RESET} %s\n" "$1"; }
error() { printf "${RED}==>${RESET} %s\n" "$1" >&2; }

# ---- Detect platform -------------------------------------------------------
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        case "$ARCH" in
            x86_64) ASSET="angkorfetch-linux-x86_64.tar.gz" ;;
            aarch64|arm64) ASSET="angkorfetch-linux-aarch64.tar.gz" ;;
            *) error "Unsupported Linux architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    Darwin)
        case "$ARCH" in
            x86_64) ASSET="angkorfetch-macos-x86_64.tar.gz" ;;
            arm64) ASSET="angkorfetch-macos-aarch64.tar.gz" ;;
            *) error "Unsupported macOS architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        error "This script supports Linux and macOS. On Windows, use install.ps1 or the PowerShell one-liner instead."
        exit 1
        ;;
esac

# ---- Resolve latest release URL -------------------------------------------
API_URL="https://api.github.com/repos/${REPO}/releases/latest"
info "Looking up the latest release of ${REPO}..."
DOWNLOAD_URL=$(curl -fsSL "$API_URL" | grep "browser_download_url" | grep "$ASSET" | cut -d '"' -f 4)

if [ -z "$DOWNLOAD_URL" ]; then
    error "Could not find a release asset named ${ASSET} for ${REPO}."
    error "Check https://github.com/${REPO}/releases for available downloads."
    exit 1
fi

# ---- Download + install -----------------------------------------------------
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

info "Downloading ${ASSET}..."
curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$ASSET"

info "Extracting..."
tar xzf "$TMP_DIR/$ASSET" -C "$TMP_DIR"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"
cp "$TMP_DIR/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
chmod +x "$INSTALL_DIR/$BIN_NAME"

info "Installed ${BIN_NAME} to ${INSTALL_DIR}/${BIN_NAME}"

case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        SHELL_RC="$HOME/.bashrc"
        [ -n "${ZSH_VERSION:-}" ] && SHELL_RC="$HOME/.zshrc"
        printf '\nexport PATH="%s:$PATH"\n' "$INSTALL_DIR" >> "$SHELL_RC"
        warn "Added ${INSTALL_DIR} to PATH in ${SHELL_RC}. Run: source ${SHELL_RC} (or open a new terminal)"
        ;;
esac

echo
info "Done! Try it out:"
echo "    angkorfetch"
echo "    angkorfetch -v"
echo "    angkorfetch --hinfo   (or --hard)"
