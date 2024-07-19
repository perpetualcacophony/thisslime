#!/bin/bash
set -e

sed -Ei "s|version = \"(.*)\"|version = \"$1\"|g" ./Cargo.toml

./scripts/fix_versions.sh