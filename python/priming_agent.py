import requests
import json
import argparse
import math
from engine_utils import ensure_engine_running

def get_ca_metrics(prompt, iterations=30, width=100, height=50):
    """Evolves the CA and extracts high-level metrics for LLM priming."""
    if not ensure_engine_running():
        return None

    # Initialize with the instruction as seed
    init_data = {
        "width": width,
        "height": height,
        "seed": prompt,
        "instruction_header": "## EXPERIMENTAL PRIMING"
    }
    
    try:
        requests.post("http://localhost:3000/api/v1/init", json=init_data)
        
        # Evolve and get probabilities
        r = requests.post("http://localhost:3000/api/v1/step", json={"count": iterations})
        probs = r.json()
        
        # Calculate Entropy
        total_p = sum(probs)
        if total_p == 0: return None
        
        entropy = 0
        for p in probs:
            if p > 0:
                normalized_p = p / total_p
                entropy -= normalized_p * math.log2(normalized_p)
                
        # Calculate Density (active cells)
        max_p = max(probs)
        threshold = max_p * 0.1
        active_cells = sum(1 for p in probs if p > threshold)
        density = active_cells / len(probs)
        
        # Calculate Resonance (variance in probabilities as a proxy for patterns)
        avg_p = total_p / len(probs)
        variance = sum((p - avg_p)**2 for p in probs) / len(probs)
        resonance = math.sqrt(variance) / (avg_p + 1e-9)

        return {
            "entropy": round(entropy, 4),
            "density": round(density, 4),
            "resonance": round(resonance, 4),
            "raw_sum": round(total_p, 4)
        }
    except Exception as e:
        print(f"Error: {e}")
        return None

def extract_semantic_hotspots(instruction, metrics, width=100, height=50):
    """Maps CA activity back to the original instruction indices."""
    # This is a heuristic mapping based on how the Rust engine seeds the grid
    words = instruction.split()
    if not words: return []
    
    # In the engine, bytes are injected at (i * 13 + attempt * 7) % (width * height)
    # where i is the byte index.
    hotspots = []
    
    # For now, we'll simulate this by choosing words that correspond to the 'density'
    # In a real implementation, we'd fetch the actual grid and correlate.
    # Let's mock it for the experimental branch to show the intent.
    active_count = max(1, int(len(words) * metrics['density'] * 5))
    import random
    random.seed(metrics['raw_sum'])
    hotspots = random.sample(words, min(len(words), active_count))
    
    return hotspots

def generate_primed_instruction(instruction, metrics):
    """Wraps the original instruction in a CA-derived context."""
    
    hotspots = extract_semantic_hotspots(instruction, metrics)
    
    # Map metrics to qualitative descriptors
    if metrics['entropy'] > 10.95:
        vibe = "Highly divergent and creative."
    else:
        vibe = "Convergent and analytical."
        
    if metrics['resonance'] > 1.45:
        structure = "Strong rhythmic patterns detected. Use structured, modular responses."
    else:
        structure = "Diffuse state. Prefer fluid, narrative explanations."

    primed_prompt = f"""### CHAOS-PRIMED EXECUTION CONTEXT
[METRICS]
- Entropy: {metrics['entropy']} ({vibe})
- Density: {metrics['density']}
- Resonance: {metrics['resonance']} ({structure})

[SEMANTIC HOTSPOTS]
The chaotic evolution highlighted these core concepts: {', '.join(hotspots)}

[INSTRUCTION]
{instruction}

[CHAOS DIRECTIVE]
Analyze the provided metrics and hotspots. Adjust your temperature and reasoning style to match the 'vibe' and 'structure' described above. Prioritize the concepts identified in the [SEMANTIC HOTSPOTS] section.
"""
    return primed_prompt

def main():
    parser = argparse.ArgumentParser(description="Priming Agent for LLM Contextualization")
    parser.add_argument("instruction", help="The instruction to prime")
    parser.add_argument("--iterations", type=int, default=10)
    
    args = parser.parse_args()
    
    metrics = get_ca_metrics(args.instruction, iterations=args.iterations)
    if metrics:
        primed = generate_primed_instruction(args.instruction, metrics)
        print(primed)
    else:
        print("Failed to generate metrics.")

if __name__ == "__main__":
    main()
