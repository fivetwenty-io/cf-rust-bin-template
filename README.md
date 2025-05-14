# Rust Binary Buildpack Test

This project demonstrates how to create a Rust web server application that deploys to Cloud Foundry using the binary buildpack.

## Project Structure

```
cf-rust-binary/
├── Cargo.toml          # Rust package definition
├── src/
│   └── main.rs         # Main Rust source code
├── manifest.yml        # Cloud Foundry manifest
├── build.sh            # Build script for Unix/Mac
└── build.bat           # Build script for Windows
```

## Prerequisites

1. Install Rust:
   ```
   # Linux/Mac
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Windows
   # Download from https://rustup.rs/ and run installer
   ```

2. Install Cloud Foundry CLI:
   ```
   # Follow instructions at https://docs.cloudfoundry.org/cf-cli/install-go-cli.html
   ```

## Building the Binary

### On Linux/Mac:

```bash
# Make the build script executable
chmod +x build.sh

# Run the build script
./build.sh
```

### On Windows:

```cmd
# Run the build script
build.bat
```

The build scripts will:
1. Compile the Rust application with optimizations
2. Create a deployment directory
3. Copy the binary and manifest to the deployment directory

## Deploying to Cloud Foundry

After building, deploy with:

```bash
cd target/deploy
cf login -a API_ENDPOINT -u USERNAME -p PASSWORD -o ORGANIZATION -s SPACE
cf push
```

## How It Works

1. **The Binary Buildpack**:
   - Simply runs your compiled binary
   - No runtime dependencies needed
   - Minimal overhead

2. **The Rust Application**:
   - Uses Actix Web for HTTP handling
   - Binds to `$PORT` environment variable (provided by CF)
   - Has health check endpoint at `/health`
   - Exposes environment info at `/env`

3. **Cloud Foundry Integration**:
   - The manifest specifies the binary buildpack
   - Sets the command to run the binary
   - Configures health check endpoint

## Testing Locally

To test the application locally before deploying:

```bash
# Build in development mode
cargo build

# Run the binary
PORT=8080 target/debug/cf-rust-binary
```

Then visit:
- http://localhost:8080/ - Main endpoint
- http://localhost:8080/health - Health check
- http://localhost:8080/env - Environment information

## Notes for Binary Buildpack

- The binary must be compiled for the correct target platform (Linux/x86_64 for most CF deployments)
- The binary must be statically linked or include all dependencies
- It must handle the `PORT` environment variable
- It must bind to `0.0.0.0` (all interfaces) not just localhost