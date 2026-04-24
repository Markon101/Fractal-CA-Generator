import requests
import json
import sys
import time
import argparse
from engine_utils import ensure_engine_running

def generate_chaos_map(prompt, iterations=20, width=80, height=40):
    """Initializes the Rust CA engine and evolves it to get a mature chaotic map."""
    if not ensure_engine_running():
        print("Error: Could not connect to or start the CA Engine.")
        return None

    init_data = {
        "width": width,
        "height": height,
        "seed": prompt,
        "instruction_header": "## CHAOS AGENT DIRECTIVE"
    }
    
    try:
        requests.post("http://localhost:3000/api/v1/init", json=init_data)
        
        # Evolve deeper into chaos in one go
        r_state = requests.post("http://localhost:3000/api/v1/formatted", json={"steps": iterations})
            
        return r_state.text
    except Exception as e:
        print(f"Engine connection failed: {e}")
        return None

def extract_focal_points(map_text, max_points=3):
    """Finds the densest coordinates (X and @) to act as 'seeds' for new thoughts."""
    lines = map_text.split('\n')
    grid = [list(line) for line in lines if len(line) > 10 and not line.startswith('#') and not line.startswith('PROMPT') and not line.startswith('CA ITER')]
    
    focal_points = []
    for y, row in enumerate(grid):
        for x, char in enumerate(row):
            if char in ['@', 'X', '*']:
                # Calculate local density
                density = 0
                for dy in [-1, 0, 1]:
                    for dx in [-1, 0, 1]:
                        if 0 <= y+dy < len(grid) and 0 <= x+dx < len(grid[y+dy]):
                            if grid[y+dy][x+dx] in ['@', 'X', 'o', '*']:
                                density += 1
                if density > 0: # Add any non-zero density point to be sorted later
                    focal_points.append({"x": x, "y": y, "density": density, "char": char})
    
    # Sort by density and take top distinct clusters
    focal_points.sort(key=lambda p: p["density"], reverse=True)
    distinct_points = []
    for p in focal_points:
        if not any(abs(p['x'] - dp['x']) < 5 and abs(p['y'] - dp['y']) < 5 for dp in distinct_points):
            distinct_points.append(p)
            if len(distinct_points) == max_points: break
            
    return distinct_points

def agent_reasoning(prompt, focal_points):
    print(f"\n[AGENT THOUGHT PROCESS: {prompt}]")
    print("The CA has evolved. Scanning fractal landscape for semantic clusters...\n")
    
    if not focal_points:
        print(">> No stable clusters formed. The idea dissolved into entropy. Try a different approach.")
        return

    strategies = [
        "Architectural Core (Deep Structure)",
        "Edge-case Anomaly (Unexpected Behavior)",
        "Emergent Bridge (Connecting distant concepts)",
        "Fractal Resonance (Pattern duplication)",
        "Entropic Drift (Natural decay analysis)"
    ]

    for i, pt in enumerate(focal_points):
        strategy = strategies[i % len(strategies)]
        print(f"Cluster {i+1} found at [X:{pt['x']}, Y:{pt['y']}] (Density: {pt['density']}) -> {strategy}")
        
        # Procedurally generate a task based on the coordinates
        if pt['y'] < 10:
            focus = "Front-end/User Interface"
        elif pt['y'] < 25:
            focus = "Business Logic/Middleware"
        else:
            focus = "Database/Infrastructure"
            
        if pt['x'] < 30:
            action = "optimize for speed and low latency."
        elif pt['x'] < 60:
            action = "ensure high reliability and fault tolerance."
        else:
            action = "design for infinite scalability and modularity."
            
        print(f"   Actionable Insight: Focus on '{focus}' and {action}")
        print(f"   Chaos Prompt: 'How does coordinate ({pt['x']}, {pt['y']}) change if the system state suddenly collapses?'\n")

def run_agent():
    parser = argparse.ArgumentParser(description="Chaos Agent Fractal Analyzer")
    parser.add_argument("prompt", nargs="?", default="Build a distributed AI orchestration platform", help="The seed prompt")
    parser.add_argument("--iterations", type=int, default=15, help="Number of CA steps")
    parser.add_argument("--width", type=int, default=80, help="Grid width")
    parser.add_argument("--height", type=int, default=40, help="Grid height")
    parser.add_argument("--points", type=int, default=5, help="Number of focal points to extract")
    
    args = parser.parse_args()
        
    map_text = generate_chaos_map(args.prompt, iterations=args.iterations, width=args.width, height=args.height)
    if map_text:
        focal_points = extract_focal_points(map_text, max_points=args.points)
        agent_reasoning(args.prompt, focal_points)

if __name__ == "__main__":
    run_agent()
