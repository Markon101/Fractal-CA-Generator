import requests
import json
import sys

# Attempt to load torch, but allow fallback for scaffolding tests
try:
    import torch
    import torch.nn as nn
    import torch.nn.functional as F
    TORCH_AVAILABLE = True
except ImportError:
    TORCH_AVAILABLE = False
    print("Warning: torch not found. Running in mock-mode for engine connectivity test.")

class TitanMemoryMock:
    def __init__(self, dim=128): pass
    def forward(self, x): return x

def run_scaffold_test():
    print("### FRACTAL CA-TOKEN BRAIN TEST ###")
    
    # 1. Initialize the Engine with a custom prompt
    init_data = {
        "width": 40,
        "height": 20,
        "seed_prompt": "Exploring the infinite ladder of coherent complexity.",
        "instruction_header": "## SYSTEM DIRECTIVE: FRACTAL MAP ANALYSIS\nIntegrate the following chaotic density map into your reasoning process."
    }
    
    try:
        print("Initializing Engine with prompt...")
        r_init = requests.post("http://localhost:3000/api/v1/lattice/init", json=init_data)
        if r_init.status_code == 200:
            print("Lattice initialized successfully.")
        
        # 2. Fetch the Formatted Output
        print("Fetching formatted chaotic map...")
        r_state = requests.get("http://localhost:3000/api/v1/lattice/formatted")
        if r_state.status_code == 200:
            print("\n--- OUTPUT START ---")
            print(r_state.text)
            print("--- OUTPUT END ---\n")
            
        # 3. Analyze Impact
        print("Analysis: This chaotic map provides a 'non-linear priming' signal.")
        print("By injecting high-entropy but structured data (CA patterns), we break")
        print("the standard autoregressive bias, forcing the LLM to search for")
        print("latent patterns in the 'noise', which can trigger higher-order reasoning.")
        
    except Exception as e:
        print(f"Test failed: {e}")
        print("Ensure the Rust engine is running (make run-engine).")

if __name__ == "__main__":
    run_scaffold_test()
