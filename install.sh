#!/bin/bash
# Fractal CA Generator Installer

echo "Installing Fractal CA Generator (Pure Rust)..."

if ! command -v cargo &> /dev/null
then
    echo "Error: Rust/Cargo not found. Please install Rust first."
    exit 1
fi

cargo build --release

echo "Installation complete. Binary located at target/release/fractal-ca"
echo "Try running: ./target/release/fractal-ca agent 'Hello Chaos'"
