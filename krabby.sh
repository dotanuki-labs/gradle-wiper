#! /usr/bin/env bash
# Copyright 2024 Dotanuki Labs
# SPDX-License-Identifier: MIT

set -e

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$dir"

readonly callinectes="ghcr.io/dotanuki-labs/callinectes:latest@sha256:bc29aa196efec8d72ffc986319c86e0295f4a11b6a4cde5330e9c60ab5af693a"
readonly output_dir="artifacts"
readonly e2e_bin_home="target/e2e"
readonly task="$1"
readonly argument="$2"

usage() {
    echo
    echo "Available tasks:"
    echo
    echo "setup             # Installs required Cargo extensions"
    echo "lint              # Check code formatting and smells"
    echo "tests             # Run tests for Rust modules"
    echo "assemble          # Builds binaries according to the environment (local or CI)"
    echo "e2e <target>      # Runs E2E tests against a target ('ram' or 'disk')"
    echo "security          # Run security checks and generates supply-chain artifacts"
    echo "sbom              # Generates cycloneDX SBOM from project dependencies"
    echo "prepare-release   # Prepare assets for Github release"
    echo
}

setup_rust_toolchain() {
    echo "🔥 Installing and activating Rust toolchain"
    rustup show active-toolchain
    echo
}

check_code_smells() {
    echo
    echo "🔥 Checking code smells for Rust code"
    echo
    docker run --rm -v "${PWD}:/usr/src" "$callinectes" fmt clippy
}

run_cargo_tests() {
    echo
    echo "🔥 Running unit + integration tests for Rust code"
    echo
    cargo test
    echo
}

build_binaries() {
    echo
    echo "🔥 Building project according to environment"
    echo
    local gha_runner="${RUNNER_OS:-local}"
    local platform

    echo "Detected environment → $gha_runner"

    case "$gha_runner" in
    "local")
        cargo build --release
        exit 0
        ;;
    "macOS")
        platform="apple-darwin"
        ;;
    "Linux")
        platform="unknown-linux-gnu"
        ;;
    *)
        echo "Error: unsupported environment → $gha_runner"
        echo
        exit 1
        ;;
    esac

    rm -rf "$output_dir" && mkdir -p "$output_dir"

    for arch in x86_64 aarch64; do
        local target="$arch-$platform"
        rustup target add "$target"
        cargo build --release --target "$target"

        local binary="target/$target/release/gradle-wiper"
        cp "$binary" "$output_dir"/gradle-wiper-"$target"
        chmod +x "$output_dir"/gradle-wiper-"$target"
    done
}

e2e() {
    echo "→ Cleaning up existing files"
    rm -rf "$e2e_bin_home" && mkdir -p "$e2e_bin_home"

    local target=

    if [[ -z "$CI" ]]; then
        echo "→ Detected environment : local machine"
        target="aarch64-unknown-linux-musl"
    else
        echo "→ Detected environment : CI machine"
        target="x86_64-unknown-linux-musl"
    fi

    echo "→ Building target : $target"
    rustup target add "$target"
    cargo build --release --target "$target"
    local binary="target/$target/release/gradle-wiper"
    cp "$binary" "$e2e_bin_home"/gradle-wiper

    echo "→ Building Docker image for tests"
    docker build . -t dotanuki-labs/gradle-wiper-tests -f e2e/Dockerfile

    echo "→ Running E2E tests"
    docker run dotanuki-labs/gradle-wiper-tests "$argument"
    echo
}

check_supply_chain() {
    echo
    echo "🔥 Checking dependencies and supply-chain"
    echo
    docker run --rm -v "${PWD}:/usr/src" "$callinectes" msrv machete deny
}

generate_cyclonedx_sbom() {
    echo
    echo "🔥 Generating cycloneDX SBOM"
    echo
    docker run --rm -v "${PWD}:/usr/src" "$callinectes" cyclonedx
}

compute_checksums() {
    readonly checksums="checksums.txt"

    cd "$output_dir"
    touch "$checksums"
    find . -name 'gradle-wiper-*' -exec sha256sum {} \; |
        sed "s/\.\///g" |
        sed "s/gradle-wiper-binaries-macOS\///g" |
        sed "s/gradle-wiper-binaries-Linux\///g" >"$checksums"
}

export_release_version() {
    version=$(grep 'version' Cargo.toml | head -1 | sed "s/version[[:space:]]=[[:space:]]//g" | tr -d '"')
    echo "version=$version" >>"$GITHUB_OUTPUT"
}

prepare_github_release() {
    compute_checksums
    export_release_version
}

if [[ -z "$task" ]]; then
    usage
    exit 0
fi

case "$task" in
"setup")
    setup_rust_toolchain
    ;;
"lint")
    check_code_smells
    ;;
"tests")
    run_cargo_tests
    ;;
"assemble")
    build_binaries
    ;;
"security")
    check_supply_chain
    ;;
"sbom")
    generate_cyclonedx_sbom
    ;;
"e2e")
    e2e
    ;;
"prepare-release")
    prepare_github_release
    ;;
*)
    echo "Error: unsupported task → $task"
    usage
    exit 1
    ;;
esac
