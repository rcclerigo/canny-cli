#!/usr/bin/env bash
set -euo pipefail

info()  { printf "\033[1;34m==> %s\033[0m\n" "$1"; }
ok()    { printf "\033[1;32m==> %s\033[0m\n" "$1"; }
warn()  { printf "\033[1;33m==> %s\033[0m\n" "$1"; }
fail()  { printf "\033[1;31m==> %s\033[0m\n" "$1"; exit 1; }

# --- Xcode Command Line Tools ---
info "Checking for Xcode Command Line Tools..."
if xcode-select -p &>/dev/null; then
    ok "Xcode Command Line Tools found"
else
    warn "Xcode Command Line Tools not found — installing..."
    xcode-select --install
    echo ""
    echo "    A system dialog should have appeared."
    echo "    Complete the installation, then re-run this script."
    echo ""
    exit 1
fi

# --- Rust toolchain ---
info "Checking for Rust toolchain..."
if command -v cargo &>/dev/null; then
    ok "Rust toolchain found ($(rustc --version))"
else
    warn "Rust toolchain not found — installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    ok "Rust toolchain installed ($(rustc --version))"
fi

# --- Build ---
info "Building canny CLI..."
cargo build --release

# --- Install ---
INSTALL_DIR="${CARGO_HOME:-$HOME/.cargo}/bin"
cp target/release/canny "$INSTALL_DIR/canny"
ok "Installed canny to $INSTALL_DIR/canny"

# --- Verify ---
if command -v canny &>/dev/null; then
    echo ""
    ok "Done! Run 'canny auth' to get started."
else
    echo ""
    ok "Done! You may need to add $INSTALL_DIR to your PATH, then run 'canny auth' to get started."
fi
