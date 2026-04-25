# Fractal CA Generator

A high-performance, pure Rust implementation of a Fractal Cellular Automata (CA) engine. This project unifies the evolution engine, semantic analysis (Chaos Agent), and neural memory (Titan) into a single, efficient binary.

## Core Features

- **Hilbert-Space Evolution**: Multi-dimensional CA state mapping.
- **Titan Neural Memory**: Native Online SGD implementation using `ndarray` for real-time state modulation.
- **Semantic Analysis**: Heuristic cluster detection and focal point extraction.
- **LLM Priming**: Chaos-driven context generation for enhanced AI prompting.
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
Runs the Chaos Agent to analyze a prompt and extract semantic focal points.
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

### 3. `titan`
Trains the Titan Neural Memory on a specific seed to learn its semantic signature.
- **Parameters**:
  - `<SEED>`: The seed to train on (Required).
  - `-s, --steps <N>`: Number of training steps (default: `100`).
  - `-w, --width <N>`: Grid width (default: `60`).
  - `-h, --height <N>`: Grid height (default: `30`).
- **Example**:
  ```bash
  ./target/release/fractal-ca titan "Quantum Logic Gates" --steps 1000
  ```

### 4. `prime`
Generates a "Chaos-Primed" instruction for an LLM (e.g., GPT-4, Claude).
- **Parameters**:
  - `<INSTRUCTION>`: The instruction to prime (Required).
  - `-i, --iterations <N>`: Evolution steps to determine "vibe" (default: `10`).
- **Example**:
  ```bash
  ./target/release/fractal-ca prime "Write a story about a clockmaker"
  ```

### 5. `observe`
Visualizes the CA evolution in the terminal in real-time.
- **Parameters**:
  - `<SEED>`: The seed string (Required).
  - `-d, --duration <SECONDS>`: How long to run (default: `0`, which runs for 100 iterations).
- **Example**:
  ```bash
  ./target/release/fractal-ca observe "Solar Flare" --duration 30
  ```

### 6. `deep-time`
Simulates long-term evolution (epochs: 1, 100, 500, 1000, 5000) and extracts temporal insights.
- **Parameters**:
  - `[PROMPT]`: Optional seed (default: Europa Orbital Research Station).
- **Example**:
  ```bash
  ./target/release/fractal-ca deep-time "Cybernetic Jungle"
  ```

### 7. `lab`
Runs the internal test suite, including determinism checks and entropy sweeps.
- **Example**:
  ```bash
  ./target/release/fractal-ca lab
  ```

---

## Combining Commands & Workflows

### The "Creative Loop"
1. **Observe** a seed to see if it produces interesting visual structures:
   ```bash
   ./target/release/fractal-ca observe "Bioluminescent Reef"
   ```
2. **Train** Titan memory to capture that seed's specific resonance:
   ```bash
   ./target/release/fractal-ca titan "Bioluminescent Reef" --steps 500
   ```
3. **Prime** a related prompt using the learned complexity metrics:
   ```bash
   ./target/release/fractal-ca prime "Describe the biology of a bioluminescent reef"
   ```

### Scripting with `agent` output
You can pipe the `agent` output into other tools or use it to generate logs:
```bash
./target/release/fractal-ca agent "Distributed Systems" > analysis.log
```

---

## API Integration

When running in `server` mode, you can interact with the engine via POST requests:

- `POST /api/v1/init`: Initialize a new lattice.
- `POST /api/v1/step`: Evolve the lattice (supports neural modulation fields).
- `GET /api/v1/formatted`: Retrieve the ASCII representation of the current state.

See `src/main.rs` for detailed JSON request/response schemas.
