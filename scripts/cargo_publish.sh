#!/bin/bash
set -e

cargo publish -p thisslime-core
cargo publish -p thisslime-derive
cargo publish -p thisslime