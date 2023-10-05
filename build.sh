#!/bin/bash
set -e

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release --workspace --exclude tests
mkdir -p res && cp target/wasm32-unknown-unknown/release/*.wasm ./res/
