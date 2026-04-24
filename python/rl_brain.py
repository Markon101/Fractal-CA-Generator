import requests
import json
import sys
import time
from engine_utils import ensure_engine_running

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

def run_scaffold_test(duration=0):
    print("### FRACTAL CA-TOKEN BRAIN TEST ###")
    
    if not ensure_engine_running():
        print("Error: Engine unavailable.")
        return

    init_data = {
        "width": 60,
        "height": 30,
        "seed": "Continuous evolution test at 60Hz semantic density.",
        "instruction_header": "## SYSTEM DIRECTIVE: CONTINUOUS FRACTAL ANALYSIS"
    }
    
    try:
        print(f"Initializing Engine (Duration: {duration}s)...")
        r_init = requests.post("http://localhost:3000/api/v1/init", json=init_data)
        if r_init.status_code == 200:
            print("Lattice initialized successfully.")
        
        start_time = time.time()
        while True:
            r_state = requests.get("http://localhost:3000/api/v1/formatted")
            if r_state.status_code == 200:
                # Clear screen for animation effect if running continuously
                if duration > 0:
                    print("\033[H\033[J", end="") 
                print(r_state.text)
            
            if duration == 0 or (time.time() - start_time) > duration:
                break
            
            time.sleep(0.5) # 2 FPS for visibility
            
        print("\nTest completed.")
        
    except Exception as e:
        print(f"Test failed: {e}")

if __name__ == "__main__":
    dur = 0
    if len(sys.argv) > 1:
        dur = int(sys.argv[1])
    run_scaffold_test(dur)
