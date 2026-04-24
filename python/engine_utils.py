import socket
import time
import subprocess
import os

def is_engine_running():
    """Checks if the Rust engine is listening on port 3000."""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        return s.connect_ex(('localhost', 3000)) == 0

def ensure_engine_running():
    """Checks for the engine and attempts to start it if missing."""
    if is_engine_running():
        return True
    
    print("Engine not detected. Attempting to start Titan-Hilbert Engine...")
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.dirname(script_dir)
    engine_dir = os.path.join(project_root, "engine")
    
    try:
        # Start engine in background using 'cargo run --release'
        subprocess.Popen(
            ["cargo", "run", "--release"],
            cwd=engine_dir,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL
        )
        # Wait up to 15 seconds for the engine to boot
        for _ in range(15):
            if is_engine_running():
                print("Engine started successfully.")
                return True
            time.sleep(1)
    except Exception as e:
        print(f"Failed to start engine: {e}")
    
    return False
