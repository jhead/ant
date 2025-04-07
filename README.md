# Ant Farm Simulation

A 2D ant farm simulation game built with Bevy, where you can observe and interact with an ant colony as it explores and builds in a procedurally generated world.

## Features

- Procedurally generated terrain with different layers (Surface, Dirt, Stone, Deep, Bedrock)
- Terrain features including caves, water pockets, resource veins, and tunnels
- Camera controls for exploring the world
- Ant colony simulation (in development)
- Resource management system (in development)

## Controls

- **Arrow Keys**: Move the camera around the world
- **ESC**: Exit the game

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:

   ```
   git clone https://github.com/yourusername/ant-farm.git
   cd ant-farm
   ```

2. Build and run the game:
   ```
   cargo run
   ```

## Project Structure

- `src/ant.rs`: Ant behavior and properties
- `src/colony.rs`: Colony management and ant spawning
- `src/resources.rs`: Resource types and management
- `src/world/`: World generation and management
  - `camera.rs`: Camera controls and setup
  - `terrain.rs`: Terrain generation and chunk management
  - `ui.rs`: User interface elements
  - `resources.rs`: World resources and obstacles

## Development Status

This project is currently in early development. The terrain generation system is functional, but the ant colony simulation is still being implemented.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
