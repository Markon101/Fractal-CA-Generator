# Fractal CA Generator

A hybrid generative system combining a high-performance Rust engine for Hilbert space Cellular Automata (CA) with Python-based neural analysis and reinforcement learning agents.

## Features
- **Rust Engine**: Fast Hilbert space evolution with an Axum-based REST API.
- **Python Agents**: Titan neural memory, RL brains, and chaos analysis tools.
- **Configurable Chaos**: Dynamic grid sizing and iteration control.

## Prerequisites
- [Rust](https://rustup.rs/) (1.70+)
- [Python](https://www.python.org/) (3.10+)
- [MicroMamba](https://mamba.readthedocs.io/en/latest/micromamba-installation.html) (Optional, recommended for environment management)

## Quick Start

### 1. Automatic Installation

#### Linux/macOS
```bash
chmod +x install.sh
./install.sh
```

#### Windows
```batch
install.bat
```

### 2. Manual Setup with MicroMamba

If you prefer using MicroMamba for dependency management:

```bash
# Create and activate environment
micromamba create -n fractal-ca python=3.10 -c conda-forge
micromamba activate fractal-ca

# Install Python dependencies
pip install -r python/requirements.txt

# Build the engine
cd engine
cargo build --release
cd ..
```

## Usage

### Starting the Engine
The engine provides the core simulation logic via a REST API.
```bash
cd engine
cargo run --release
```

### Running Analysis Agents
With the engine running, you can run various Python scripts:
```bash
# Semantic chaos analysis
python3 python/chaos_agent.py "Quantum Seed"

# Titan Neural Memory observation
python3 python/observe_titan.py "Seed" 50
```

Refer to `GEMINI.md` for detailed CLI arguments and API documentation.
