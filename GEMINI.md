# Fractal CA Generator Project Notes

## Chaotic Output Configuration
The length and complexity of chaotic outputs (ASCII art grids) can now be adjusted at runtime.

### Engine (Rust)
The engine now supports configurable dimensions and iterations via CLI and API.

**CLI Usage:**
```bash
cd engine && cargo run -- "<seed>" [iterations] [width] [height]
```
- `iterations`: Number of steps to evolve (default: 10)
- `width`: Grid width (default: 120 for CLI, 80 for Server)
- `height`: Grid height (default: 60 for CLI, 40 for Server)

**API Changes:**
- `/api/v1/init` (POST): Accepts `width`, `height`, `seed` (or `seed_prompt`), and `instruction_header`.
- `/api/v1/formatted` (GET/POST): Accepts an optional JSON body `{"steps": N}` to evolve the CA by N steps before returning the grid.
- `/api/v1/step` (POST): Accepts an optional `count` parameter to run multiple steps.

### Chaos Agent (Python)
The `chaos_agent.py` has been updated to support command-line arguments for all parameters.

**Usage:**
```bash
python3 python/chaos_agent.py "Your Prompt" --iterations 20 --width 100 --height 50 --points 5
```
- `--iterations`: Deeper evolution into chaos.
- `--width`/`--height`: Larger ASCII maps.
- `--points`: Extract more focal points for longer reasoning output.

## Bug Fixes
- Standardized API paths (removed `/lattice/` prefix from scripts).
- Fixed `seed_prompt` vs `seed` discrepancy.
- Improved evolution efficiency by using the `steps` parameter in `formatted` requests.
