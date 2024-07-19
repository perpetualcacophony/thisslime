#!/bin/bash
set -e

sed -Ei "s|workspace.package.version = \"(.*)\"|workspace.package.version = \"$1\"|g" ./Cargo.toml

./scripts/fix_versions.sh