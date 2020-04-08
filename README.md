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
......
```

如果提示没有对应的工具链，使用 `rustup show` 查看已安装的工具链，并安装 `wasm32-unknown-unknown target`:

```shell
$ proxychains rustup target add wasm32-unknown-unknown
```

# 参数

传入结构体为：

```go
type ContractParam struct {
	Method string   `json:"method"`
	Args   []string `json:"args"`
}
```

其中 `Method` 是具体的方法名，例如 demo 合约中调用 export 的函数 handle，并分别调用不同的 Method: inc 和 reset。`Args` 为调用参数。

返回结构体为：

```go
type Response struct {
	Data    []byte                   `json:"data"`
}
```

由于 Rust 返回的 Result(实际上是个枚举体) 分为 `ok` 和 `err` 两种情况，若为 `ok` 则是返回 `Response`，否则返回 `string`。因此需要先解析相应的字段再进一步取出数据，详细参考 demo `/test/go/main.go` 。

正常情况下，合约内部错误通过 Result 返回，如果go层调用出现错误，那么是合约执行发生了panic，如数组访问越界等。