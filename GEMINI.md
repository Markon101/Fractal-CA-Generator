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
- **Visual Clarity**: Default CLI dimensions increased to 120x60 for more immersive chaotic patterns.

## Future Research & Structural Ideation

### Analysis of Large-Scale Evolution
Recent tests on 200x80 grids with 50+ iterations show that while the initial seed determines the starting "galaxy" of points, long-term evolution results in "Semantic Diffusion." The original prompt bytes act as a phase-offset seed that diffuses into complex filament structures and diagonal interference patterns.

### Structurally Effective Injection
Currently, the prompt is injected only at initialization. To increase structural resilience:
- **Persistent Modulation:** Instead of one-time seeding, the prompt could be converted into a constant "modulation field" (similar to the Titan feedback) that influences the Hilbert space at every step. This would make the prompt act as a "force" rather than just a "starting state."
- **Fractal Folding:** Mapping prompt bytes to a recursive geometric structure (like a Sierpinski triangle) within the grid could make the information more resilient to entropic decay.
- **Topological Prompting:** Letting the prompt define the grid's wrap-around logic (e.g., torus vs. sphere) would fundamentally change the system's structural evolution.

### Creative Applications
- **Procedural World-Building:** Using large grids (200x80+) as "territory maps" where focal points represent semantic hubs or landmarks.
- **Sonic Mapping:** Mapping the complex probability fields of high-iteration runs into audio spectral filters for generative music.
