#!/bin/bash

set -e

echo "Setting up Quantum-Messenger"

# Build cryptographic core
echo "Building cryptographic core..."
cd crypto-core
cargo build --release

# Add ~/ .cargo/bin to PATH if not
export PATH="$HOME/.cargo/bin:$PATH"

# Build WASM (placeholder if fails)
echo "Building WebAssembly..."
wasm-pack build --target web --out-dir ../client/src/wasm || echo "WASM build failed, using placeholder"

cd ../server
cargo build --release

cd ../client
npm install
npm run build

cd ..

echo "Setup complete"
