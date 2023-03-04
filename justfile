set dotenv-load

# List all just receipes
default:
    @just --list

# Install require tools
requirements:
    @echo "Install Rust nightly for formatting"
    rustup toolchain add nightly
    @echo "Install cargo-nextest to run test"
    cargo install cargo-nextest
    @echo "Install cargo-nextest for tdd"
    cargo install cargo-watch
    @echo "Install cargo-audit for audit"
    cargo install cargo-audit
    @echo "Install cargo-deny for audit"
    cargo install cargo-deny
    @echo "Install bat"
    cargo install bat
    @echo "Install cargo-smart-release for release"
    cargo install cargo-smart-release

# Run TDD mode
tdd:
    cargo watch -c -s "just check"

# Help of the application
help:
    cargo run --quiet -- --help

# Launch tests
test:
    @echo "🧪 Testing..."
    cargo nextest run
    cargo test --doc

# Format the code
format:
    cargo +nightly fmt
    cargo sort --workspace --grouped

# Format the code
lint:
    @echo "🎩 Linting..."
    cargo check --all-features
    cargo clippy --all-features

# Check the code (formatting, lint, and tests)
check:
    @echo "🦀 Check formatting..."
    cargo +nightly fmt --all -- --check
    cargo sort --workspace --grouped --check
    @just lint
    @just test

# Audit (security issue, licences)
audit:
    @echo "🚨 Audit CVE..."
    cargo audit

    @echo "🪪 Check licences..."
    cargo deny check

# Build in production mode
build:
    cargo build --release

# Build the documentation
doc:
    cargo doc

# Install to the cargo bin path
install:
    cargo install --path .

# Release
release *ARGS:
    cargo smart-release --update-crates-index gitmoji-rs {{ARGS}}