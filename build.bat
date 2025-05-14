@echo off
REM Build script for creating the Rust binary on Windows

echo Building Rust binary for Cloud Foundry deployment...

REM Make sure Rust is installed
where rustc >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Error: Rust is not installed. Please install Rust first from https://rustup.rs/
    exit /b 1
)

REM Create release build
echo Creating release build...
cargo build --release

REM Prepare for deployment
echo Preparing for deployment...
if not exist target\deploy mkdir target\deploy
copy target\release\cf-rust-binary.exe target\deploy\cf-rust-binary.exe
copy manifest.yml target\deploy\

echo Done! Your binary is ready for deployment in the target\deploy directory.
echo To deploy to Cloud Foundry:
echo   cd target\deploy
echo   cf push