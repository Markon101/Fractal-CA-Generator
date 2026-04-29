# Fractal CA Generator: LLM Priming Experiments

## Objective
To test the efficacy of injecting LLM instruction prompts into a Rust-based Cellular Automaton (CA) via "Fractal Folding", evolving the state, and extracting metrics (Entropy, Density, Resonance) to create a "Chaotic Context" that primes an LLM's response style and focus.

## Experiment Setup
- **Branch:** `experiment/llm-priming`
- **Grid Size:** 100x50
- **Iterations:** 30
- **Seed Method:** Fractal Folding (Sierpinski-like pattern injection)
- **Script:** `python/priming_agent.py`

## Test Cases & Results

### Test 1: Abstract/Philosophical
**Prompt:** "Define the nature of consciousness."
- **Entropy:** 10.989 (Highly divergent and creative)
- **Density:** 0.3398
- **Resonance:** 1.4121 (Diffuse state. Prefer fluid, narrative explanations)

### Test 2: Technical/Structured
**Prompt:** "Implement a robust error handling middleware in Express.js."
- **Entropy:** 10.8906 (Highly divergent and creative)
- **Density:** 0.2886
- **Resonance:** 1.569 (Diffuse state. Prefer fluid, narrative explanations)

### Test 3: Creative/Narrative
**Prompt:** "Write a story about a clockmaker who discovers a gear that turns backwards in time."
- **Entropy:** 11.0357 (Highly divergent and creative)
- **Density:** 0.332
- **Resonance:** 1.3419 (Diffuse state. Prefer fluid, narrative explanations)

### Test 4: Complex/Multi-domain
**Prompt:** "Design a decentralized voting system using zero-knowledge proofs and smart contracts on Ethereum, ensuring anonymity and verifiable tallies."
- **Entropy:** 10.9814 (Highly divergent and creative)
- **Density:** 0.361
- **Resonance:** 1.4157 (Diffuse state. Prefer fluid, narrative explanations)

## Analysis & Theoretical Reflections

1. **Rapid Thermalization (Semantic Diffusion):**
   Across all four disparate test cases, the Entropy remained consistently high (~10.8 - 11.0) and Resonance remained relatively low (~1.3 - 1.5) after 30 iterations. The maximum theoretical entropy for a uniform distribution over 5000 cells is ~12.28. A value around 11 indicates the CA rapidly "thermalizes", diffusing the initial prompt's distinct phase footprint into a highly distributed probability field. 

2. **Differentiating Concept Topologies:**
   Because the system reaches high entropy quickly, the current 30-iteration duration may be over-saturating the differences between seeds. To extract more varied "vibes" and "structures" from different prompts, we should theoretically observe the CA at earlier intervals (e.g., iterations 5, 10, or 15) before the "Semantic Diffusion" described in the roadmap washes out the initial fractal geometry.

3. **Resonance Thresholding:**
   The `priming_agent.py` script uses a resonance threshold of `> 2.0` to detect "Strong rhythmic patterns". In these tests, resonance consistently fell between 1.3 and 1.6. The thresholds for determining the LLM's requested output structure need to be recalibrated based on the empirical dynamic range of the CA.

4. **Semantic Hotspots:**
   The identification of Semantic Hotspots provides a novel way to instruct an LLM. By highlighting certain words based on grid density clusters, the CA can act as an attention-focusing lens. While currently simulated via a random selection seeded by the grid sum, a true coordinate-to-word mapping (referencing the injection indices) could reveal fascinating structural intersections where different parts of a complex prompt "collide" within the Hilbert space.

## Next Steps
- Implement "Time-Lapse Profiling": capture metrics at 5, 10, 20, and 30 steps to find the ideal "divergence window".
- Recalibrate the qualitative thresholding logic in `priming_agent.py`.
- Develop a deterministic reverse-mapping for Semantic Hotspots from grid clusters back to the prompt byte indices.

---

## Round 2: Tuning the Priming Agent
Based on the previous analysis, `python/priming_agent.py` was tuned:
- **Iterations reduced:** 30 -> 10
- **Entropy Threshold:** > 10.95
- **Resonance Threshold:** > 1.45

### Round 2 Test Cases & Results

### Test 1: Abstract/Philosophical
**Prompt:** "Define the nature of consciousness."
- **Entropy:** 10.1632 (Convergent and analytical)
- **Density:** 0.0886
- **Resonance:** 2.4881 (Strong rhythmic patterns detected. Use structured, modular responses)

### Test 2: Technical/Structured
**Prompt:** "Implement a robust error handling middleware in Express.js."
- **Entropy:** 10.3403 (Convergent and analytical)
- **Density:** 0.0866
- **Resonance:** 2.3367 (Strong rhythmic patterns detected. Use structured, modular responses)

### Test 3: Creative/Narrative
**Prompt:** "Write a story about a clockmaker who discovers a gear that turns backwards in time."
- **Entropy:** 10.5806 (Convergent and analytical)
- **Density:** 0.1246
- **Resonance:** 1.9870 (Strong rhythmic patterns detected. Use structured, modular responses)

### Test 4: Complex/Multi-domain
**Prompt:** "Design a decentralized voting system using zero-knowledge proofs and smart contracts on Ethereum, ensuring anonymity and verifiable tallies."
- **Entropy:** 10.7325 (Convergent and analytical)
- **Density:** 0.1818
- **Resonance:** 1.7673 (Strong rhythmic patterns detected. Use structured, modular responses)

## Round 3: IIT, Momentum & Thermodynamic Work (April 2026)
With the migration to a pure Rust implementation and the integration of Integrated Information Theory (IIT) and Information Momentum, the priming process has been fundamentally transformed.

### Objectives
- Validate the Integrated Information Potential ($\Phi$) as a measure of structural integration.
- Observe the effects of Information Momentum on the trajectory of the CA evolution.
- Test the new "Cognitive Priming Field" output for LLM instruction.

### Test Case: "Integrated Information and Information Momentum"
- **Iterations:** 10
- **Entropy:** 10.0087 (Convergent and analytical)
- **Resonance:** 2.4800 (Strong rhythmic patterns detected. Use structured, modular responses)
- **Integrated Information ($\Phi$):** 24.8218
- **$\Phi$ Trajectory:** Descending (Differentiating)
- **Thermodynamic Work:** 0.0456 (Energy of transition)

### Round 3 Analysis & Theoretical Reflections

1. **Integration as Identity ($\Phi$):**
   The new $\Phi$ metric successfully captures the relationship between differentiation (Entropy) and integration (Resonance). A high $\Phi$ value (~24.8) at 10 iterations indicates a highly structured, integrated state. The "Descending" trajectory suggests the system was moving from a highly ordered seed toward a more differentiated, complex state.

2. **Inertial Identity (Momentum):**
   The introduction of `w_momentum` and `b_momentum` in the Titan memory provides the system with a "temporal self". The updates are no longer purely local and instantaneous; they carry the weight of the previous trajectory, stabilizing the evolution against random fluctuations while accelerating consistent semantic shifts.

3. **Thermodynamic Equivalence:**
   The "Thermodynamic Work" metric provides a quantitative measure of the energy cost of the state transition. In this test, 0.0456 units of work were done per step to integrate the new information into the grid's attractor.

4. **Cognitive Priming:**
   The structured "Cognitive Priming Field" output now provides a high-signal directive for LLMs, including systemic momentum and integration trends. This aligns with the "Information in Motion" philosophy, treating the LLM priming not as a static prompt, but as a field of force within which the reasoning occurs.
