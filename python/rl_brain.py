import torch
import torch.nn as nn
import torch.nn.functional as F
import numpy as np
import requests

class TitanMemory(nn.Module):
    """Neural Long-Term Memory for global context persistence."""
    def __init__(self, dim=128):
        super().__init__()
        self.memory_bank = nn.Parameter(torch.randn(1, 1024, dim))
        self.query_proj = nn.Linear(dim, dim)
        
    def forward(self, x):
        # Simplistic read operation for the architectural scaffold
        q = self.query_proj(x)
        attn = torch.matmul(q, self.memory_bank.transpose(-2, -1))
        context = torch.matmul(F.softmax(attn, dim=-1), self.memory_bank)
        return context

class FractalCARules(nn.Module):
    """Neural CA ruleset that determines token transitions."""
    def __init__(self, n_tokens=50257, hidden_dim=64):
        super().__init__()
        self.embedding = nn.Embedding(n_tokens, hidden_dim)
        self.conv = nn.Conv2d(hidden_dim, hidden_dim, kernel_size=3, padding=1)
        self.memory = TitanMemory(dim=hidden_dim)
        self.to_logits = nn.Linear(hidden_dim, n_tokens)

    def forward(self, grid_ids):
        # grid_ids: [B, H, W]
        x = self.embedding(grid_ids) # [B, H, W, D]
        x = x.permute(0, 3, 1, 2) # [B, D, H, W]
        
        # Local neighbor interaction via Convolution
        x = F.relu(self.conv(x))
        
        # Global context via Titan Memory (flattened query)
        b, d, h, w = x.shape
        x_flat = x.permute(0, 2, 3, 1).reshape(b, h*w, d)
        context = self.memory(x_flat)
        x_flat = x_flat + context
        
        # Project back to logits
        logits = self.to_logits(x_flat)
        return logits.reshape(b, h, w, -1)

def run_rl_loop():
    print("Initializing Fractal RL Brain with Titans Memory...")
    model = FractalCARules(n_tokens=1000) # Using small vocab for scaffold
    
    # Example interaction with Rust Engine
    try:
        r = requests.get("http://localhost:3000/api/v1/lattice/state")
        if r.status_code == 200:
            print("Successfully connected to Rust Engine.")
            state = r.json()
            grid = torch.tensor(state['grid']).unsqueeze(0)
            logits = model(grid)
            print(f"Computed next step probabilities for lattice of size {state['width']}x{state['height']}")
    except Exception as e:
        print(f"Could not connect to Rust Engine (ensure it is running): {e}")

if __name__ == "__main__":
    run_rl_loop()
