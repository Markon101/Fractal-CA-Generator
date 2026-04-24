@echo off
REM Fractal CA Generator - Windows Install Script

echo Checking prerequisites...

REM Check for Rust
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo Rust not found. Please install it from https://rustup.rs/
    pause
    exit /b 1
) else (
    echo Rust found.
)

REM Check for Python
where python >nul 2>nul
if %errorlevel% neq 0 (
    echo Python not found. Please install Python 3.10+ and ensure it is in your PATH.
    pause
    exit /b 1
) else (
    echo Python found.
)

echo Setting up Python virtual environment...
python -m venv venv
call venv\Scripts\activate

echo Installing Python dependencies...
python -m pip install --upgrade pip
pip install -r python/requirements.txt

echo Building Rust engine...
cd engine
cargo build --release
cd ..

echo ----------------------------------------
echo Installation complete!
echo To start, run:
echo call venv\Scripts\activate
echo cd engine
echo cargo run --release
echo ----------------------------------------
pause
