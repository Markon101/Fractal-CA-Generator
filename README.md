# Fractal CA Generator

A pure Rust implementation of a Fractal Cellular Automata (CA) engine with integrated semantic analysis, Titan Neural Memory, and LLM priming capabilities.

## Features

- **Fractal CA Engine**: A 2D Hilbert-space evolution engine using fractal folding for seeding.
- **Titan Neural Memory**: Online learning architecture that warps the CA evolution based on learned states.
- **Chaos Agent**: Analyzes chaotic states to extract semantic focal points and actionable insights.
- **LLM Priming**: Generates high-context prompts based on CA entropy, density, and resonance.
- **Deep Time Simulation**: Observes long-term evolution of semantic structures.
- **Interactive Observation**: Real-time terminal visualization of the CA.
- **API Server**: Axum-based REST API for remote integration.

## Installation

Ensure you have Rust installed.

```bash
cargo build --release
```

## Usage

The project is a single binary with multiple subcommands.

### Run the API Server
```bash
./target/release/fractal-ca server --port 3000
```

### Analyze a Prompt (Chaos Agent)
```bash
./target/release/fractal-ca agent "Design a decentralized energy grid"
```

### Train Titan Neural Memory
```bash
./target/release/fractal-ca titan "Quantum Consciousness" --steps 500
```

### Generate Primed Instruction
```bash
./target/release/fractal-ca prime "Explain the butterfly effect in 5 sentences"
```

### Real-time Observation
```bash
./target/release/fractal-ca observe "Nebula Formation"
```

### Deep Time Evolution
```bash
./target/release/fractal-ca deep-time
```

### Laboratory Experiments
```bash
./target/release/fractal-ca lab
```

## Architecture

- **Engine**: Rust (Hilbert-space CA)
- **Neural Memory**: Integrated Online SGD (Rust/ndarray)
- **API**: Axum
- **CLI**: Clap
