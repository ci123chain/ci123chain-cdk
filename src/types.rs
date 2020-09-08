use crate::codec::{from_hex_u8, Sink};
use crate::runtime::panic;
use crate::util::clone_into_array;

use crate::prelude::{Deserialize, Serialize, String, Vec, Write};

pub const ADDR_SIZE: usize = 20;
pub const ADDR_HEX_SIZE: usize = 42;

#[derive(Clone, Copy, Default, Debug, PartialEq, Deserialize, Serialize)]
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
        Self::new(&addr).unwrap()
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.0.len() * 2 + 2);
        s += "0x";
        for v in self.0.iter() {
            write!(s, "{:02x}", *v).unwrap();
        }
        s
    }
}

impl Into<[u8; ADDR_SIZE]> for Address {
    /// 取出Address内部的数组表示
    ///
    /// ```
    /// use c123chain_cdk::types::{Address, ADDR_SIZE};
    /// let address = Address::default();
    /// let array: [u8; ADDR_SIZE] = address.into();
    /// let array_ref = &array;
    /// ```
    fn into(self) -> [u8; ADDR_SIZE] {
        self.0
    }
}

impl Address {
    pub fn new(addr: &[u8]) -> Option<Address> {
        if addr.len() != ADDR_SIZE {
            None
        } else {
            Some(Address(clone_into_array(addr)))
        }
    }

    pub fn len() -> usize {
        ADDR_SIZE
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
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
