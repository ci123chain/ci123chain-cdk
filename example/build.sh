#!/bin/bash 

cargo build --target wasm32-unknown-unknown --release

if [ $? -ne 0 ]; then
    exit 1
fi

cd ./target

cp ./wasm32-unknown-unknown/release/*.wasm ./

# https://github.com/WebAssembly/wabt
wasm-strip ./*.wasm

# https://github.com/WebAssembly/binaryen
wasm-opt -Oz ./*.wasm -o example.wasm

cd ..
