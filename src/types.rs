#[derive(Clone, Default, PartialEq)]
pub struct Param {
    pub method: String,
    pub args: Vec<String>,
}

impl Param {
    pub(crate) fn from_slice(raw: &[u8]) -> Param {
        Param {
            method: String::from("ok"),
            args: vec![], //TODO
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
        vec![] //TODO
    }
}
