# Ant Farm Simulation

A procedurally generated ant farm simulation game built with Bevy and Rust.

## Features

- Procedurally generated terrain with different layers (Surface, Dirt, Stone, Deep, Bedrock)
- Terrain features including caves, water pockets, resource veins, and tunnels
- Ant colony simulation with worker ants that dig and explore
- Camera controls for exploring the world
- Chunk-based terrain loading system
- WebAssembly support for playing in the browser

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version)
- wasm-pack (for WebAssembly builds)
- Python 3 (for local development server)

### Building and Running

The project uses a Makefile for building and running. Here are the available commands:

```bash
# Show help
make help

# Build for WebAssembly
make wasm

# Build for native platform
make native

# Build for both WASM and native
make all

# Run the native build
make run

# Serve the WASM build locally
make serve

# Check if WASM build is valid
make check

# Test the WASM build with a simple server
make test

# Clean build artifacts (except Bevy dependencies)
make clean
```

### Development Workflow

1. Build the project:

   ```bash
   make all
   ```

2. Run the native version:

   ```bash
   make run
   ```

3. Serve the WASM version locally:

   ```bash
   make serve
   ```

4. Open your browser and navigate to http://localhost:8000

## Controls

- **WASD**: Move camera
- **Space**: Spawn new worker ants
- **Mouse Click**: Command ants to move to a location

## Project Structure

- `src/`: Source code
  - `ant/`: Ant-related code
    - `mod.rs`: Main module file with plugin setup
    - `components.rs`: Ant-related components and constants
    - `systems.rs`: Systems for ant movement and interaction
    - `behaviors.rs`: Core ant behavior logic
    - `app.rs`: Application setup for both WASM and native builds
  - `colony/`: Colony-related code
  - `terrain/`: Terrain-related code
- `public/`: Web deployment files
  - `index.html`: Main HTML file for WASM deployment
  - `404.html`: 404 page for GitHub Pages
  - `pkg/`: WASM build output

## License

This project is licensed under the MIT License - see the LICENSE file for details.
