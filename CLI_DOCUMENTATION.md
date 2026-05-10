# 🌀 Titan-Hilbert Fractal CA: Command Reference

The `fractal-ca` binary is a multi-dimensional semantic engine that utilizes **Non-Local Semantic Entanglement** and **Titan Memory** to simulate the evolution of information.

---

## 1. Core Agent Operations
Use these commands for standard prompt analysis and "thought process" visualization.

### `agent`
Runs a simulation based on a prompt and identifies "Focal Points" of semantic density.
```bash
cargo run -- agent "The ethics of artificial general intelligence" \
    --iterations 30 \
    --width 100 --height 50 \
    --points 8 \
    --save-model "agi_model.safetensors"
```
*   `--iterations` / `-i`: Number of evolution steps (Default: 15).
*   `--width` / `-w`: Grid width (Default: 80).
*   `--height`: Grid height (Default: 40).
*   `--points` / `-p`: Number of focal clusters to identify (Default: 5).
*   `--load-model`: Path to a previously saved `.safetensors` Titan Memory.
*   `--save-model`: Path to save the resulting Titan Memory state.

### `observe`
Real-time terminal visualization of the lattice evolution.
```bash
cargo run -- observe "Cybernetic ecosystem" --duration 60
```
*   `--duration` / `-d`: How long to run in seconds. Use `0` for an infinite loop until it hits 100 iterations.

---

## 2. Cognitive Priming & Analysis
These commands are designed to generate high-signal reports for LLM steering.

### `prime`
Generates a **Chaos-Primed Cognitive Field Report**, identifying the optimal pre-thermalization window.
```bash
cargo run -- prime "Design a modular space station" --iterations 20 --homeostatic
```
*   `--iterations` / `-i`: Total steps to scan for peak Phi (Integrated Information).
*   `--homeostatic`: If set, injects "breathing" phase noise every 5 steps to prevent deep-attractor lock.

### `benchmark`
Runs a comparative analysis of different evolution strategies (Baseline vs. Phi-Optimized vs. Coherence-Optimized).
```bash
cargo run -- benchmark --samples 5
```

---

## 3. Self-Evolution & Deep Time
Explore recursive feedback loops and long-horizon stability.

### `self-prime`
**Recursive Evolution.** Uses the "Semantic Eigenstate" (the lattice's own interpretation) as the prompt for the next generation.
```bash
cargo run -- self-prime "Genesis" --generations 5 --iterations 15
```
*   `--generations` / `-g`: Number of recursive injection cycles.
*   `--iterations`: Steps per generation.
*   `--load-model` / `--save-model`: Persist Titan Memory across generations.

### `deep-time`
Observes how a prompt survives over thousands of iterations.
```bash
cargo run -- deep-time "Universal Constants"
```
*   Snapshots the lattice at epochs 1, 100, 500, 1000, and 5000.

---

## 4. Thermodynamic & Scientific Testing
Probe the fundamental information physics of the engine.

### `lab`
Runs the standard suite of Phase Transition Analysis tests.
```bash
cargo run -- lab
```
*   **Test 1**: Determinism (Seed stability).
*   **Test 2**: Entropy Sweep (Relationship between prompt length and density).
*   **Test 3**: Downward Causation (Macro-to-micro coupling impact).
*   **Test 4**: Non-Local Entanglement (Coupling vs. Global Phase Sync).

### `shock-test`
Tests the **Synchronized Resilience** of the lattice by injecting maximum entropy into its center.
```bash
cargo run -- shock-test "Stability Core"
```

### `perturb-test`
Injects continuous gentle noise (5% perturbation) to observe how the engine maintains homeostasis.
```bash
cargo run -- perturb-test "Breathing System"
```

### `self-test`
Validates the internal assumptions of the architecture (Provenance persistence and Phi proxy accuracy).
```bash
cargo run -- self-test "Architecture of Truth"
```

---

## 5. Deployment & Integration

### `server`
Starts the Axum-based REST API for remote lattice modulation.
```bash
cargo run -- server --port 3000
```
*   `--port` / `-p`: TCP port for the server (Default: 3000).

#### API Endpoints:
*   `POST /api/v1/init`: Initialize a new lattice state.
*   `POST /api/v1/step`: Advance the simulation.
*   `GET /api/v1/formatted`: Retrieve an ASCII-art snapshot of the current state.

---

## 6. Key Metrics Output
Regardless of the command, the engine outputs several high-dimensional metrics:
*   **Phi ($\Phi$)**: Integrated Information Potential. Measures the balance of differentiation and integration.
*   **Non-Local Coherence**: Measures how well the "Entanglement" is syncing conceptual clusters.
*   **Titan Work (Szilárd)**: The thermodynamic cost of the memory update.
*   **Arrow of Time**: The total accumulated state complexity (phase rotations).
