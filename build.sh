#!/bin/bash 

cargo build --target wasm32-unknown-unknown --release

wasm-gc target/wasm32-unknown-unknown/release/rust_sdk.wasm 

cp target/wasm32-unknown-unknown/release/rust_sdk.wasm example.wasm
