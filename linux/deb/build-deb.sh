#!/usr/bin/env bash
set -euo pipefail

VERSION="1.0.0"
BIN="angkorfetch"
ARCH="amd64"
DEB_DIR="deb-pkg"

mkdir -p "$DEB_DIR/DEBIAN"
mkdir -p "$DEB_DIR/usr/bin"
mkdir -p "$DEB_DIR/usr/share/doc/$BIN"

cat > "$DEB_DIR/DEBIAN/control" <<EOF
Package: angkorfetch
Version: $VERSION
Section: utils
Priority: optional
Architecture: $ARCH
Maintainer: AMRSKH <inforithseyhacambo@gmail.com>
Description: A fast, cross-platform system fetch tool
 AngkorFetch displays system information (OS, CPU, GPU, memory,
 disk, network, battery, etc.) in a colorful terminal output.
Homepage: https://github.com/AMRSKH/angkorfetch
License: MIT
EOF

cargo build --release
cp "target/release/$BIN" "$DEB_DIR/usr/bin/"
chmod 755 "$DEB_DIR/usr/bin/$BIN"

cp README.md "$DEB_DIR/usr/share/doc/$BIN/"
gzip -9 -n "$DEB_DIR/usr/share/doc/$BIN/README.md" 2>/dev/null || true

dpkg-deb --build "$DEB_DIR" "angkorfetch_${VERSION}_${ARCH}.deb"
rm -rf "$DEB_DIR"

echo "Created: angkorfetch_${VERSION}_${ARCH}.deb"
