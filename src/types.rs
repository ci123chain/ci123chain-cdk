use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Response {
    // let's make the positive case a struct, it contrains Msg: {...}, but also Data, Log, maybe later Events, etc.
    // pub messages: Vec<CosmosMsg>,
    // pub log: Vec<LogAttribute>, // abci defines this as string
    // pub data: Option<Binary>,   // abci defines this as bytes
    pub data: String,
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ContractResult {
    Ok(Response),
    Err(String),
}

impl ContractResult {
    // unwrap will panic on err, or give us the real data useful for tests
    pub fn unwrap(self) -> Response {
        match self {
            ContractResult::Err(msg) => panic!("Unexpected error: {}", msg),
            ContractResult::Ok(res) => res,
        }
    }

    pub fn is_err(&self) -> bool {
        match self {
            ContractResult::Err(_) => true,
            _ => false,
        }
    }
}

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

    fn getStore(prefix: String) -> Store {
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