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

# On macOS, clear Gatekeeper quarantine attribute to prevent execution blocks
if [ "$OS" = "Darwin" ]; then
    xattr -c "$INSTALL_DIR/$BIN_NAME" 2>/dev/null || xattr -d com.apple.quarantine "$INSTALL_DIR/$BIN_NAME" 2>/dev/null || true
fi

info "Installed ${BIN_NAME} to ${INSTALL_DIR}/${BIN_NAME}"

case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        UPDATED_CONFIGS=""
        if [ "$OS" = "Darwin" ]; then
            # macOS default shell is zsh since macOS Catalina (10.15); login shells also read .zprofile
            for rc in "$HOME/.zprofile" "$HOME/.zshrc"; do
                if [ -f "$rc" ] || [ ! -f "$HOME/.zprofile" -a ! -f "$HOME/.zshrc" ]; then
                    touch "$rc" 2>/dev/null || true
                    if ! grep -q "$INSTALL_DIR" "$rc" 2>/dev/null; then
                        printf '\nexport PATH="%s:$PATH"\n' "$INSTALL_DIR" >> "$rc"
                        UPDATED_CONFIGS="${UPDATED_CONFIGS} ${rc}"
                    fi
                fi
            done
            if [ -f "$HOME/.bash_profile" ] && ! grep -q "$INSTALL_DIR" "$HOME/.bash_profile" 2>/dev/null; then
                printf '\nexport PATH="%s:$PATH"\n' "$INSTALL_DIR" >> "$HOME/.bash_profile"
                UPDATED_CONFIGS="${UPDATED_CONFIGS} $HOME/.bash_profile"
            fi
        else
            SHELL_NAME="$(basename "${SHELL:-bash}")"
            if [ "$SHELL_NAME" = "zsh" ] || [ -f "$HOME/.zshrc" ]; then
                TARGET_RC="$HOME/.zshrc"
            else
                TARGET_RC="$HOME/.bashrc"
            fi
            touch "$TARGET_RC" 2>/dev/null || true
            if ! grep -q "$INSTALL_DIR" "$TARGET_RC" 2>/dev/null; then
                printf '\nexport PATH="%s:$PATH"\n' "$INSTALL_DIR" >> "$TARGET_RC"
                UPDATED_CONFIGS="${UPDATED_CONFIGS} ${TARGET_RC}"
            fi
        fi

        if [ -n "$UPDATED_CONFIGS" ]; then
            warn "Added ${INSTALL_DIR} to PATH in:${UPDATED_CONFIGS}"
            warn "Restart your terminal or run: export PATH=\"${INSTALL_DIR}:\$PATH\""
        fi
        ;;
esac

echo
info "Done! Try it out:"
echo "    angkorfetch"
echo "    angkorfetch -v"
echo "    angkorfetch --hinfo   (or --hard)"
