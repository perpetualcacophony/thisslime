#!/bin/bash
set -e

VERSION=$(sed -En 's|workspace.package.version = "(.*)"|\1|p' ./Cargo.toml)

function fix_dependencies () {
    sed -Ei "s|(dependencies.thisslime-.+.version =)(.*)|\1 \"=$VERSION\"|g" ./$1/Cargo.toml
}

fix_dependencies thisslime-derive
fix_dependencies thisslime