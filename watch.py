#!/usr/bin/env python3
import os
import time
import subprocess
import http.server
import socketserver
import threading
import webbrowser
from pathlib import Path
from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler

PORT = 8000
RELOAD_DELAY = 0.5  # Delay in seconds before triggering rebuild

class RebuildHandler(FileSystemEventHandler):
    def __init__(self):
        self.last_rebuild = 0
        self.rebuild_lock = threading.Lock()
        self.observer = None
        self.project_root = os.path.abspath(os.path.dirname(__file__))

    def on_modified(self, event):
        if event.is_directory:
            return
        
        # Only rebuild for Rust source files
        if not event.src_path.endswith(('.rs', '.toml')):
            return

        current_time = time.time()
        with self.rebuild_lock:
            if current_time - self.last_rebuild < RELOAD_DELAY:
                return
            self.last_rebuild = current_time

        print(f"\nChange detected in {event.src_path}")
        print("Rebuilding WASM...")
        
        # Run make wasm-dev in a subprocess from the project root
        result = subprocess.run(
            ['make', 'wasm-dev'],
            capture_output=True,
            text=True,
            cwd=self.project_root
        )
        if result.returncode == 0:
            print("Build successful!")
            # Trigger page reload by touching index.html
            index_path = Path('public/index.html')
            if index_path.exists():
                index_path.touch()
        else:
            print("Build failed:")
            print(result.stderr)

def start_server():
    class Handler(http.server.SimpleHTTPRequestHandler):
        def end_headers(self):
            # Add CORS headers for WASM
            self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
            self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
            super().end_headers()

    os.chdir('public')
    with socketserver.TCPServer(("", PORT), Handler) as httpd:
        print(f"Serving at http://localhost:{PORT}")
        print("Press Ctrl+C to stop the server")
        httpd.serve_forever()

def main():
    # Get the project root directory
    project_root = os.path.abspath(os.path.dirname(__file__))
    os.chdir(project_root)

    # Ensure public directory exists
    if not os.path.exists('public'):
        os.makedirs('public')

    # Initial build
    print("Performing initial build...")
    subprocess.run(['make', 'wasm-dev'], check=True, cwd=project_root)

    # Create index.html if it doesn't exist
    if not os.path.exists('public/index.html'):
        with open('public/index.html', 'w') as f:
            f.write('''<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Ant Farm Simulation</title>
    <script>
        // Auto-reload script
        let lastModified = new Date().getTime();
        setInterval(() => {
            fetch('index.html')
                .then(response => {
                    const currentModified = new Date(response.headers.get('Last-Modified')).getTime();
                    if (currentModified > lastModified) {
                        lastModified = currentModified;
                        window.location.reload();
                    }
                });
        }, 1000);
    </script>
</head>
<body>
    <script type="module">import init from "./pkg/ant.js";init();</script>
</body>
</html>''')

    # Start the file watcher
    event_handler = RebuildHandler()
    observer = Observer()
    observer.schedule(event_handler, path='src', recursive=True)
    observer.schedule(event_handler, path='.', recursive=False)  # Watch Cargo.toml
    observer.start()
    event_handler.observer = observer

    # Start the server in a separate thread
    server_thread = threading.Thread(target=start_server)
    server_thread.daemon = True
    server_thread.start()

    # Open the browser
    webbrowser.open(f'http://localhost:{PORT}')

    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        observer.stop()
        print("\nStopping server...")
    observer.join()

if __name__ == "__main__":
    main() 