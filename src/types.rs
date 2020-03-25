// 定义数据类型

#[derive(Debug)]
struct Deps {
}

impl Deps {
    // add code here
    fn getArgs(arg: Type) -> RetType {
        unimplemented!();
    }

    fn getStringArgs(arg: Type) -> RetType {
        unimplemented!();
    }

    fn getFunctionAndParameters(arg: Type) -> RetType {
        unimplemented!();
    }

    fn getState(arg: Type) -> RetType {
        unimplemented!();
    }

    fn getStore(prefix: Type) -> Store {
        unimplemented!();
    }

    fn getCreator(arg: Type) -> RetType {
        unimplemented!();
    }

    fn getInvoker(arg: Type) -> RetType {
        unimplemented!();
    }

    fn getTxTimestamp(arg: Type) -> RetType {
        unimplemented!();
    }
}

#[derive(Debug)]
struct Store {
    prefix: String
}

impl Store {
    // add code here
    fn set(key: Type, value: Type) -> RetType {
        unimplemented!();
    }

    fn get(arg: Type) -> RetType {
        unimplemented!();
    }
}