use crate::codec::Sink;
use crate::runtime::panic;
use crate::util::clone_into_array;
use hex;

use crate::prelude::{
    fmt, str, Deserialize, Deserializer, Error as SerdeError, Serialize, Serializer, String, Vec,
    Visitor,
};

pub const ADDR_SIZE: usize = 20;
pub const ADDR_HEX_SIZE: usize = 42;

#[derive(Clone, Copy, Default, Debug, PartialEq, Hash, Eq)]
pub struct Address([u8; ADDR_SIZE]);

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AddressVisitor;

        impl<'de> Visitor<'de> for AddressVisitor {
            type Value = Address;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Address")
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Address, E>
            where
                E: SerdeError,
            {
                match Address::from_bytes(value) {
                    Ok(a) => Ok(a),
                    Err(e) => Err(E::custom(e)),
                }
            }
        }
        deserializer.deserialize_bytes(AddressVisitor)
    }
}

impl From<&str> for Address {
    fn from(s: &str) -> Self {
        match Address::from_str(s) {
            Ok(a) => a,
            Err(e) => {
                panic(e);
                Address::default()
            }
        }
    }
}

impl From<&[u8]> for Address {
    fn from(b: &[u8]) -> Self {
        match Address::from_bytes(b) {
            Ok(a) => a,
            Err(e) => {
                panic(e);
                Address::default()
            }
        }
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        format!("{}{}", "0x", hex::encode(&self.0))
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

    pub(crate) fn from_bytes(raw: &[u8]) -> Result<Address, &str> {
        if raw.len() != ADDR_HEX_SIZE {
            return Err("unexpected address string length");
        }
        if &raw[0..2] != &[48, 120] {
            return Err("unexpected address string prefix");
        }
        let addr = hex::decode(&raw[2..])?;
        Ok(Self::new(&addr).unwrap())
    }

    pub fn from_str(s: &str) -> Result<Address, &str> {
        Address::from_bytes(s.as_bytes())
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
pub enum ContractResult {
    Ok(Vec<u8>),
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
                sink.write_bytes(&resp);
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

pub struct BlockHeader {
    pub height: u64,
    pub timestamp: u64,
}

#[derive(Debug)]
pub enum Error {
    UnexpectedEOF,
    InvalidUtf8,
    IrregularData,
}
