#!/bin/bash 

cargo build --target wasm32-unknown-unknown --release

cd ./target/wasm32-unknown-unknown/release

wasm-strip ./*.wasm

wasm-opt -Oz ./*.wasm -o example.wasm

cd ../../..

cp target/wasm32-unknown-unknown/release/*.wasm target/

