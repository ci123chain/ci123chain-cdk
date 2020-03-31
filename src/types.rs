use serde::{Deserialize, Serialize};
use std::os::raw::c_void;
use crate::errors::{Error, NullPointer};
use std::mem;

// This is the buffer we pre-allocate in get
static MAX_READ: usize = 2000;

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

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Addr(pub String);

impl Addr {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Env {
    pub block: BlockInfo,
    pub message: MessageInfo,
    pub contract: ContractInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct BlockInfo {
    pub height: i64,
    // time is seconds since epoch begin (Jan. 1, 1970)
    pub time: i64,
    pub chain_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MessageInfo {
    pub signer: Addr,
    // go likes to return null for empty array, make sure we can parse it (use option)
    pub sent_funds: Option<Vec<Coin>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContractInfo {
    pub address: Addr,
    // go likes to return null for empty array, make sure we can parse it (use option)
    pub balance: Option<Vec<Coin>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Coin {
    pub denom: String,
    pub amount: String,
}

// #[derive(Debug)]
// struct Deps {
// }

// impl Deps {
//     // add code here
//     fn getArgs(arg: Type) -> RetType {
//         unimplemented!();
//     }

//     fn getStringArgs(arg: Type) -> RetType {
//         unimplemented!();
//     }

//     fn getFunctionAndParameters(arg: Type) -> RetType {
//         unimplemented!();
//     }

//     fn getState(arg: Type) -> RetType {
//         unimplemented!();
//     }

//     fn getStore(prefix: String) -> Store {
//         unimplemented!();
//     }

//     fn getCreator(arg: Type) -> RetType {
//         unimplemented!();
//     }

//     fn getInvoker(arg: Type) -> RetType {
//         unimplemented!();
//     }

//     fn getTxTimestamp(arg: Type) -> RetType {
//         unimplemented!();
//     }
// }

pub trait Storage {
    fn set(&mut self, key: &[u8], value: &[u8]);
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn delete(&self, key: &[u8]);
}

#[derive(Debug)]
pub struct Store {
    pub prefix: String,
}

impl Storage for Store {
    fn set(&mut self, key: &[u8], value: &[u8]) {
        let mut prefix = self.prefix.clone();
        let real_key = unsafe{ prefix.as_mut_vec() };
        for &ele in key {
            real_key.push(ele);
        }
        let key = build_region(real_key);
        let key_ptr = &*key as *const Region as *const c_void;
        let mut value = build_region(value);
        let value_ptr = &mut *value as *mut Region as *mut c_void;
        unsafe {
            write_db(key_ptr, value_ptr);
        }
    }

    fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        let mut prefix = self.prefix.clone();
        let real_key = unsafe{ prefix.as_mut_vec() };
        for &ele in key {
            real_key.push(ele);
        }
        let key = build_region(real_key);
        let key_ptr = &*key as *const Region as *const c_void;
        let value = allocate(MAX_READ);

        let read = unsafe { read_db(key_ptr, value) };
        if read == -1000002 {
            panic!("Allocated memory too small to hold the database value for the given key.");
        } else if read < 0 {
            panic!("An unknown error occurred in the read_db call.")
        }

        match unsafe { consume_region(value) } {
            Ok(data) => {
                if data.len() == 0 {
                    None
                } else {
                    Some(data)
                }
            }
            Err(_) => None,
        }
    }

    fn delete(&self, key: &[u8]) {
        let mut prefix = self.prefix.clone();
        let real_key = unsafe{ prefix.as_mut_vec() };
        for &ele in key {
            real_key.push(ele);
        }
        let key = build_region(real_key);
        let key_ptr = &*key as *const Region as *const c_void;
        unsafe {
            delete_db(key_ptr);
        }
    }
}

// This interface will compile into required Wasm imports.
extern "C" {
    fn read_db(key: *const c_void, value: *mut c_void) -> i32;
    fn write_db(key: *const c_void, value: *mut c_void);
    fn delete_db(key: *const c_void);
}

/// Refers to some heap allocated data in Wasm.
/// A pointer to an instance of this can be returned over FFI boundaries.
///
/// This struct is crate internal since the VM defined the same type independently.
#[repr(C)]
pub struct Region {
    pub offset: u32,
    /// The number of bytes available in this region
    pub capacity: u32,
    /// The number of bytes used in this region
    pub length: u32,
}

/// Return the data referenced by the Region and
/// deallocates the Region (and the vector when finished).
/// Warning: only use this when you are sure the caller will never use (or free) the Region later
///
/// # Safety
///
/// If ptr is non-nil, it must refer to a valid Region, which was previously returned by alloc,
/// and not yet deallocated. This call will deallocate the Region and return an owner vector
/// to the caller containing the referenced data.
///
/// Naturally, calling this function twice on the same pointer will double deallocate data
/// and lead to a crash. Make sure to call it exactly once (either consuming the input in
/// the wasm code OR deallocating the buffer from the caller).
pub unsafe fn consume_region(ptr: *mut c_void) -> Result<Vec<u8>, Error> {
    if ptr.is_null() {
        return NullPointer {}.fail();
    }
    let region = Box::from_raw(ptr as *mut Region);
    let buffer = Vec::from_raw_parts(
        region.offset as *mut u8,
        region.length as usize,
        region.capacity as usize,
    );
    Ok(buffer)
}

/// Returns a box of a Region, which can be sent over a call to extern
/// note that this DOES NOT take ownership of the data, and we MUST NOT consume_region
/// the resulting data.
/// The Box must be dropped (with scope), but not the data
pub fn build_region(data: &[u8]) -> Box<Region> {
    let data_ptr = data.as_ptr() as usize;
    build_region_from_components(data_ptr as u32, data.len() as u32, data.len() as u32)
}

fn build_region_from_components(offset: u32, capacity: u32, length: u32) -> Box<Region> {
    Box::new(Region {
        offset,
        capacity,
        length,
    })
}

pub fn allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);
    pointer as *mut c_void
}

pub fn deallocate(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}