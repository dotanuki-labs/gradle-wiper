#! /usr/bin/env bash
# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

set -euo pipefail

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${dir%/*}"

echo "Setting up required tools"

if ! which asdf >/dev/null; then
    echo -e "Error : 'asdf' required but not available"
    echo
    exit 1
fi

echo
asdf plugin add just || true
asdf install
echo

echo "Done!"
echo
