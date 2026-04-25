@echo off
echo Installing Fractal CA Generator (Pure Rust)...

where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo Error: Rust/Cargo not found. Please install Rust first.
    exit /b 1
)

cargo build --release

echo Installation complete. Binary located at target\release\fractal-ca.exe
echo Try running: target\release\fractal-ca.exe agent "Hello Chaos"
