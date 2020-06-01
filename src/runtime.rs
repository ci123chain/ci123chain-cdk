use crate::errors::Error;
use crate::types::{Address, ContractResult, Param, Response};

use serde::{Serialize, Serializer};
use serde_json;

use std::collections::HashMap;
use std::os::raw::c_void;

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

pub fn ret(result: Result<Response, Error>) {
    match result {
        Ok(response) => {
            let output = serde_json::to_vec(&ContractResult::Ok(response)).unwrap();
            unsafe { return_contract(output.as_ptr() as *const c_void, output.len()) };
        }
        Err(err) => {
            let output = serde_json::to_vec(&ContractResult::Err(err.to_string())).unwrap();
            unsafe { return_contract(output.as_ptr() as *const c_void, output.len()) };
        }
    }
}

pub fn notify(event: &Event) {
    let raw = serde_json::to_vec(event).unwrap();
    unsafe { notify_contract(raw.as_ptr() as *const c_void, raw.len()) };
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

        let key_ptr = key.as_ptr() as *const c_void;
        let key_size = key.len();

        let value_ptr = value.as_ptr() as *const c_void;
        let value_size = value.len();
        unsafe {
            write_db(key_ptr, key_size, value_ptr, value_size);
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        const INITIAL: usize = 32;
        let mut real_value = vec![0; INITIAL];
        let mut prefix = self.prefix.clone();
        let real_key = unsafe { prefix.as_mut_vec() };
        for &ele in key {
            real_key.push(ele);
        }

        let key_ptr = key.as_ptr() as *const c_void;
        let key_size = key.len();

        let value_ptr = real_value.as_mut_ptr() as *mut c_void;
        let value_size = real_value.len();

        let size = unsafe { read_db(key_ptr, key_size, value_ptr, value_size, 0) } as usize;

        real_value.resize(size, 0);
        if size > INITIAL {
            let value = &mut real_value[INITIAL..];
            unsafe {
                read_db(
                    key_ptr,
                    key_size,
                    value.as_mut_ptr() as *mut c_void,
                    value.len(),
                    INITIAL,
                )
            };
        }

        Some(real_value)
    }

    pub fn delete(&self, key: &[u8]) {
        let mut prefix = self.prefix.clone();
        let real_key = unsafe { prefix.as_mut_vec() };
        for &ele in key {
            real_key.push(ele);
        }
        let key_ptr = key.as_ptr() as *const c_void;
        let key_size = key.len();
        unsafe {
            delete_db(key_ptr, key_size);
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
        let size = unsafe { get_input_length() };
        let input: Vec<u8> = vec![0; size];
        let pointer = input.as_ptr();
        unsafe { get_input(pointer, size) };
        let param: Param = serde_json::from_slice(&input.as_ref()).unwrap();
        return param;
    }

    pub fn send(&self, to: &Address, amount: i64) -> Result<i32, i32> {
        let res = unsafe { send(to.as_ptr() as *const c_void, amount) };
        if res == 0 {
            Ok(0)
        } else {
            Err(1)
        }
    }

    pub fn get_creator(&self) -> Address {
        let creator = Address::zero();
        unsafe { get_creator(creator.as_ptr() as *mut c_void) };
        creator
    }

    pub fn get_invoker(&self) -> Address {
        let invoker = Address::zero();
        unsafe { get_invoker(invoker.as_ptr() as *mut c_void) };
        invoker
    }

    pub fn get_timestamp(&self) -> Option<u64> {
        let data = unsafe { get_time() };
        Some(data)
    }
}

// This interface will compile into required Wasm imports.
extern "C" {
    fn get_input_length() -> usize;
    fn get_input(method: *const u8, size: usize);
    fn notify_contract(msg: *const c_void, msg_size: usize);
    fn return_contract(value: *const c_void, value_size: usize);

    fn read_db(
        key_ptr: *const c_void,
        key_size: usize,
        value_ptr: *mut c_void,
        value_size: usize,
        offset: usize,
    ) -> i32;
    fn write_db(
        key_ptr: *const c_void,
        key_size: usize,
        value_ptr: *const c_void,
        value_size: usize,
    );
    fn delete_db(key_ptr: *const c_void, key_size: usize);
    fn send(to_ptr: *const c_void, amount: i64) -> i32;
    fn get_creator(creator_ptr: *mut c_void);
    fn get_invoker(invoker_ptr: *mut c_void);
    fn get_time() -> u64;
}

// /// Refers to some heap allocated data in Wasm.
// /// A pointer to an instance of this can be returned over FFI boundaries.
// ///
// /// This struct is crate internal since the VM defined the same type independently.
// #[repr(C)]
// struct Region {
//     offset: u32,
//     /// The number of bytes available in this region
//     capacity: u32,
//     /// The number of bytes used in this region
//     length: u32,
// }

// /// Return the data referenced by the Region and
// /// deallocates the Region (and the vector when finished).
// /// Warning: only use this when you are sure the caller will never use (or free) the Region later
// ///
// /// # Safety
// ///
// /// If ptr is non-nil, it must refer to a valid Region, which was previously returned by alloc,
// /// and not yet deallocated. This call will deallocate the Region and return an owner vector
// /// to the caller containing the referenced data.
// ///
// /// Naturally, calling this function twice on the same pointer will double deallocate data
// /// and lead to a crash. Make sure to call it exactly once (either consuming the input in
// /// the wasm code OR deallocating the buffer from the caller).
// unsafe fn consume_region(ptr: *mut c_void) -> Result<Vec<u8>, Error> {
//     if ptr.is_null() {
//         return NullPointer {}.fail();
//     }
//     let region = Box::from_raw(ptr as *mut Region);
//     let buffer = Vec::from_raw_parts(
//         region.offset as *mut u8,
//         region.length as usize,
//         region.capacity as usize,
//     );
//     Ok(buffer)
// }

// /// Returns a box of a Region, which can be sent over a call to extern
// /// note that this DOES NOT take ownership of the data, and we MUST NOT consume_region
// /// the resulting data.
// /// The Box must be dropped (with scope), but not the data
// fn build_region(data: &[u8]) -> Box<Region> {
//     let data_ptr = data.as_ptr() as usize;
//     build_region_from_components(data_ptr as u32, data.len() as u32, data.len() as u32)
// }

// fn build_region_from_components(offset: u32, capacity: u32, length: u32) -> Box<Region> {
//     Box::new(Region {
//         offset,
//         capacity,
//         length,
//     })
// }

// fn allocate(size: usize) -> *mut c_void {
//     let mut buffer = Vec::with_capacity(size);
//     let pointer = buffer.as_mut_ptr();
//     mem::forget(buffer);
//     pointer as *mut c_void
// }

// fn deallocate(pointer: *mut c_void, capacity: usize) {
//     unsafe {
//         let _ = Vec::from_raw_parts(pointer, 0, capacity);
//     }
// }
