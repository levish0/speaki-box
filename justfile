set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

default:
    @just --list

# Format Rust source.
fmt:
    cargo fmt --all

# Check formatting without changing files.
fmt-check:
    cargo fmt --all -- --check

# Check native source and test targets.
check:
    cargo check --locked --all-targets

# Treat native Clippy warnings as errors.
lint:
    cargo clippy --locked --all-targets -- -D warnings

# Run behavior regression tests without opening a window.
test:
    cargo test --locked --lib

# Check the browser target without native platform features.
check-web:
    cargo check --locked --target wasm32-unknown-unknown --no-default-features

# Run the required automated checks.
verify: fmt-check lint test check-web

# Build the native debug executable.
build:
    cargo build --locked

# Build a distributable native executable with embedded assets.
build-release:
    cargo build --locked --release

# Run the ordinary window. Pass optional application arguments.
run *arguments:
    cargo run --locked -- {{arguments}}

# Run the transparent native window.
transparent:
    cargo run --locked -- --transparent

# Exercise Q input and save the native GPU output. The test window closes automatically.
smoke-window:
    cargo run --locked --example window_smoke

# Exercise the transparent renderer and save its GPU output.
smoke-transparent:
    cargo run --locked --example window_smoke -- --transparent

# Check 32 GPU frames with three fixed sprite expressions.
smoke-stationary:
    cargo run --locked --example window_smoke -- --transparent --stationary

# Observe direct presentation for 60 seconds. Optional flags: --no-msaa and --ldr.
probe-renderer *arguments:
    cargo run --locked --example window_smoke -- --transparent --stationary-live {{arguments}}

# Build the GitHub Pages site. Install Trunk first.
build-web:
    trunk build --release --public-url "/speaki-box/"

# Serve the browser app locally. Install Trunk first.
web:
    trunk serve
