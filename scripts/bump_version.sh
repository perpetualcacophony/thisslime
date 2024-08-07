#!/bin/bash
set -e

MAJOR=$(sed -En 's|version = "([0-9]+).[0-9]+.[0-9]+"|\1|p' ./Cargo.toml)
MINOR=$(sed -En 's|version = "[0-9]+.([0-9]+).[0-9]+"|\1|p' ./Cargo.toml)
PATCH=$(sed -En 's|version = "[0-9]+.[0-9]+.([0-9]+)"|\1|p' ./Cargo.toml)

if [[ "$1" == "minor" ]]; then
    ((MINOR++))
    PATCH=0
else
    PATCH=$((PATCH+1))
fi

VERSION="$MAJOR.$MINOR.$PATCH"

git branch "release/v$VERSION"
git checkout "release/v$VERSION"

./scripts/set_version.sh "$VERSION-rc1"

git commit -a -m "bumped version to release candidate 1"

echo $VERSION