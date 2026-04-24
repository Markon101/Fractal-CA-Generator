import requests
import json
import sys
import time

def run_deep_time():
    prompt = "Europa Orbital Research Station: Thousand Year Legacy"
    print(f"### DEEP TIME EVOLUTION: {prompt} ###\n")
    
    # Init
    requests.post("http://localhost:3000/api/v1/init", json={
        "width": 80,
        "height": 40,
        "seed": prompt
    })

    epochs = [1, 100, 500, 1000, 5000]
    current_iteration = 1
    
    for target in epochs:
        steps_needed = target - current_iteration
        if steps_needed > 0:
            print(f"Evolving to iteration {target}...")
            r = requests.post("http://localhost:3000/api/v1/formatted", json={"steps": steps_needed})
            current_iteration = target
        else:
            r = requests.get("http://localhost:3000/api/v1/formatted")
            
        map_text = r.text
        
        # Simple entropy check
        density = sum(1 for c in map_text if c in ['@', 'X', 'o', '*', '.'])
        print(f"\n--- EPOCH {target} (Density: {density}) ---")
        
        # Extract a "Temporal Insight"
        if density > 1500:
            print("Status: Over-growth / Hyper-complexity. The station has become a biological-mechanical hybrid city.")
        elif density > 800:
            print("Status: Mature equilibrium. The original blueprint is barely visible beneath centuries of adaptations.")
        elif density > 100:
            print("Status: Entropic decay. The station is a skeleton, preserved by vacuum but losing its semantic core.")
        else:
            print("Status: Ghost. Only radiation and dust remain.")

        # Show a snippet of the 'heart' of the station
        lines = map_text.split('\n')
        if len(lines) > 25:
            print("\nCenter Core Snapshot:")
            for l in lines[15:25]:
                print(l[30:50])
        
        print("-" * 40)
        time.sleep(1)

if __name__ == "__main__":
    run_deep_time()
