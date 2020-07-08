# Rust 智能合约 SDK

```
├── Cargo.toml          # sdk rust项目配置文件
├── example
│   ├── build.sh        # 编译脚本，使用 wasm-opt 等工具，若无工具不影响文件输出
│   ├── Cargo.toml      # example rust项目配置文件
│   └── src
│       └── lib.rs      # example 合约
├── README.md
├── src
│   ├── codec.rs        # 与 go 层交互的编解码基础设施
│   ├── lib.rs          # 包引用
│   ├── math.rs         # u128 和 i128 的安全计算
│   ├── runtime.rs      # 为合约提供运行时的 api 和 store
│   └── types.rs        # 类型定义，如 Address
└── test
    └── go
        ├── api.go      # go 层 api
        ├── codec.go    # 与合约交互的编解码基础设施
        ├── go.mod
        ├── go.sum
        ├── main.go     # go example
        ├── region.go   # 弃用的 wasm 内存指针
        └── store.go    # go 层 store
```

函数原型定义可以查看 `src/runtime.rs` 文件的最后部分，其中 `send` 和 `migrate_contract` 返回 `bool` ，在 go 层需要返回 `1 (true)` 或者 `0 (false)`。
