#!/usr/bin/env bash
# AngkorFetch — universal installer
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/AMRSKH/angkorfetch/main/get.sh | bash
#
# Detects OS + architecture, downloads the matching prebuilt binary from the
# latest GitHub Release, and installs it onto the user's PATH. No Rust
# toolchain required on the user's machine.

set -euo pipefail

REPO="AMRSKH/angkorfetch"
BIN_NAME="angkorfetch"
BASE_URL="https://github.com/${REPO}/releases/latest/download"

GREEN="\033[1;32m"; YELLOW="\033[1;33m"; RED="\033[1;31m"; RESET="\033[0m"
info()  { printf "${GREEN}==>${RESET} %s\n" "$1"; }
warn()  { printf "${YELLOW}==>${RESET} %s\n" "$1"; }
error() { printf "${RED}==>${RESET} %s\n" "$1" >&2; }

# ---- Dependency check -------------------------------------------------------
if ! command -v curl >/dev/null 2>&1; then
    error "curl is required but not found. Please install curl and re-run."
    exit 1
fi
if ! command -v tar >/dev/null 2>&1; then
    error "tar is required but not found. Please install tar and re-run."
    exit 1
fi

# ---- Detect platform -------------------------------------------------------
OS="$(uname -s)"
ARCH="$(uname -m)"

# Detect musl libc (Alpine or musl-based distro)
IS_MUSL=false
if [ "$OS" = "Linux" ]; then
    if ldd /bin/sh 2>/dev/null | grep -q musl \
        || [ -e /lib/ld-musl-x86_64.so.1 ] \
        || [ -e /lib/ld-musl-aarch64.so.1 ]; then
        IS_MUSL=true
    fi
fi

case "$OS" in
    Linux)
        case "$ARCH" in
            x86_64)
                if [ "$IS_MUSL" = "true" ]; then
                    ASSET="angkorfetch-linux-x86_64-musl.tar.gz"
                else
                    ASSET="angkorfetch-linux-x86_64.tar.gz"
                fi
                ;;
                        aarch64|arm64)
                if [ "$IS_MUSL" = "true" ]; then
                    error "No prebuilt binary for musl on ARM64 (e.g. Alpine)."
                    error "Install with: cargo install angkorfetch"
                    exit 1
                fi
                ASSET="angkorfetch-linux-aarch64.tar.gz"
                ;;
            *) error "Unsupported Linux architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    Darwin)
        case "$ARCH" in
            x86_64) ASSET="angkorfetch-macos-x86_64.tar.gz" ;;
            arm64)  ASSET="angkorfetch-macos-aarch64.tar.gz" ;;
            *) error "Unsupported macOS architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        error "This script supports Linux and macOS. On Windows, use get.ps1 instead."
        exit 1
        ;;
esac

DOWNLOAD_URL="${BASE_URL}/${ASSET}"
CHECKSUM_URL="${BASE_URL}/checksums.txt"

# ---- Download + verify ------------------------------------------------------
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

info "Downloading ${ASSET}..."
if ! curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$ASSET"; then
    error "Download failed: ${DOWNLOAD_URL}"
    error "Check https://github.com/${REPO}/releases for available downloads."
    exit 1
fi

info "Downloading checksums.txt..."
if ! curl -fsSL "$CHECKSUM_URL" -o "$TMP_DIR/checksums.txt"; then
    error "Could not download checksums.txt from ${CHECKSUM_URL}"
    exit 1
fi

info "Verifying SHA256 checksum..."
# Plain-text format: "<hash>  <filename>"
EXPECTED=$(grep " ${ASSET}$" "$TMP_DIR/checksums.txt" | awk '{print $1}')
if [ -z "$EXPECTED" ]; then
    error "Could not find checksum for ${ASSET} in checksums.txt"
    exit 1
fi

if command -v sha256sum >/dev/null 2>&1; then
    ACTUAL=$(sha256sum "$TMP_DIR/$ASSET" | awk '{print $1}')
elif command -v shasum >/dev/null 2>&1; then
    ACTUAL=$(shasum -a 256 "$TMP_DIR/$ASSET" | awk '{print $1}')
else
    error "No sha256sum or shasum found; cannot verify checksum. Aborting."
    exit 1
fi

if [ "$EXPECTED" != "$ACTUAL" ]; then
    error "Checksum mismatch for ${ASSET}!"
    error "  Expected: ${EXPECTED}"
    error "  Actual:   ${ACTUAL}"
    error "Download may be corrupted or tampered. Aborting."
    exit 1
fi
info "Checksum OK."

# ---- Extract + install ------------------------------------------------------
info "Extracting..."
tar xzf "$TMP_DIR/$ASSET" -C "$TMP_DIR"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"
cp "$TMP_DIR/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
chmod +x "$INSTALL_DIR/$BIN_NAME"

# On macOS, clear Gatekeeper quarantine attribute to prevent execution blocks
if [ "$OS" = "Darwin" ]; then
    xattr -c "$INSTALL_DIR/$BIN_NAME" 2>/dev/null \
        || xattr -d com.apple.quarantine "$INSTALL_DIR/$BIN_NAME" 2>/dev/null \
        || true
fi

info "Installed ${BIN_NAME} to ${INSTALL_DIR}/${BIN_NAME}"

# ---- PATH setup -------------------------------------------------------------
case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        UPDATED_CONFIGS=""
        if [ "$OS" = "Darwin" ]; then
            for rc in "$HOME/.zprofile" "$HOME/.zshrc"; do
                if [ -f "$rc" ] || { [ ! -f "$HOME/.zprofile" ] && [ ! -f "$HOME/.zshrc" ]; }; then
                    touch "$rc" 2>/dev/null || true
                    if ! grep -qF "$INSTALL_DIR" "$rc" 2>/dev/null; then
                        printf '\nexport PATH="%s:$PATH"\n' "$INSTALL_DIR" >> "$rc"
                        UPDATED_CONFIGS="${UPDATED_CONFIGS} ${rc}"
                    fi
                fi
            done
            if [ -f "$HOME/.bash_profile" ] && ! grep -qF "$INSTALL_DIR" "$HOME/.bash_profile" 2>/dev/null; then
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
            if ! grep -qF "$INSTALL_DIR" "$TARGET_RC" 2>/dev/null; then
                printf '\nexport PATH="%s:$PATH"\n' "$INSTALL_DIR" >> "$TARGET_RC"
                UPDATED_CONFIGS="${UPDATED_CONFIGS} ${TARGET_RC}"
            fi
        fi

        if [ -n "$UPDATED_CONFIGS" ]; then
            warn "Added ${INSTALL_DIR} to PATH in:${UPDATED_CONFIGS}"
            warn "Open a new terminal, or run:  export PATH=\"${INSTALL_DIR}:\$PATH\""
        else
            warn "${INSTALL_DIR} is not in your PATH."
            warn "Open a new terminal, or run:  export PATH=\"${INSTALL_DIR}:\$PATH\""
        fi
        ;;
esac

echo
info "Done! Try it out:"
echo "    angkorfetch"
echo "    angkorfetch -v"
echo "    angkorfetch --hinfo   (or --hard)"
