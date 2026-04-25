# Fractal CA Generator

A high-performance, pure Rust implementation of a Fractal Cellular Automata (CA) engine. This project unifies the evolution engine, semantic analysis (Chaos Agent), and intrinsic neural memory (Titan) into a single, efficient binary.

## Core Features

- **Hilbert-Space Evolution**: Multi-dimensional CA state mapping using fractal folding seeding.
- **Continuous Semantic Modulation**: The prompt acts as a persistent field influencing every evolution step, preventing semantic decay.
- **Intrinsic Titan Memory**: Microscopic Online SGD embedded into every lattice cell.
- **Self-Optimizing Structure**: Dynamic, per-cell learning rates (`alpha_field`) that adapt based on local entropy and error gradients.
- **Semantic Analysis**: Heuristic cluster detection and focal point extraction for actionable AI insights.
- **LLM Priming**: Chaos-driven context generation with native memory loss metrics.
- **Native CLI**: Robust command-line interface with subcommands for all workflows.
- **REST API**: Integrated Axum server for remote integration.

## Installation

### Prerequisites
- [Rust & Cargo](https://rustup.rs/)

### Build
```bash
# Clone the repository
git clone https://github.com/Markon101/Fractal-CA-Generator
cd Fractal-CA-Generator

# Build the release binary
make build
```
The binary will be located at `target/release/fractal-ca`.

---

## Command Reference

The `fractal-ca` binary uses subcommands to handle different modes of operation.

### 1. `server`
Starts the engine as a REST API server.
- **Parameters**:
  - `-p, --port <PORT>`: Port to listen on (default: `3000`).
- **Example**:
  ```bash
  ./target/release/fractal-ca server --port 8080
  ```

### 2. `agent`
Runs the Chaos Agent to analyze a prompt and extract semantic focal points. Memory is updated natively during evolution.
- **Parameters**:
  - `<PROMPT>`: The input seed string (Required).
  - `-i, --iterations <N>`: Number of evolution steps (default: `15`).
  - `-w, --width <N>`: Grid width (default: `80`).
  - `-h, --height <N>`: Grid height (default: `40`).
  - `-p, --points <N>`: Max focal points to extract (default: `5`).
- **Example**:
  ```bash
  ./target/release/fractal-ca agent "Design a Martian colony" --iterations 30 --width 120
  ```

### 3. `prime`
Generates a "Chaos-Primed" instruction for an LLM (e.g., GPT-4, Claude). Includes native Titan memory loss metrics.
- **Parameters**:
  - `<INSTRUCTION>`: The instruction to prime (Required).
  - `-i, --iterations <N>`: Evolution steps to determine "vibe" (default: `10`).
- **Example**:
  ```bash
  ./target/release/fractal-ca prime "Write a story about a clockmaker"
  ```

### 4. `observe`
Visualizes the CA evolution and native memory loss in the terminal in real-time.
- **Parameters**:
  - `<SEED>`: The seed string (Required).
  - `-d, --duration <SECONDS>`: How long to run (default: `0`, which runs for 100 iterations).
- **Example**:
  ```bash
  ./target/release/fractal-ca observe "Solar Flare" --duration 30
  ```

### 5. `deep-time`
Simulates long-term evolution (epochs: 1, 100, 500, 1000, 5000) and extracts temporal insights using intrinsic learning.
- **Parameters**:
  - `[PROMPT]`: Optional seed (default: Europa Orbital Research Station).
- **Example**:
  ```bash
  ./target/release/fractal-ca deep-time "Cybernetic Jungle"
  ```

### 6. `lab`
Runs the internal test suite, including determinism checks, entropy sweeps, and native Titan loss validation.
- **Example**:
  ```bash
  ./target/release/fractal-ca lab
  ```

---

## Combining Commands & Workflows

### The "Creative Loop"
1. **Observe** a seed to see if it produces interesting visual structures and low memory loss:
   ```bash
   ./target/release/fractal-ca observe "Bioluminescent Reef"
   ```
2. **Analyze** with `agent` to extract actionable insights from the mature memory state:
   ```bash
   ./target/release/fractal-ca agent "Bioluminescent Reef" --iterations 50
   ```
3. **Prime** a related prompt using the complexity metrics derived from the intrinsic learning:
   ```bash
   ./target/release/fractal-ca prime "Describe the biology of a bioluminescent reef"
   ```

### Recursive Chaos (AI Agency)
Feed a directive about the system's own evolution into the engine to derive self-optimizing strategies:
```bash
./target/release/fractal-ca prime "Evolve the engine for hyper-dimensional feedback"
```

---

## API Integration

When running in `server` mode, you can interact with the engine via POST requests. Note: standalone memory modulation is no longer required as it is handled natively in the `step` logic.

- `POST /api/v1/init`: Initialize a new lattice with native memory.
- `POST /api/v1/step`: Evolve the lattice (Intrinsic Titan modulation).
- `GET /api/v1/formatted`: Retrieve the ASCII representation and memory state.

See `src/main.rs` and `src/core.rs` for detailed JSON request/response schemas.
