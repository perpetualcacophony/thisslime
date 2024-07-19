#!/bin/bash
set -e
VERSION=$(sed -En 's|version = "(.+)-rc.+"|\1|p' ./Cargo.toml)

./scripts/set_version.sh $VERSION

git commit -a -m "removed release candidate tag"

echo $VERSION