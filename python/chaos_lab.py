import requests
import sys
import time

def get_grid(prompt):
    requests.post("http://localhost:3000/api/v1/init", json={
        "width": 40,
        "height": 20,
        "seed": prompt
    })
    # Get iteration 1
    r = requests.get("http://localhost:3000/api/v1/formatted")
    return r.text

def run_lab():
    print("### CHAOS LABORATORY: PHASE TRANSITION ANALYSIS ###")
    
    # 1. Determinism Test
    print("\n[Test 1: Determinism]")
    p1 = "Alpha" # len 5
    p2 = "Omega" # len 5
    grid1 = get_grid(p1)
    grid2 = get_grid(p2)
    
    if grid1 == grid2:
        print("RESULT: Confirmed. Seeding is length-deterministic (Length 5).")
    else:
        print("RESULT: Failed. Grids differ for same-length prompts.")

    # 2. Entropy Sweep (Lengths 1 to 20)
    print("\n[Test 2: Entropy Sweep]")
    results = []
    for length in range(1, 21):
        prompt = "A" * length
        requests.post("http://localhost:3000/api/v1/init", json={
            "width": 40,
            "height": 20,
            "seed": prompt
        })
        
        # Evolve 10 steps
        r = requests.post("http://localhost:3000/api/v1/formatted", json={"steps": 10})
        
        map_text = r.text
        # Count non-space characters in the grid
        density = sum(1 for c in map_text if c in ['@', 'X', 'o', '*', '.'])
        results.append((length, density))
        print(f"Length {length:2} | Density: {density:4} {'#' * (density // 10)}")

    # 3. Analyze Findings
    results.sort(key=lambda x: x[1])
    print(f"\n[Analysis]")
    print(f">> Minimal Entropy Seed (Extinction Path): Length {results[0][0]}")
    print(f">> Maximal Entropy Seed (Growth Path): Length {results[-1][0]}")

if __name__ == "__main__":
    run_lab()
