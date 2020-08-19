#!/bin/bash 

# Usage:
# release: ./build.sh
# debug: ./build.sh debug

mode="release"

if [ $# -ne 0 ] && [ $1 == "debug" ]; then
    mode="debug"
    cargo build --target wasm32-unknown-unknown
else
    cargo build --target wasm32-unknown-unknown --release
fi

if [ $? -ne 0 ]; then
    exit 1
    unset mode
fi

cd ./target

cp ./wasm32-unknown-unknown/${mode}/*.wasm ./

# https://github.com/WebAssembly/wabt
wasm-strip ./*.wasm

# https://github.com/WebAssembly/binaryen
wasm-opt -Oz ./*.wasm -o example.wasm

cd ..

unset mode