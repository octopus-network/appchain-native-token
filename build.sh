#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
if [ ! -d "out" ]; then
    mkdir -p "out"
fi
cp target/wasm32-unknown-unknown/release/appchain_native_token.wasm ./out/main.wasm