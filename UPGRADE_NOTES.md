# Fractal CA Generator: Upgrade Report (May 2026)

## Overview
The Fractal CA Generator has been upgraded from an evocative semantic reactor into a rigorous, deterministic, and trajectory-aware **Prompt Compiler**. The system now provides traceable evidence of how prompt concepts evolve into lattice attractors.

## Key Upgrades

### 1. Deterministic Provenance Tracking
- **Word-Based Tokenization**: Upgraded from byte-level seeding to word-level seeding. The system now splits prompts into tokens and maps them to a robust 10,000-word dictionary.
- **Provenance Layer**: Added `source_idx` to the `Cell` struct. Each cell now tracks the index of the original prompt *word* it was seeded with.
- **Reverse Mapping**: `extract_grounded_attractors()` now performs deduplicated word-level influence mapping, providing a clear link between evolved clusters and prompt concepts.

### 2. Expanded Semantic Vocabulary
- **10k Dictionary**: Integrated the `google-10000-english` word list. The system's emergent phase-language (eigenstate) is now significantly more descriptive and high-signal.
- **Rarity Bias**: The eigenstate extraction now favors "resonant" (rare) words from the latter half of the dictionary when probability density is high, avoiding common stop-words and increasing the signal-to-noise ratio of the field translation.

### 3. Trajectory-Aware Priming
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
