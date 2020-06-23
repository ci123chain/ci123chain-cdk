use crate::codec::{Sink, Source};

#[derive(Clone, Default, PartialEq)]
pub struct Param {
    pub method: String,
    pub args: Vec<String>,
}

impl Param {
    pub(crate) fn from_slice(raw: &[u8]) -> Param {
        // [method, num of args, args...]
        // [string, u32,         string...]
        let mut source = Source::new(raw);
        let method = source.read_string();
        let num = source.read_u32();
        let mut args = vec![];
        for _ in 0..num {
            args.push(source.read_string());
        }
        Param {
            method: method,
            args: args,
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Address([u8; 20]);

impl Address {
    pub fn new(addr: &[u8; 20]) -> Address {
        Address(*addr)
    }

    pub fn zero() -> Address {
        Address([0; 20])
    }

    pub fn into(&self) -> [u8; 20] {
        self.0
    }

    pub fn into_slice(&self) -> &[u8] {
        &self.0[..]
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

#[derive(Clone, Debug, PartialEq)]
pub struct Response {
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContractResult {
    Ok(Response),
    Err(String),
}

impl ContractResult {
    pub(crate) fn to_vec(&self) -> Vec<u8> {
        // [ok or error, size of data, data]
        // [bool,        usize,        bytes]
        let mut sink = Sink::new(0);
        match self {
            ContractResult::Ok(resp) => {
                sink.write_bool(true);
                sink.write_usize(resp.data.len());
                sink.write_bytes(&resp.data);
                sink.into()
            }
            ContractResult::Err(err) => {
                sink.write_bool(false);
                sink.write_usize(err.len());
                sink.write_bytes(err.as_bytes());
                sink.into()
            }
        }
    }
}
