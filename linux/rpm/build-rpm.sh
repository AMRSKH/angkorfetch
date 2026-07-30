#!/usr/bin/env bash
set -euo pipefail

VERSION="1.1.1"
SPEC="angkorfetch.spec"
RPMBUILD_DIR="$HOME/rpmbuild"

mkdir -p "$RPMBUILD_DIR"/{SOURCES,SPECS,RPMS,SRPMS,BUILD}

git archive --format=tar.gz -o "$RPMBUILD_DIR/SOURCES/angkorfetch-$VERSION.tar.gz" --prefix="angkorfetch-$VERSION/" HEAD

cp "$SPEC" "$RPMBUILD_DIR/SPECS/"

rpmbuild -ba "$RPMBUILD_DIR/SPECS/$SPEC"

echo "RPMS built at: $RPMBUILD_DIR/RPMS/"
