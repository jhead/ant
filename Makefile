# Ant Farm Simulation Makefile

# Default target
.PHONY: help
help:
	@echo "Ant Farm Simulation Build System"
	@echo ""
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  help        Show this help message"
	@echo "  wasm        Build for WebAssembly"
	@echo "  wasm-dev    Build for WebAssembly (fast development build)"
	@echo "  native      Build for native platform"
	@echo "  run         Run the native build"
	@echo "  all         Build for both WASM and native"
	@echo "  serve       Serve the WASM build locally"
	@echo "  watch       Serve with auto-rebuild on file changes"
	@echo "  check       Check if WASM build is valid"
	@echo "  test        Test the WASM build with a simple server"
	@echo "  clean       Clean build artifacts (except Bevy dependencies)"

# Build for WebAssembly
.PHONY: wasm
wasm:
	@echo "Building for WebAssembly..."
	wasm-pack build --target web
	@echo "Copying WASM files to public directory..."
	@mkdir -p public/pkg
	@cp -r pkg/* public/pkg/
	@echo "WASM build complete. Output in public/pkg directory."

# Build for WebAssembly (fast development build)
.PHONY: wasm-dev
wasm-dev:
	@echo "Building for WebAssembly (fast development build)..."
	RUSTFLAGS="-C link-arg=-s" wasm-pack build --dev --target web
	@echo "Copying WASM files to public directory..."
	@mkdir -p public/pkg
	@cp -r pkg/* public/pkg/
	@echo "WASM development build complete. Output in public/pkg directory."

# Check if WASM build is valid
.PHONY: check
check:
	@echo "Checking WASM build..."
	@if [ ! -d "pkg" ]; then \
		echo "pkg directory not found! Run 'make wasm' first."; \
		exit 1; \
	fi
	@if [ ! -f "pkg/ant.js" ]; then \
		echo "ant.js file not found! Run 'make wasm' first."; \
		exit 1; \
	fi
	@if [ ! -f "pkg/ant_bg.wasm" ]; then \
		echo "ant_bg.wasm file not found! Run 'make wasm' first."; \
		exit 1; \
	fi
	@echo "WASM build check successful!"
	@echo "Files found:"
	@ls -la pkg/

# Test the WASM build with a simple server
.PHONY: test
test:
	@echo "Testing WASM build..."
	@if [ ! -d "pkg" ]; then \
		echo "pkg directory not found! Run 'make wasm' first."; \
		exit 1; \
	fi
	@echo "Build check successful!"
	@echo "You can now serve the files using a local server:"
	@echo "  python3 -m http.server 8000"
	@echo "Then open your browser and navigate to http://localhost:8000"

# Build for native platform
.PHONY: native
native:
	@echo "Building for native platform..."
	cargo build --release
	@echo "Native build complete."

# Run the native build
.PHONY: run
run:
	@echo "Running native build..."
	cargo run --release

# Build for both WASM and native
.PHONY: all
all: wasm native

# Serve the WASM build locally
.PHONY: serve
serve:
	@echo "Serving WASM build locally..."
	@if [ ! -d "public" ]; then \
		echo "Creating public directory..."; \
		mkdir -p public; \
	fi
	@if [ ! -d "public/pkg" ]; then \
		echo "WASM build not found. Building first..."; \
		make wasm; \
	fi
	@if [ ! -f "public/index.html" ]; then \
		echo "Creating index.html..."; \
		echo '<!DOCTYPE html><html><head><meta charset="utf-8"><title>Ant Farm Simulation</title></head><body><script type="module">import init from "./pkg/ant.js";init();</script></body></html>' > public/index.html; \
	fi
	@echo "Starting server..."
	./serve.py

# Serve with auto-rebuild on file changes
.PHONY: watch
watch:
	@echo "Starting development server with auto-rebuild..."
	@if [ ! -f "watch.py" ]; then \
		echo "watch.py not found! Please ensure it exists in the project root."; \
		exit 1; \
	fi
	@chmod +x watch.py
	@./watch.py

# Clean build artifacts (except Bevy dependencies)
.PHONY: clean
clean:
	@echo "Cleaning build artifacts..."
	@rm -rf pkg
	@echo "Note: Bevy dependencies are not cleaned to avoid long rebuilds." 