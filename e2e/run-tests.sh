#! /usr/bin/env bash
# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

set -euo pipefail

readonly target="${1:-disk}"

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${dir%/*}"

echo

case "$target" in
"disk")
    bats /usr/disk.bats
    ;;
"ram")
    bats /usr/ram.bats
    ;;
*)
    echo "Error: unsupported test target → $target"
    echo
    exit 1
    ;;
esac

echo
