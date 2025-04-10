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
- Colony distance constraints for ant movement
- Improved pathfinding with terrain obstacle avoidance
- Enhanced logging for pathfinding decisions and ant movement
- New pathfinding algorithm to find nearest accessible points
- Improved digging behavior that respects terrain
- Two-pass terrain generation approach for improved clarity and maintainability
- Split `spawn_terrain` into `generate_initial_terrain` and `create_central_cavity` functions
- New `TileStore` system for efficient tile management and lookup
- Enhanced ant movement system with improved pathfinding and collision detection
- Better logging for debugging ant movement and pathfinding
- New tile type system using traits for flexible terrain representation
- Implemented `DirtTile` and `AirTile` types with specific behaviors
- Added `TilePosition` wrapper to handle Vec2 hashing for HashMap keys
- Enhanced terrain generation with proper tile type conversion
- Improved cavity creation by converting tiles to air instead of removing them
- New spacebar spawn system to create additional worker ants
- Enhanced logging for ant spawning and movement
- Improved ant movement system with better pathfinding
- WebAssembly support for playing the game in the browser
- GitHub Actions workflow for automatic WASM deployment to GitHub Pages
- Standalone index.html and 404.html files for WASM deployment
- Build script for both WASM and native builds
- Replaced build.sh script with a standard Makefile for better build system integration
- Added comprehensive Makefile targets for building, running, and testing
- Added detailed README.md with build instructions and project structure
- Added GitHub Actions workflow for automated WASM deployment
- Added support for both native and WASM builds
- Added local development server for testing WASM builds
- Added build validation checks for WASM output
- Added clean target to remove build artifacts

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
  - `src/ant/app.rs`: Application setup for both WASM and native builds
- Improved code organization and maintainability
- Enhanced ant movement and digging behavior
- Added better logging for ant state changes and actions
- Ant movement system now respects maximum distance from colony
- Pathfinding system now properly handles terrain obstacles
- Improved waypoint detection and movement precision
- Enhanced logging for debugging ant behavior
- Ants now find paths to the edge of terrain before digging
- Digging behavior now properly considers terrain obstacles
- Refactored terrain generation to use TileStore instead of separate solid tiles list
- Updated ant movement system to use TileStore for pathfinding
- Improved waypoint detection threshold for smoother ant movement
- Enhanced velocity control for better ant acceleration and movement
- Refactored `TileStore` to use the new tile type system
- Updated terrain generation to use tile types for better extensibility
- Modified cavity creation to convert tiles to air instead of removing them
- Improved tile rendering with type-specific colors
- Modified main.rs to support both native and WASM targets
- Restructured project to use lib.rs as the main entry point for WASM
- Simplified GitHub Actions workflow to use existing HTML files
- Unified application setup for both WASM and native builds
- Reorganized HTML files into a public directory for better deployment structure
- Updated build scripts to handle the new public directory structure
- Enhanced GitHub Actions workflow to properly deploy from the public directory
- Removed clean option from build script to avoid long Bevy rebuilds
- Eliminated duplication between serve_wasm.sh and serve.py
- Consolidated all build and test scripts into a single build.sh file
- Replaced build.sh with a standard Makefile for better build system integration
- Simplified build process with standardized Makefile commands
- Improved documentation with detailed project structure
- Enhanced GitHub Actions workflow to use Makefile
- Updated README with comprehensive build and development instructions

### Fixed

- Fixed type mismatches in chunk entity spawning
- Resolved issues with moved values in mesh generation
- Fixed camera positioning and zoom settings
- Addressed compilation errors related to mesh attributes
- Resolved borrowing conflicts in ant movement system
- Fixed terrain generation to properly handle tile removal
- Improved collision detection and pathfinding accuracy
- Resolved HashMap key issues with Vec2 by implementing custom Hash and Eq traits
- Fixed terrain generation to properly handle tile type conversion
- Fixed duplicate run_app function issue by moving it to ant/app.rs

### Technical Debt

- Several unused imports and variables need to be cleaned up
- Some struct fields and enum variants are currently unused and will be implemented in future updates
- Physics plugins were removed as they're not currently being used
- Removed redundant build scripts in favor of Makefile
- Consolidated build process into a single Makefile
- Improved build system maintainability
- Enhanced documentation structure

### Performance Improvements

- Added vsync and frame rate limiting (60 FPS) to reduce screen tearing and ensure consistent frame timing
- Optimized ant movement system to update at 30 FPS instead of every frame
- Reduced pheromone update frequency to 7.5 FPS with adjusted fade rates
- Improved camera movement smoothing with delta time capping and gentler acceleration/deceleration
- Added interpolation for smoother camera transitions
