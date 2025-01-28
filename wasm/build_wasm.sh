#!/bin/bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ../out/web/ \
    --out-name "sky-striker" \
    ../target/wasm32-unknown-unknown/release/sky-striker.wasm
cp -r ../assets ../out/web/

