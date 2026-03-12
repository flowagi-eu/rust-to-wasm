#!/bin/bash
mkdir -p build

# Build Rust plugin
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/rust_plugin.wasm build/rust_plugin.wasm

node run.js
