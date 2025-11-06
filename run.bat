@echo off
echo ================================================
echo   TransformerForge - Starting Server
echo ================================================
echo.

if not exist target\release\transformer-forge.exe (
    echo ERROR: Binary not found!
    echo Please run build.bat first
    pause
    exit /b 1
)

echo Starting TransformerForge AI Inference Engine...
echo.
target\release\transformer-forge.exe
