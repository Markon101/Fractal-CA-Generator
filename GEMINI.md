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

## Theoretical Alignment
- **Titans: Learning to Memorize**: Native implementation of weight-updates during inference.
- **Information Thermalization**: Optimized pre-thermalization capture windows (5-15 steps) now defaults in `agent` mode.
- **Fractal Globule Packing**: Seeding logic remains Hilbert-curve inspired for maximum semantic locality.
