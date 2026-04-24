import requests
import numpy as np
import sys
import time

class TitanPredictor:
    def __init__(self, latents_path):
        data = np.load(latents_path)
        self.W = data['W']
        self.b = data['b']
        
    def predict(self, x):
        return np.tanh(x * self.W + self.b)

def run_observation(width, h, steps, seed):
    print(f"### TITAN OBSERVATION RUN: {width}x{h} ###")
    
    predictor = None
    try:
        predictor = TitanPredictor("titan_latents.npz")
        print("Trained Titan latents loaded.")
    except Exception as e:
        print(f"Warning: Could not load latents ({e}). Running with identity modulation.")

    # Initialize Engine
    requests.post("http://localhost:3000/api/v1/init", json={
        "width": width, "height": h, "seed": seed
    })
    
    current_probs = np.ones(width * h)
    
    for i in range(steps):
        # 1. Modulation from Titan Memory
        if predictor:
            modulation = predictor.predict(current_probs)
        else:
            modulation = np.ones(width * h)
            
        # 2. Step the Hilbert Space CA
        r = requests.post("http://localhost:3000/api/v1/step", json={
            "field": modulation.tolist()
        })
        current_probs = np.array(r.json())
        
        # 3. Fetch ASCII visualization every 5 steps
        if i % 5 == 0:
            print("\033[H\033[J", end="") # Clear screen
            r_viz = requests.get("http://localhost:3000/api/v1/formatted")
            print(f"PROMPT: {seed} | STEP: {i}")
            print(r_viz.text)
            time.sleep(0.5)

if __name__ == "__main__":
    w, h, s = 60, 30, 50
    seed = "Quantum Observation"
    if len(sys.argv) > 1: seed = sys.argv[1]
    if len(sys.argv) > 2: s = int(sys.argv[2])
    run_observation(w, h, s, seed)
