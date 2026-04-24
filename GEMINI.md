# Fractal CA Generator Project Notes

## Chaotic Output Configuration
The length and complexity of chaotic outputs (ASCII art grids) can now be adjusted at runtime.

### Engine (Rust)
The engine now supports configurable dimensions and iterations via CLI and API.

**CLI Usage:**
```bash
cd engine && cargo run -- "<seed>" [iterations] [width] [height]
```
- `seed`: String used to initialize the Hilbert space.
- `iterations`: Number of steps to evolve (default: 10).
- `width`: Grid width (default: 120 for CLI, 80 for Server).
- `height`: Grid height (default: 60 for CLI, 40 for Server).

*Example:*
```bash
cd engine && cargo run -- "Quantum Chaos" 20 150 50
```

**API Usage:**
Start the server:
```bash
cd engine && cargo run
```
The server runs on `http://localhost:3000`.

- `/api/v1/init` (POST): Accepts `width`, `height`, `seed` (or `seed_prompt`), and `instruction_header`.
- `/api/v1/formatted` (GET/POST): Accepts an optional JSON body `{"steps": N}` to evolve the CA by N steps before returning the grid.
- `/api/v1/step` (POST): Accepts an optional `count` parameter to run multiple steps.

### Python Scripts
All Python scripts now include a shared utility (`engine_utils.py`) that automatically detects if the Rust engine is running. If the engine is not found on port 3000, the scripts will attempt to start it automatically using `cargo run --release`.

#### Chaos Agent
The `chaos_agent.py` script performs semantic analysis on the evolved CA state.

**Usage:**
```bash
python3 python/chaos_agent.py "Your Prompt" --iterations 20 --width 100 --height 50 --points 5
```
- `--iterations`: Deeper evolution into chaos.
- `--width`/`--height`: Larger ASCII maps.
- `--points`: Number of focal points to extract (default: 5).

#### Chaos Lab
Used for phase transition and entropy analysis.

**Usage:**
```bash
python3 python/chaos_lab.py
```

#### Deep Time
Simulates extremely long-term evolution (epochs up to 5000 steps).

**Usage:**
```bash
python3 python/deep_time.py
```

#### Titan Neural Memory
Training and observing the Titan architecture's interaction with the CA.

**Train:**
```bash
python3 python/titan_brain.py "Seed" [steps] [width] [height]
```

**Observe:**
```bash
python3 python/observe_titan.py "Seed" [steps]
```

## Bug Fixes & Improvements
- **Standardized API paths**: Removed `/lattice/` prefix from scripts to match the actual engine routes.
- **Improved Evolution Efficiency**: Added a `steps` parameter to the `/api/v1/formatted` endpoint, allowing multi-step evolution and state retrieval in a single round-trip.
- **Unified Parameter Support**: The engine now correctly handles `seed_prompt` as an alias for `seed` in the JSON payload, ensuring compatibility with all legacy and new scripts.
### Research Synthesis (April 2026)
Recent research validates the core pillars of the Fractal CA project:
- **Titan Alignment**: Our "Titan" feedback loop mirrors the "Titans: Learning to Memorize at Test Time" (ArXiv: 2501.00663) paradigm, which uses weight-updates during inference as a long-term memory module.
- **Reservoir Computing (ReCA)**: The CA grid acts as a high-dimensional reservoir for state persistence, a recognized technique for mapping signals into complex non-linear spaces.
- **Information Thermalization**: The observed entropy peaks (~11.0) confirm that 30 iterations push the system toward thermalization, necessitating observation in the "Pre-Thermalization window" (5-15 steps) to maintain prompt "vibes."
- **Fractal Globule Packing**: The project's "Fractal Folding" goal is supported by the Fractal Globule model (Science 326), which enables dense information packing while preserving locality.

*For detailed research citations and analysis, see [THEORETICAL_FOUNDATIONS.md](THEORETICAL_FOUNDATIONS.md).*

### Roadmap: Research-Driven Evolution
- **Fractal Globule Injection**: Implement a Hilbert-curve based folding algorithm in the Rust engine to maximize semantic locality.
- **Surprise-Based Titan Updates**: Refine the Titan brain feedback to use gradient-based "Surprise Metrics" for selecting CA states to memorize.
- **Pre-Thermalization Profiling**: Automate multi-interval state capture to find the optimal divergence window for different prompt types.
- **Persistent Modulation:** Instead of one-time seeding, the prompt could be converted into a constant "modulation field" (similar to the Titan feedback) that influences the Hilbert space at every step. This would make the prompt act as a "force" rather than just a "starting state."


### Creative Applications
- **Procedural World-Building:** Using large grids (200x80+) as "territory maps" where focal points represent semantic hubs or landmarks.
- **Sonic Mapping:** Mapping the complex probability fields of high-iteration runs into audio spectral filters for generative music.

## Bug Fixes & Improvements

