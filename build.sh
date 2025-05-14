#!/bin/bash
# Build script for creating the Rust binary

set -e

echo "Building Rust binary for Cloud Foundry deployment..."

# Make sure Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Error: Rust is not installed. Please install Rust first:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Create release build
echo "Creating release build..."
cargo build --release

# Prepare for deployment
echo "Preparing for deployment..."
mkdir -p target/deploy
cp target/release/cf-rust-binary target/deploy/
cp manifest.yml target/deploy/

echo "Done! Your binary is ready for deployment in the target/deploy directory."
echo "To deploy to Cloud Foundry:"
echo "  cd target/deploy"
echo "  cf push"