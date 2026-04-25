# Fractal CA Generator Project Notes

## Pure Rust Transformation (April 2026)
The project has been fully converted to a pure Rust implementation. The Python scripts have been decommissioned and their logic integrated directly into the `fractal-ca` binary.

### Unified CLI Usage
The engine now supports multiple operational modes via subcommands:

**Chaos Agent:**
```bash
cargo run --release -- agent "<prompt>" [iterations] [width] [height]
```

**Titan Neural Memory:**
```bash
cargo run --release -- titan "<seed>" --steps 500
```

**LLM Priming:**
```bash
cargo run --release -- prime "<instruction>"
```

**Server Mode:**
```bash
cargo run --release -- server --port 3000
```

### Key Improvements
- **Zero Python Dependencies**: Reduced overhead and simplified deployment.
- **Native Neural Memory**: Titan Online Learning now runs natively in Rust using `ndarray`.
- **Integrated Metrics**: Entropy, density, and resonance calculations are now core engine features.
- **Performance**: Evolution and analysis are significantly faster due to the elimination of JSON-over-HTTP overhead for local CLI tasks.

## Research-Driven Evolution (Refinement)

### Continuous Semantic Modulation
We have moved beyond static seeding. The engine now implements **Continuous Semantic Modulation**, where the prompt is mapped into a persistent `semantic_field`. This field biases the Hilbert space rotation at *every* step of the evolution.
- **Result**: The "vibe" of the prompt is physically scattered through every frame of the generation, preventing semantic decay during thermalization.

### Modular Rust Architecture
To improve scalability and maintainability, the core engine has been refactored into a modular structure:
- `src/core.rs`: Contains the fundamental `Cell` and `TitanMemory` definitions.
- `src/lib.rs`: Exposes the core modules.
- `src/main.rs`: Handles CLI orchestration, API routing, and Lattice evolution logic.

### Recursive Chaos Loops (AI Agency)
During development, the engine was used to self-prompt for architectural improvements. By feeding agentic directives into the `prime` and `agent` modes, we derived strategies for refactoring the project itself, proving the utility of the "Chaos-Primed Execution Context."

## Theoretical Alignment
- **Titans: Learning to Memorize**: Native implementation of weight-updates during inference.
- **Information Thermalization**: Optimized pre-thermalization capture windows (5-15 steps) now defaults in `agent` mode.
- **Fractal Globule Packing**: Seeding logic remains Hilbert-curve inspired for maximum semantic locality.
