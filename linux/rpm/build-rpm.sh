#!/usr/bin/env bash
set -euo pipefail

VERSION="1.2.1"
SPEC="angkorfetch.spec"
RPMBUILD_DIR="$HOME/rpmbuild"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel)"

mkdir -p "$RPMBUILD_DIR"/{SOURCES,SPECS,RPMS,SRPMS,BUILD}

git -C "$REPO_ROOT" archive --format=tar.gz -o "$RPMBUILD_DIR/SOURCES/v$VERSION.tar.gz" --prefix="angkorfetch-$VERSION/" HEAD

cp "$SCRIPT_DIR/$SPEC" "$RPMBUILD_DIR/SPECS/"

rpmbuild -ba "$RPMBUILD_DIR/SPECS/$SPEC"

echo "RPMS built at: $RPMBUILD_DIR/RPMS/"
