#!/bin/bash
cargo +nightly build --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/lorenz_attractor.wasm \\n  --out-dir .
npm install
npm run serve
