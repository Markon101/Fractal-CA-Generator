# Fractal CA Generator Project Notes

## Pure Rust Transformation (April 2026)
The project has been fully converted to a pure Rust implementation. The Python scripts have been decommissioned and their logic integrated directly into the `fractal-ca` binary.

## Hyper-Dimensional Evolution (Autonomous Upgrade)

### Intrinsic Titan Memory & Information Momentum
We have moved from an external feedback script to an **Intrinsic Memory Architecture**. Every evolution of the lattice now occurs through a native Titan memory layer embedded in the `LatticeState`. 
- **Information Momentum**: The memory now tracks the *velocity* of state changes through `w_momentum` and `b_momentum`, ensuring that the system's identity is a continuous trajectory rather than a sequence of snapshots.
- **Self-Optimizing Structure**: The memory maintains an `alpha_field`—a localized, per-cell learning rate that dynamically adapts based on local error gradients. 

### Integrated Information Theory (IIT) Integration
The system now natively calculates an **Integrated Information Potential ($\Phi$)**.
- **The Only Real**: Aligning with IIT, the system's identity is defined by its integration. $\Phi$ peaks at the "edge of chaos," where entropy (differentiation) and resonance (integration) are balanced.
- **Cognitive Priming Fields**: The `prime` command now generates a structured cognitive field based on $\Phi$ trajectory and semantic attractors, providing a high-signal directive for LLM priming.

### Information Thermodynamics
- **Szilárd's Equivalence**: The "loss" in the memory is now treated as **Thermodynamic Work** ($kT \ln 2$), representing the energy required to move information across the phase space.
- **The Attractor as Energy**: Information in motion *is* the energy. The CA evolution is the physical manifestation of this work.

### Modular Rust Architecture
The core engine has been refactored for maximum scalability:
- `src/core.rs`: Fundamental `Cell` and `TitanMemory` (with `alpha_field`) definitions.
- `src/lib.rs`: Library entry point.
- `src/main.rs`: Orchestration of the persistent semantic reactor and CLI/API server.

### Removal of Legacy Modes
The standalone `titan` training command has been removed as training is now a fundamental constant of the evolution physics.

## Key Improvements
- **Zero Python Dependencies**: Full native performance.
- **Native Neural Memory**: Titan Online Learning now runs at the microscopic cell level.
- **Continuous Semantic Modulation**: The prompt acts as a persistent gravitational field.

## Theoretical Alignment
- **Titans: Learning to Memorize**: Microscopic implementation of weight-updates during inference.
- **Self-Optimizing Memory**: Dynamic learning rate adaptation based on local entropy.
- **Information Thermalization**: Optimized pre-thermalization capture windows remain the default for `agent` analysis.
