@echo off
echo ================================================
echo   TransformerForge - Build Script
echo ================================================
echo.

echo [1/3] Checking Rust installation...
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Rust not found! Install from https://rustup.rs
    pause
    exit /b 1
)
echo Rust found!

echo.
echo [2/3] Building project (Release mode)...
cargo build --release
if %errorlevel% neq 0 (
    echo ERROR: Build failed!
    pause
    exit /b 1
)

echo.
echo [3/3] Running tests...
cargo test --release
if %errorlevel% neq 0 (
    echo WARNING: Some tests failed
)

echo.
echo ================================================
echo   Build completed successfully!
echo   Binary: target\release\transformer-forge.exe
echo   Run: run.bat
echo ================================================
pause
