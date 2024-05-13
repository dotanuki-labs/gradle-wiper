# General SDLC tasks driven by Just
# https://just.systems

_default:
  @just --list --unsorted

# Performs setup for this project
setup:
    @echo "→ Installing and activating Rust toolchain"
    rustup show active-toolchain
    @echo
    @echo "→ Installing Cargo Binstall"
    ./scripts/cargo-binstaller.sh
    @echo
    @echo "→ Installing Cargo plugins (CI)"
    ./scripts/cargo-plugins.sh
    @echo
    @echo "✅ Setup concluded"
    @echo

# Checks minimum supported Rust toolchain version
msrv:
    @echo "→ Checking minimum supported Rust toolchain version (MSRV)"
    cargo msrv verify
    @echo

# Checks code formatting and smells
lint:
    @echo "→ Checking code formatting (rustfmt)"
    cargo fmt --check
    @echo

    @echo "→ Checking code smells (clippy)"
    cargo clippy --all-features -- -D warnings -W clippy::unwrap_used
    @echo

# Checks compilation errors
compile:
    @echo "→ Checking for compilation errors"
    cargo check --all-features
    @echo

# Runs unit/module tests
tests:
    @echo "→ Running unit/module tests"
    cargo nextest run
    @echo

# Builds binaries according to local or CI environment
assemble:
    @echo "→ Building project according to local or CI environment"
    ./scripts/flex-build.sh
    @echo

# Runs supply-chain checks and generates SecOps artifacts
security:
    @echo "→ Checking supplying chain"
    cargo deny check
    @echo

    @echo "→ Generating SBOMs"
    cargo cyclonedx --format json
    @echo

# Runs E2E/Component tests, where suite is 'ram' or 'disk'
e2e suite:
    @echo "→ Preparing Docker image for tests"
    ./scripts/prepare-e2e.sh
    @echo

    @echo "→ Running E2E tests"
    docker run dotanuki-labs/gradle-wiper-tests {{suite}}
    @echo
