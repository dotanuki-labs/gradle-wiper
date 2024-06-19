#! /usr/bin/env bash
# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

set -e

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${dir%/*}"

readonly e2e_bin_home="target/e2e"

prepare_folder() {
    echo "→ Cleaning up existing files"
    rm -rf "$e2e_bin_home" && mkdir -p "$e2e_bin_home"
}

build_and_copy_release_binary() {
    local target=$1
    echo "→ Building target : $target"

    cargo build --release --target "$target"
    local binary="target/$target/release/gradle-wiper"
    cp "$binary" "$e2e_bin_home"/gradle-wiper
}

build_docker_image_for_tests() {
    echo "→ Building Docker image for tests"
    docker build . -t dotanuki-labs/gradle-wiper-tests -f e2e/Dockerfile
}

build_for_environment() {
    if [[ -z "$CI" ]]; then
        echo "→ Detected environment : local machine"
        build_and_copy_release_binary "aarch64-unknown-linux-gnu"
    else
        echo "→ Detected environment : CI machine"
        build_and_copy_release_binary "x86_64-unknown-linux-gnu"
    fi
}

echo
prepare_folder
build_for_environment
build_docker_image_for_tests
echo
