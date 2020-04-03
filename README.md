# Rust 智能合约 SDK

计数合约demo，/test/go下为对应的go-ext-wasm程序。

环境要求: go, rust, gcc(cgo)。

# 编译

wasm-gc用于减小wasm体积，不一定要执行。现有的/bin/wasm-gc是linux的，mac可以从 [https://github.com/alexcrichton/wasm-gc/releases](https://github.com/alexcrichton/wasm-gc/releases) 下载。当前目录下：

```shell
$ cargo build --target wasm32-unknown-unknown
$ ./bin/wasm-gc ./target/wasm32-unknown-unknown/debug/rust_sdk.wasm
$ cd ./test/go
$ go build -o main && ./main
{"method":"init","args":["3"]}
write key [test-count], value [3]
{"ok":{"messages":[],"log":[],"data":[91,34,51,34,44,34,105,110,105,116,34,93]}}
{"method":"inc","args":[]}
read key [test-count]
write key [test-count], value [4]
{"ok":{"messages":[],"log":[],"data":[51]}}
{"method":"reset","args":["10"]}
write key [test-count], value [10]
{"ok":{"messages":[],"log":[],"data":[91,34,49,48,34,44,34,114,101,115,101,116,34,93]}}
{"method":"init","args":[]}
{"ok":{"messages":[],"log":[],"data":[91,34,105,110,105,116,34,44,34,113,117,101,114,121,34,93]}}
```

如果提示没有对应的工具链，使用 `rustup show` 查看已安装的工具链，并安装 `wasm32-unknown-unknown target`:

```shell
$ proxychains rustup target add wasm32-unknown-unknown
```