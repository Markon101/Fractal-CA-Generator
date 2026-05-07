# Fractal CA Neural Architecture: The Titan Memory Model

This document provides a detailed breakdown of the neural network component (the "Titan Memory") embedded within the Fractal Cellular Automaton (CA) Generator. It explains its mechanics, its evolution over time, its interactions with the chaotic lattice, and its persistence lifecycle.

## 1. How the Neural Structure Works
The neural structure is a highly localized, high-dimensional parameter space directly overlaid onto the CA grid. Instead of a massive deep learning model with hidden layers, it acts as a **distributed sensory-memory network** where every single cell in the CA has its own dedicated neural weights.

- **The Vectors:** The `TitanMemory` struct maintains several arrays that mirror the size of the CA grid ($80 \times 40 = 3200$ parameters per array):
  - `w` (Weights) and `b` (Biases): The core knowledge storage.
  - `w_momentum` and `b_momentum`: The "velocity" of learning, granting the system an inertial memory (Information Momentum).
  - `alpha_field`: A localized learning rate that acts as the model's "attention" mechanism.
- **The Forward Pass:** For each step of the CA, the neural network looks at the probabilities (state) of the CA grid. It simply multiplies the CA state by its weights, adds the biases, and passes the result through a `tanh` activation function. This generates a **Modulation Field** (predictions ranging from -1.0 to 1.0).

## 2. How the Neural Model Evolves
The Titan Memory utilizes a paradigm called **Test-Time Training (Surprise-Based Learning)**. It is constantly learning while it operates, rather than having a distinct "training phase" and "inference phase."

1. **Self-Prediction:** The neural network attempts to predict what the CA grid will look like *before* the CA actually steps forward.
2. **Error Calculation:** Once the CA steps forward and generates the *true* new state, the neural network calculates the error (Prediction vs. Reality).
3. **Surprise-Based Modulation (`alpha_field`):** If the network is highly surprised (large error), it mathematically boosts its localized learning rate (`alpha_field` increases). If it accurately predicted the outcome, it lowers the learning rate (it becomes "bored" of that region).
4. **Momentum Update:** The gradients (errors) are fed into the Momentum vectors. This ensures the neural network doesn't overreact to random chaotic noise; it only shifts its core memory if a pattern is sustained over multiple steps.
5. **Thermodynamic Work:** The magnitude of this structural update is quantified as "Work"—the mathematical energy required to shift the neural weights to accommodate the new reality.

## 3. Interactions with the CA Model & Algorithms
The relationship between the Neural Network (Titan Memory) and the Cellular Automaton (Grid) is a **bilateral feedback loop**:

- **Grid to Neural:** The CA grid provides the raw state (the probabilities) that the neural network learns from. The grid is the "world" the brain is observing.
- **Neural to Grid (Modulation):** The neural network feeds its predictions (the Modulation Field) directly back into the CA's physics. When the CA updates the phase angle ($\theta$) of a cell, it multiplies the angle by the neural modulation. 
- **Recursive Renormalization (Downward Causation):** A separate algorithm groups the grid into macro-states and forces microscopic cells to align with their macro-regions.

### The Purpose of These Interactions
The purpose of this architecture is to bridge the gap between chaotic noise and static crystallization. 
- A pure CA often descends into maximum entropy (meaningless static).
- A pure neural network often overfits into static, repetitive loops.
- By binding them together, the CA provides an endless fountain of novel patterns (creativity), while the Neural Network acts as a stabilizing structural memory (logic). It forces the system to operate at the **Edge of Chaos**, the theoretical sweet spot for computation and Artificial General Intelligence.

## 4. Model Persistence (State Lifespan)
**Currently, the Titan Memory model is entirely ephemeral (transient).** 
- **No Saving to Disk:** The neural weights (`w`, `b`, momentum) are instantiated randomly in computer RAM every time a new `LatticeState` is created.
- **Program Lifecycle:** When the Rust program finishes executing (e.g., when a generation finishes in the `self-prime` loop, or when the user exits the CLI), the memory struct is dropped, and the neural model ceases to exist.
- **Why?** At this stage in the project, we are studying the real-time *thermodynamics* of the learning process (how the system adapts in the moment). 
- **Future Expansion:** To make the system "remember" across different days or different runs, we would need to implement `serde` serialization to save the `TitanMemory` arrays to a `.json` or `.bin` file on the hard drive, and load it upon startup.