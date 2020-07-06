use crate::codec::{from_hex_u8, Sink};
use crate::runtime::panic;

use crate::prelude::{String, Vec, Write};

const ADDR_SIZE: usize = 20;
const ADDR_HEX_SIZE: usize = 42;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Address([u8; ADDR_SIZE]);

impl From<&str> for Address {
    fn from(s: &str) -> Self {
        if s.len() != ADDR_HEX_SIZE {
            panic("invalid address string length");
        }
        if &s[0..2] != "0x" {
            panic("unexpected address string prefix")
        }
        let raw = s.as_bytes();
        let mut addr = [0 as u8; ADDR_SIZE];
        let (mut i, mut j) = (0, 3);
        while j < s.len() {
            let panic_err = "unexpected base64 char";
            let (mut a, mut b) = (0, 0);
            let res = from_hex_u8(raw[j - 1]);
            match res {
                Ok(c) => a = c,
                Err(_) => panic(panic_err),
            }
            let res = from_hex_u8(raw[j]);
            match res {
                Ok(c) => b = c,
                Err(_) => panic(panic_err),
            }
            addr[i] = (a << 4) | b;
            i += 1;
            j += 2;
        }
        Self::new(&addr)
    }
}

impl Address {
    pub fn new(addr: &[u8; ADDR_SIZE]) -> Address {
        Address(*addr)
    }

    pub fn zero() -> Address {
        Address([0; ADDR_SIZE])
    }

    pub fn len() -> usize {
        ADDR_SIZE
    }

    pub fn into(&self) -> [u8; ADDR_SIZE] {
        self.0
    }

    pub fn into_slice(&self) -> &[u8] {
        &self.0[..]
    }

    pub fn to_hex_string(&self) -> String {
        let mut s = String::with_capacity(self.0.len() * 2 + 2);
        s += "0x";
        for v in self.0.iter() {
            write!(s, "{:02x}", *v).unwrap();
        }
        s
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0.as_mut_ptr()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Response<'a> {
    pub data: &'a [u8],
}

#[derive(Clone, Debug, PartialEq)]
pub enum ContractResult<'a> {
    Ok(Response<'a>),
    Err(&'a str),
}

impl ContractResult<'_> {
    pub(crate) fn to_vec(&self) -> Vec<u8> {
        // [ok or error, size of data, data]
        // [bool,        usize,        bytes]
        let mut sink = Sink::new(0);
        match self {
            ContractResult::Ok(resp) => {
                sink.write_bool(true);
                sink.write_bytes(&resp.data);
                sink.into()
            }
            ContractResult::Err(err) => {
                sink.write_bool(false);
                sink.write_bytes(err.as_bytes());
                sink.into()
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    UnexpectedEOF,
    InvalidUtf8,
    IrregularData,
}
