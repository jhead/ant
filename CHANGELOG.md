# Changelog

All notable changes to the Ant Farm Simulation project will be documented in this file.

## [Unreleased]

### MVP Simplification

- Removed resources and pheromone systems to focus on core functionality
- Simplified ant movement to basic random wandering
- Added space bar to spawn new ants
- Fixed vsync configuration using PresentMode::Fifo
- Streamlined ant component structure
- Removed unused enums and components

### Added

- Initial project setup with Bevy engine
- Basic terrain generation system with different layers (Surface, Dirt, Stone, Deep, Bedrock)
- Terrain features including caves, water pockets, resource veins, and tunnels
- Camera controls for exploring the world
- Chunk-based terrain loading system
- UI legend showing terrain types and features

### Changed

- Adjusted camera zoom from 1.0 to 0.25 to show more of the world at once
- Increased chunk size from 16 to 32 for better visibility
- Increased chunk load distance from 3 to 5 to ensure better buffer around the edges
- Fixed integer overflow issues in chunk generation
- Improved terrain feature rendering with proper mesh generation
- Enhanced color scheme for better visual distinction between terrain types
- Refactored ant module into separate components:
  - `src/ant/mod.rs`: Main module file with plugin setup
  - `src/ant/components.rs`: Ant-related components and constants
  - `src/ant/systems.rs`: Systems for ant movement and interaction
  - `src/ant/behaviors.rs`: Core ant behavior logic
- Improved code organization and maintainability
- Enhanced ant movement and digging behavior
- Added better logging for ant state changes and actions

### Fixed

- Fixed type mismatches in chunk entity spawning
- Resolved issues with moved values in mesh generation
- Fixed camera positioning and zoom settings
- Addressed compilation errors related to mesh attributes

### Technical Debt

- Several unused imports and variables need to be cleaned up
- Some struct fields and enum variants are currently unused and will be implemented in future updates
- Physics plugins were removed as they're not currently being used

### Performance Improvements

- Added vsync and frame rate limiting (60 FPS) to reduce screen tearing and ensure consistent frame timing
- Optimized ant movement system to update at 30 FPS instead of every frame
- Reduced pheromone update frequency to 7.5 FPS with adjusted fade rates
- Improved camera movement smoothing with delta time capping and gentler acceleration/deceleration
- Added interpolation for smoother camera transitions
