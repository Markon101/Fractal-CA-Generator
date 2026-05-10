# Fractal CA Generator: Upgrade Report (May 2026)

## Overview
The Fractal CA Generator has been upgraded from an evocative semantic reactor into a rigorous, deterministic, and trajectory-aware **Prompt Compiler**. The system now provides traceable evidence of how prompt concepts evolve into lattice attractors.

## Key Upgrades
### 1. BPE Tokenization (Full LLM Alignment)
- **tiktoken-rs Integration**: Replaced dictionary-based seeding with the GPT-4 `cl100k_base` BPE tokenizer.
- **Token-Level Provenance**: Every cell now tracks the index of the original BPE token it represents, allowing for exact mapping back to prompt fragments (including sub-word tokens).
- **Direct Decoding**: Emergent eigenstates are now decoded directly from BPE token IDs, producing high-signal technical and linguistic field translations.

### 2. Adaptive Heartbeat (The Dynamic Vitality Mechanism)
- **Integrated Information Monitoring**: The system now tracks $\Phi$ in real-time.
- **Surprise-Based Perturbation**: Triggers an "Adaptive Heartbeat" (entropy injection) if the system stalls (flat $\Phi$) or collapses (rapid $\Phi$ decay).
- **Attractor Breaking**: The heartbeat injects a ripple of phase noise to break deep-attractor locks, keeping the system in a critical "alive" state for longer durations.

### 3. Spatial Semantic Manifolds (Ribbon Topology)
- **Topological Mapping**: Implemented a "Ribbon Manifold" that projects the 1D prompt sequence into a 2D space.
- **Locality-Preserving Seeding**: Uses bit-interleaving of BPE token IDs to ensure that conceptually related tokens (which often have proximal IDs) are physically clustered in the lattice.
- **Narrative Flow Preservation**: The $X$ axis is dedicated to prompt sequence position, while the $Y$ axis represents the semantic vocabulary space.

### 4. Trajectory-Aware Priming
...
- **Time-Lapse Profiling**: The `prime` command now captures metrics at multiple iterations (3, 5, 8, 10, 15, 20).
- **Optimal Window Selection**: The system automatically selects the "best" priming iteration by identifying the peak of **Integrated Information Potential ($\Phi$)** in the pre-thermalization window.
- **Trend Detection**: Tracks whether $\Phi$ is ascending (integrating) or descending (differentiating) at the point of capture.

### 4. Redesigned `prime` Output
The `prime` command now emits a structured **Chaos-Primed Cognitive Field Report** optimized for LLM consumption.
- **Mode Classification**: Categorizes the regime as Divergent-Creative, Integrative-Modular, Analytical, etc.
- **Grounded Attractors**: Lists prompt words that survived as stable dynamic clusters.
- **Reasoning Directive**: Provides specific instructions for the downstream LLM based on the detected regime.

### 5. Benchmark Harness
- Added a `benchmark` command that evaluates priming strategies across three categories: Reasoning, Coding, and Creative.
- Compares **Raw Baseline**, **Phi-Optimized**, and **Coherence-Optimized** trajectories.
- Generates artifacts ready for LLM-based comparative scoring.

### 6. Homeostatic Mode (Breathing Chaos)
- Added an optional `--homeostatic` flag to the `prime` command.
- When enabled, the system applies gentle 1% phase perturbation every 5 steps.
- This prevents the system from locking into hyper-dense deep attractors and maintains a more "alive" regime.

## How to Run

### Generate a Primed Field
```bash
cargo run -- prime "Your complex prompt here"
```

### Enable Homeostatic Perturbation
```bash
cargo run -- prime "Your complex prompt here" --homeostatic
```

### Run the Benchmark Harness
```bash
cargo run -- benchmark
```

## Technical Implementation Details
- **Cell Update**: `source_idx: i32` added to `Cell`.
- **Lattice Dynamics**: Provenance propagation logic inserted into the main `step()` loop.
- **Dictionary Integration**: 10,000 common words used for both seeding and eigenstate extraction.
- **Mode Logic**: Threshold-based classification using Entropy, Resonance, Coherence, and Phi.

## Tradeoffs and Future Work
- **Tradeoff**: Word-level provenance is more meaningful but requires dictionary-based tokenization.
- **Next Upgrades**:
  1. **Tokenization**: Integrate a real BPE tokenizer (e.g. `tiktoken-rs`) for full LLM compatibility.
  2. **Automated Evaluation**: Integrate an LLM API into the benchmark harness for end-to-end scoring.
  3. **Multi-Channel Provenance**: Allow cells to track multiple source influences via sparse maps.
