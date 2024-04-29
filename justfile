# General tasks driven by Just
# https://github.com/casey/just

# Install and activate Rust toolchain
toolchain:
    @echo "→ Install and activate Rust toolchain"
    rustup show active-toolchain
    @echo

# Install Cargo plugins (local)
cargo-plugins-local:
    @echo "→ Installing Cargo Binstall"
    ./scripts/cargo-binstaller.sh
    @echo

    @echo "→ Installing Cargo plugins (local)"
    yes | cargo binstall cargo-nextest --secure --force
    yes | cargo binstall cargo-get --secure --force
    yes | cargo binstall cargo-msrv --secure --force
    @echo

# Performs setup for this project (local)
setup-local: toolchain cargo-plugins-local
    @echo
    @echo "✅ Setup (local) concluded"
    @echo

# Check code formatting and smells
lint: toolchain
    @echo "→ Checking code formatting (rustfmt)"
    cargo fmt --check
    @echo

    @echo "→ Checking code smells (clippy)"
    cargo clippy --all-features -- -D warnings -W clippy::unwrap_used
    @echo

# Run project tests and check for errors
checks:
    @echo "→ Run project tests"
    cargo check --all-features
    cargo nextest run
    @echo

# Install required Cargo plugins (CI)
cargo-plugins-ci:
    @echo "→ Installing Cargo Binstall"
    ./scripts/cargo-binstaller.sh
    @echo

    @echo "→ Installing Cargo plugins (CI)"
    yes | cargo binstall cargo-deny --secure --force
    yes | cargo binstall cargo-cyclonedx --secure --force
    yes | cargo binstall cargo-nextest --secure --force
    yes | cargo binstall cargo-get --secure --force
    yes | cargo binstall cargo-msrv --secure --force
    yes | cargo binstall cargo-zigbuild --secure --force
    @echo

# Performs setup for this project (CI)
setup-ci: toolchain cargo-plugins-ci
    @echo "✅ Setup (CI) concluded"
    @echo

# Build project according to local or CI environment
flexible-build:
    @echo "→ Build project according to local or CI environment"
    ./scripts/flex-build.sh
    @echo

# Generates supply-chain related artifacts
supply-chain-checks:
    @echo "→ Checking supplying chain"
    cargo deny check
    @echo

    @echo "→ Generating SBOMs"
    cargo cyclonedx --format json
    @echo

# Check MSRV
msrv-check:
    @echo "→ Checking minimum supported Rust version (MSRV)"
    cargo msrv verify
    @echo

# Running E2E tests
e2e suite:
    @echo "→ Preparing Docker image for tests"
    ./scripts/prepare-e2e.sh
    @echo

    @echo "→ Running E2E tests"
    docker run dotanuki-labs/gradle-wiper-tests {{suite}}
    @echo
