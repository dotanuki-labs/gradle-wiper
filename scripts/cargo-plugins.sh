#! /usr/bin/env bash
# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

set -euo pipefail

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${dir%/*}"

grep "cargo-" "cargo-plugins.toml" |
    tr -d '\"' |
    sed "s/[[:space:]]=[[:space:]]/@/g" |
    xargs -L1 -I {} cargo-binstall {} --no-confirm --force --quiet
