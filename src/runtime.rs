use crate::errors::{Error, NullPointer};
use crate::types::Param;

use serde::{Serialize, Serializer};
use serde_json;

use std::collections::HashMap;
use std::mem;
use std::os::raw::c_void;

// This is the buffer we pre-allocate in get
static MAX_READ: usize = 2000;

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub fn make_dependencies() -> Dependencies {
    Dependencies {
        storage: Store::new(),
        api: ExternalApi::new(),
    }
}

pub fn ret(raw: &[u8]) {
    unsafe { return_contract(&*build_region(raw) as *const Region as *const c_void) };
}

pub fn notify(event: &Event) {
    let raw = serde_json::to_vec(event).unwrap();
    unsafe { notify_contract(&*build_region(&raw) as *const Region as *const c_void) };
}

#[derive(Serialize)]
pub struct Event {
    pub r#type: String,
    pub attr: HashMap<String, ItemValue>,
}

impl Event {
    pub fn new(event_type: String, attribute: HashMap<String, ItemValue>) -> Event {
        Event {
            r#type: event_type,
            attr: attribute,
        }
    }
}

pub enum ItemValue {
    String(String),
    Int64(i64),
}

impl Serialize for ItemValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self {
            ItemValue::String(value) => serializer.serialize_str(value),
            ItemValue::Int64(value) => serializer.serialize_i64(*value),
        }
    }
}

pub struct Dependencies {
    pub storage: Store,
    pub api: ExternalApi,
}

#[derive(Debug)]
pub struct Store {
    prefix: String,
}

impl Store {
    fn new() -> Store {
        Store {
            prefix: "test-".to_string(),
        }
    }
}

impl Store {
    pub fn set(&mut self, key: &[u8], value: &[u8]) {
        let mut prefix = self.prefix.clone();
        let real_key = unsafe { prefix.as_mut_vec() };
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

    // pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        
    //     let mut prefix = self.prefix.clone();
    //     let real_key = unsafe { prefix.as_mut_vec() };
    //     for &ele in key {
    //         real_key.push(ele);
    //     }
    //     let key = build_region(real_key);
    //     let key_ptr = &*key as *const Region as *const c_void;
    //     let value = allocate(MAX_READ);

    //     let read = unsafe { read_db(key_ptr, value) };
    //     if read == -1000002 {
    //         panic!("Allocated memory too small to hold the database value for the given key.");
    //     } else if read < 0 {
    //         panic!("An unknown error occurred in the read_db call.")
    //     }

    //     match unsafe { consume_region(value) } {
    //         Ok(data) => {
    //             if data.len() == 0 {
    //                 None
    //             } else {
    //                 Some(data)
    //             }
    //         }
    //         Err(_) => None,
    //     }
    // }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        const INITIAL: usize = 32;
        let mut val = vec![0; INITIAL];
        
        let mut prefix = self.prefix.clone();
        let real_key = unsafe { prefix.as_mut_vec() };
        for &ele in key {
            real_key.push(ele);
        }

        let key = build_region(real_key);
        let key_ptr = &*key as *const Region as *const c_void;

        let size = unsafe { read_db(key_ptr, val.as_mut_ptr(), val.len() as u32, 0) };
        let size = size as usize;
        val.resize(size, 0);
        if size > INITIAL {
            let value = &mut val[INITIAL..];
            debug_assert!(value.len() == size - INITIAL);
            unsafe {
                read_db(
                    key_ptr,
                    value.as_mut_ptr(),
                    value.len() as u32,
                    INITIAL as i32,
                )
            };
        }

        Some(val)
    }

    pub fn delete(&self, key: &[u8]) {
        let mut prefix = self.prefix.clone();
        let real_key = unsafe { prefix.as_mut_vec() };
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

pub struct ExternalApi {}

impl ExternalApi {
    fn new() -> ExternalApi {
        ExternalApi {}
    }
}

impl ExternalApi {
    pub fn input(&self) -> Param {
        let input_len = unsafe { get_input_length() };
        let buffer: Vec<u8> = vec![0; input_len as usize];
        let pointer = buffer.as_ptr();
        unsafe { get_input(pointer, input_len) };
        let param: Param = serde_json::from_slice(&buffer.as_ref()).unwrap();
        return param;
    }

    pub fn send(&self, to: &[u8], amount: &[u8]) -> Result<i32, i32> {
        let mut to = build_region(to);
        let to_ptr = &mut *to as *mut Region as *mut c_void;
        let mut amount = build_region(amount);
        let amount_ptr = &mut *amount as *mut Region as *mut c_void;

        let res = unsafe { send(to_ptr, amount_ptr) };
        if res == 0 {
            Ok(0)
        } else {
            Err(1)
        }
    }

    pub fn get_creator(&self) -> Option<Vec<u8>> {
        let creator_ptr = allocate(MAX_READ);
        unsafe { get_creator(creator_ptr) }
        match unsafe { consume_region(creator_ptr) } {
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

    pub fn get_invoker(&self) -> Option<Vec<u8>> {
        let invoker_ptr = allocate(MAX_READ);
        unsafe { get_invoker(invoker_ptr) }
        match unsafe { consume_region(invoker_ptr) } {
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

    pub fn get_timestamp(&self) -> Option<u64> {
        let data = unsafe { get_time() };
        Some(data)
    }
}

// This interface will compile into required Wasm imports.
extern "C" {
    fn get_input_length() -> i32;
    fn get_input(method: *const u8, size: i32);
    fn notify_contract(msg: *const c_void);
    fn return_contract(value: *const c_void);

    fn read_db(key: *const c_void, value: *mut u8, vsize: u32, offset: i32) -> i32;
    fn write_db(key: *const c_void, value: *mut c_void);
    fn delete_db(key: *const c_void);
    fn send(to_ptr: *mut c_void, amount_ptr: *mut c_void) -> i32;
    fn get_creator(creator_ptr: *mut c_void);
    fn get_invoker(invoker_ptr: *mut c_void);
    fn get_time() -> u64;
}

/// Refers to some heap allocated data in Wasm.
/// A pointer to an instance of this can be returned over FFI boundaries.
///
/// This struct is crate internal since the VM defined the same type independently.
#[repr(C)]
struct Region {
    offset: u32,
    /// The number of bytes available in this region
    capacity: u32,
    /// The number of bytes used in this region
    length: u32,
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
unsafe fn consume_region(ptr: *mut c_void) -> Result<Vec<u8>, Error> {
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
fn build_region(data: &[u8]) -> Box<Region> {
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

fn allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);
    pointer as *mut c_void
}

// fn deallocate(pointer: *mut c_void, capacity: usize) {
//     unsafe {
//         let _ = Vec::from_raw_parts(pointer, 0, capacity);
//     }
// }
