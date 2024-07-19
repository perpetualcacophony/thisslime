#!/bin/bash
set -e

RC=$(sed -En 's|version = "[0-9]+.[0-9]+.[0-9]+-rc([0-9]+)"|\1|p' ./Cargo.toml)
VERSION=$(sed -En 's|version = "(.+)"|\1|p' ./Cargo.toml)

if [ -n "$RC" ]; then
    ./scripts/set_version.sh "$VERSION-rc$((RC+1))"
    git commit -a -m "bumped version to release candidate $RC"
fi