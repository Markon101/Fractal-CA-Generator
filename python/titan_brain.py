import requests
import numpy as np
import sys
import time
from engine_utils import ensure_engine_running

class TitanMemory:
    """A simplified Neural Memory (Titans architecture) that learns in real-time."""
    def __init__(self, size, lr=0.01):
        self.size = size
        self.lr = lr
        # The 'Memory' weights - our latent storage
        self.W = np.random.randn(size) * 0.1
        self.b = np.random.randn(size) * 0.1
        
    def forward(self, x):
        # Neural modulation based on current memory
        return np.tanh(x * self.W + self.b)
    
    def update(self, x, target):
        # Online Learning: Update weights to better predict the CA state
        # In a real Titan, this would be a gradient update to the memory network
        pred = self.forward(x)
        error = pred - target
        
        # Simple SGD update to the memory weights
        self.W -= self.lr * error * x
        self.b -= self.lr * error
        return np.mean(np.abs(error))

def run_titan_loop(width, height, steps, seed):
    print(f"### TITAN NEURAL MEMORY RUN: {width}x{height} | {steps} Steps ###")
    
    if not ensure_engine_running():
        print("Error: Engine unavailable.")
        return

    size = width * height
    titan = TitanMemory(size)
    
    # Initialize Engine
    requests.post("http://localhost:3000/api/v1/init", json={
        "width": width, "height": height, "seed": seed
    })
    
    current_field = np.ones(size)
    
    start_time = time.time()
    for i in range(steps):
        # 1. Modulation from Titan Memory
        modulation = titan.forward(current_field)
        
        # 2. Step the Hilbert Space CA
        r = requests.post("http://localhost:3000/api/v1/step", json={
            "field": modulation.tolist()
        })
        next_probs = np.array(r.json())
        
        # 3. Online Learning: Titan learns the new state
        loss = titan.update(current_field, next_probs)
        current_field = next_probs
        
        if i % 500 == 0 or i == steps - 1:
            print(f"Step {i:5} | Loss: {loss:.8f} | Prob: {np.sum(current_field):.2f}")
            brain_slice = "".join(['#' if v > 0.3 else '.' for v in modulation[:60]])
            print(f"Brain Core [{i}]: {brain_slice}")

    print(f"\nTraining Complete. Saving Titan Latents...")
    np.savez("titan_latents.npz", W=titan.W, b=titan.b)
    print("Latents saved to 'titan_latents.npz'.")
    print(f"Total Time: {time.time() - start_time:.2f}s")

if __name__ == "__main__":
    w = 60
    h = 30
    s = 100
    seed = "Titan-Alpha"
    
    if len(sys.argv) > 1: seed = sys.argv[1]
    if len(sys.argv) > 2: s = int(sys.argv[2])
    if len(sys.argv) > 3: w = int(sys.argv[3])
    if len(sys.argv) > 4: h = int(sys.argv[4])
    
    run_titan_loop(w, h, s, seed)
