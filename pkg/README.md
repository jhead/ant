# Ant Farm Simulation

A procedurally generated ant farm simulation game built with Bevy and Rust. Watch as worker ants dig and explore through a dynamically generated terrain, creating tunnels and expanding their colony.

## Features

- Procedurally generated terrain with different layers (Surface, Dirt, Stone, Deep, Bedrock)
- Terrain features including caves, water pockets, resource veins, and tunnels
- Ant colony simulation with worker ants that dig and explore
- Camera controls for exploring the world
- Chunk-based terrain loading system
- WebAssembly support for playing in the browser
- Physics simulation using Bevy Rapier2D
- Pathfinding system for ant navigation
- Dynamic terrain modification
- Central colony cavity with surrounding tunnels
- Hot reloading for WASM development

## Game Mechanics

### Terrain System

- Tile-based world with 8x8 pixel tiles
- Multiple tile types (Dirt, Air) with different properties
- Dynamic terrain modification through digging
- Central cavity generation for colony starting point
- Efficient tile storage and retrieval system

### Ant Behavior

- Worker ants with autonomous digging behavior
- State-based ant AI (Searching, Moving, Digging)
- Pathfinding system for efficient navigation
- Configurable ant parameters:
  - Maximum colony distance: 500 units
  - Worker work radius: 400 units
  - Dig chance: 80%
  - Branch chance: 5%
  - Search radius: 100 units
  - Ant speed: 100 units/second

### Colony Management

- Central colony hub
- Worker ant spawning and management
- Resource gathering and distribution
- Colony expansion through tunnel networks

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable version)
- wasm-pack (for WebAssembly builds)
- Python 3 (for local development server)
- Python packages (install with `pip install -r requirements.txt`)

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

# Serve with auto-rebuild on file changes
make watch

# Check if WASM build is valid
make check

# Test the WASM build with a simple server
make test

# Clean build artifacts (except Bevy dependencies)
make clean
```

### Development Workflow

1. Install Python dependencies:

   ```bash
   pip install -r requirements.txt
   ```

2. Start the development server with hot reloading:

   ```bash
   make watch
   ```

3. The browser will automatically open to http://localhost:8000
   - Any changes to Rust source files will trigger automatic rebuild
   - The page will automatically reload when the build is complete
   - Press Ctrl+C to stop the server

## Controls

- **WASD**: Move camera
- **Space**: Spawn new worker ants
- **Mouse Click**: Command ants to move to a location

## Project Structure

- `src/`: Source code
  - `lib.rs`: Library entry point and plugin setup
  - `main.rs`: Binary entry point
  - `ant/`: Ant-related code
    - `mod.rs`: Main module file with plugin setup
    - `components.rs`: Ant-related components and constants
    - `systems.rs`: Systems for ant movement and interaction
    - `behaviors.rs`: Core ant behavior logic
    - `app.rs`: Application setup for both WASM and native builds
    - `pathfinding.rs`: Pathfinding implementation for ant navigation
  - `colony.rs`: Colony management and simulation
  - `terrain/`: Terrain generation and management
    - `mod.rs`: Terrain system implementation
- `public/`: Web deployment files
  - `index.html`: Main HTML file for WASM deployment
  - `pkg/`: WASM build output
- `assets/`: Game assets and resources
- `watch.py`: Development server with hot reloading
- `serve.py`: Basic development server
- `requirements.txt`: Python dependencies

## Development Notes

- The project uses optimized build settings for better performance
- Debug builds include some optimization for faster development
- Release builds use full optimization with LTO enabled
- Bevy dependencies are preserved during cleaning to avoid long rebuilds
- Physics simulation is configured for top-down 2D movement with no gravity
- Tile system uses a custom hash implementation for efficient storage
- Ant pathfinding system supports dynamic obstacle avoidance
- Hot reloading is available for WASM development with `make watch`

## License

This project is licensed under the MIT License - see the LICENSE file for details.
