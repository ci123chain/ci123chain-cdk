use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct Param {
    pub method: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Address([u8; 20]);

impl Address {
    pub fn new(addr: &[u8; 20]) -> Address {
        Address(*addr)
    }

    pub fn zero() -> Address {
        Address([0; 20])
    }

    pub fn to_hex_string(&self) -> String {
        use core::fmt::Write;
        let mut s = String::with_capacity(self.0.len() * 2);
        for v in self.0.iter() {
            write!(s, "{:02x}", *v).unwrap();
        }
        s
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// pub struct Env {
//     pub block: BlockInfo,
//     pub message: MessageInfo,
//     pub contract: ContractInfo,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// pub struct BlockInfo {
//     pub height: i64,
//     // time is seconds since epoch begin (Jan. 1, 1970)
//     pub time: i64,
//     pub chain_id: String,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// pub struct MessageInfo {
//     pub signer: Addr,
//     // go likes to return null for empty array, make sure we can parse it (use option)
//     pub sent_funds: Option<Vec<Coin>>,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// pub struct ContractInfo {
//     pub address: Addr,
//     // go likes to return null for empty array, make sure we can parse it (use option)
//     pub balance: Option<Vec<Coin>>,
// }

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
// pub struct Coin {
//     pub denom: String,
//     pub amount: String,
// }
