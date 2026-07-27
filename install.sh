#!/usr/bin/env bash
# AngkorFetch installer
#
# Usage (from inside the project folder):
#   bash install.sh
#
# This single command will:
#   1. Install the Rust toolchain via rustup if `cargo` isn't already present
#   2. Build AngkorFetch in release mode
#   3. Install the `angkorfetch` binary onto your PATH
#
# After it finishes you can run: angkorfetch | angkorfetch -v | angkorfetch --hinfo

set -euo pipefail

BOLD="\033[1m"
GREEN="\033[1;32m"
YELLOW="\033[1;33m"
RED="\033[1;31m"
RESET="\033[0m"

info()  { printf "${GREEN}==>${RESET} %s\n" "$1"; }
warn()  { printf "${YELLOW}==>${RESET} %s\n" "$1"; }
error() { printf "${RED}==>${RESET} %s\n" "$1" >&2; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 1. Ensure Rust/Cargo is available -----------------------------------------
if ! command -v cargo >/dev/null 2>&1; then
    warn "Rust (cargo) not found. Installing it via rustup..."
    if ! command -v curl >/dev/null 2>&1; then
        error "curl is required to install Rust automatically. Please install curl, or install Rust yourself from https://rustup.rs, then re-run this script."
        exit 1
    fi
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    # shellcheck disable=SC1090
    source "$HOME/.cargo/env"
fi

info "Using $(cargo --version)"

# 2. Build in release mode ----------------------------------------------------
info "Building AngkorFetch (release)..."
cargo build --release --quiet

BIN_NAME="angkorfetch"
BUILT_BIN="target/release/${BIN_NAME}"

if [ ! -f "$BUILT_BIN" ]; then
    error "Build finished but ${BUILT_BIN} was not found."
    exit 1
fi

# 3. Install onto PATH --------------------------------------------------------
# Prefer ~/.cargo/bin (already on PATH for anyone who just installed Rust via
# rustup) and fall back to ~/.local/bin, creating it if needed.
INSTALL_DIR="$HOME/.cargo/bin"
if [ ! -d "$INSTALL_DIR" ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

cp "$BUILT_BIN" "$INSTALL_DIR/$BIN_NAME"
chmod +x "$INSTALL_DIR/$BIN_NAME"

info "Installed ${BIN_NAME} to ${INSTALL_DIR}/${BIN_NAME}"

case ":$PATH:" in
    *":$INSTALL_DIR:"*) ;;
    *)
        warn "${INSTALL_DIR} is not on your PATH yet."
        SHELL_RC="$HOME/.bashrc"
        [ -n "${ZSH_VERSION:-}" ] && SHELL_RC="$HOME/.zshrc"
        printf '\nexport PATH="%s:$PATH"\n' "$INSTALL_DIR" >> "$SHELL_RC"
        warn "Added it to ${SHELL_RC}. Run: source ${SHELL_RC}  (or open a new terminal)"
        ;;
esac

echo
info "Done! Try it out:"
echo "    angkorfetch"
echo "    angkorfetch -v"
echo "    angkorfetch --hinfo   (or --hard)"
