#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "Building Arch Linux package using makepkg..."
makepkg -sfc --noconfirm

echo "Arch Linux package built successfully in $SCRIPT_DIR"
