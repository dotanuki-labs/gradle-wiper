#! /usr/bin/env bash
# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

set -euo pipefail

readonly output_dir="target/ci"

cross_compile() {
    local target="$1"

    rustup target add "$target"
    cargo zigbuild --release --target "$target"

    local binary="target/$target/release/gradle-wiper"
    cp "$binary" "$output_dir"/gradle-wiper-"$target"
    chmod +x "$output_dir"/gradle-wiper-"$target"
    sha256sum "$binary" >>"$output_dir"/gradle-wiper-"$target"-sha256
}

cross_build_full() {
    for platform in apple-darwin unknown-linux-gnu; do
        for arch in x86_64 aarch64; do
            cross_compile "$arch-$platform"
        done
    done
}

cross_build_simple() {
    cross_compile "x86_64-unknown-linux-gnu"
}

usage() {
    echo "Usage"
    echo
    echo "‣ cross-build.sh (default mode : simple)"
    echo "‣ cross-build.sh simple"
    echo "‣ cross-build.sh full"
}

readonly mode="${1:-simple}"

echo

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${dir%/*}"

rm -rf "$output_dir" && mkdir -p "$output_dir"

case "$mode" in
"simple")
    cross_build_simple
    ;;
"full")
    cross_build_full
    ;;
*)
    echo "Error: Invalid cross-build mode → $mode"
    usage
    echo
    exit 1
    ;;
esac

echo
