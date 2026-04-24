#!/bin/bash

# Fractal CA Generator - Linux/macOS Install Script

echo "Checking prerequisites..."

# Check for Rust
if ! command -v cargo &> /dev/null; then
    echo "Rust not found. Installing via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust found: $(cargo --version)"
fi

# Check for Python
if ! command -v python3 &> /dev/null; then
    echo "Error: python3 is not installed."
    exit 1
fi

echo "Setting up Python virtual environment..."
python3 -m venv venv
source venv/bin/activate

echo "Installing Python dependencies..."
pip install --upgrade pip
pip install -r python/requirements.txt

echo "Building Rust engine..."
cd engine
cargo build --release
cd ..

echo "----------------------------------------"
echo "Installation complete!"
echo "To start, run:"
echo "source venv/bin/activate"
echo "cd engine && cargo run --release"
echo "----------------------------------------"
