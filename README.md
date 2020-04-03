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
{"count":3}
write key [test-count], value [3]
{"ok":{"messages":[],"log":[],"data":[123,34,99,111,117,110,116,34,58,51,125]}}
{"increment":{}}
read key [test-count]
write key [test-count], value [4]
{"ok":{"messages":[],"log":[],"data":[51]}}
{"reset":{"count":10}}
write key [test-count], value [10]
{"ok":{"messages":[],"log":[],"data":[123,34,114,101,115,101,116,34,58,123,34,99,111,117,110,116,34,58,49,48,125,125]}}
null
{"ok":{"messages":[],"log":[],"data":[114,101,115,117,108,116,32,102,114,111,109,32,113,117,101,114,121]}}
```

如果提示没有对应的工具链，使用 `rustup show` 查看已安装的工具链，并安装 `wasm32-unknown-unknown target`:

```shell
$ proxychains rustup target add wasm32-unknown-unknown
```