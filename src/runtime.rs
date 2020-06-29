use crate::codec::{Sink, Source};
use crate::types::{Address, ContractResult, Response};

use crate::prelude::{vec, Vec};

pub fn make_dependencies() -> Dependencies {
    Dependencies {
        storage: Store::new(),
        api: ExternalApi::new(),
    }
}

pub struct Event<'a> {
    pub r#type: &'a str,
    pub attr: Vec<(&'a str, ItemValue<'a>)>,
}

impl<'a> Event<'a> {
    pub fn new(event_type: &'a str) -> Event {
        Event {
            r#type: event_type,
            attr: vec![],
        }
    }

    pub fn add(&mut self, key: &'a str, value: ItemValue<'a>) {
        self.attr.push((key, value));
    }

    pub(crate) fn to_vec(&self) -> Vec<u8> {
        // [type,   size of attr, [key,    type of value, value    ]...]
        // [string, usize,        [string, byte,          ItemValue]...]
        let mut sink = Sink::new(0);
        sink.write_str(&self.r#type);
        sink.write_usize(self.attr.len());
        for (k, v) in self.attr.iter() {
            sink.write_str(k);
            match v {
                ItemValue::Int64(i) => {
                    sink.write_byte(0);
                    sink.write_i64(*i);
                }
                ItemValue::Str(s) => {
                    sink.write_byte(1);
                    sink.write_str(s);
                }
            }
        }
        sink.into()
    }
}

pub enum ItemValue<'a> {
    Str(&'a str),
    Int64(i64),
}

pub struct Dependencies {
    pub storage: Store,
    pub api: ExternalApi,
}

pub struct Store {
    prefix: &'static str,
}

impl Store {
    fn new() -> Store {
        Store { prefix: "test-" }
    }

    pub fn set(&self, key: &[u8], value: &[u8]) {
        let key = self.gen_key(key);

        let key_ptr = key.as_ptr();
        let key_size = key.len();

        let value_ptr = value.as_ptr();
        let value_size = value.len();
        unsafe {
            write_db(key_ptr, key_size, value_ptr, value_size);
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        const INITIAL: usize = 32;

        let key = self.gen_key(key);
        let mut value = vec![0; INITIAL];

        let key_ptr = key.as_ptr();
        let key_size = key.len();

        let value_ptr = value.as_mut_ptr();
        let value_size = value.len();

        let size = unsafe { read_db(key_ptr, key_size, value_ptr, value_size, 0) } as usize;

        value.resize(size, 0);
        if size > INITIAL {
            let value = &mut value[INITIAL..];
            unsafe { read_db(key_ptr, key_size, value.as_mut_ptr(), value.len(), INITIAL) };
        }

        Some(value)
    }

    pub fn delete(&self, key: &[u8]) {
        let key = self.gen_key(key);

        let key_ptr = key.as_ptr();
        let key_size = key.len();
        unsafe {
            delete_db(key_ptr, key_size);
        }
    }

    fn gen_key(&self, key: &[u8]) -> Vec<u8> {
        let mut real_key: Vec<u8> = self.prefix.as_bytes().iter().cloned().collect();
        for &ele in key {
            real_key.push(ele);
        }
        real_key
    }
}

pub struct ExternalApi {}

impl<'a> ExternalApi {
    fn new() -> ExternalApi {
        ExternalApi {}
    }

    pub fn input(&self) -> Source {
        Source::new(self.get_input(0))
    }

    pub fn send(&self, to: &Address, amount: u64) -> bool {
        unsafe { send(to.as_ptr(), amount) }
    }

    pub fn get_creator(&self) -> Address {
        let creator = Address::zero();
        unsafe { get_creator(creator.as_ptr() as *mut u8) };
        creator
    }

    pub fn get_invoker(&self) -> Address {
        let invoker = Address::zero();
        unsafe { get_invoker(invoker.as_ptr() as *mut u8) };
        invoker
    }

    pub fn get_timestamp(&self) -> u64 {
        unsafe { get_time() }
    }

    pub fn ret(&self, result: Result<Response, &'a str>) {
        match result {
            Ok(response) => {
                let output = ContractResult::Ok(response).to_vec();
                unsafe { return_contract(output.as_ptr(), output.len()) };
            }
            Err(err) => {
                let output = ContractResult::Err(err).to_vec();
                unsafe { return_contract(output.as_ptr(), output.len()) };
            }
        }
    }

    pub fn notify(&self, event: &Event) {
        let raw = event.to_vec();
        unsafe { notify_contract(raw.as_ptr(), raw.len()) };
    }

    pub fn call_contract(&self, addr: &Address, input: &[u8]) -> Option<Vec<u8>> {
        let addr_ptr = addr.as_ptr();
        let input_ptr = input.as_ptr();
        let size = input.len();
        let token = unsafe { call_contract(addr_ptr, input_ptr, size) };
        if token < 0 {
            return None;
        }
        Some(self.get_input(token))
    }

    fn get_input(&self, token: i32) -> Vec<u8> {
        let input_size = unsafe { get_input_length(token) };
        let input: Vec<u8> = vec![0; input_size];
        let input_ptr = input.as_ptr();
        unsafe { get_input(token, input_ptr, input_size) };
        input
    }
}

// This interface will compile into required Wasm imports.
extern "C" {
    fn get_input_length(token: i32) -> usize;
    fn get_input(token: i32, input_ptr: *const u8, input_size: usize);
    fn notify_contract(msg_ptr: *const u8, msg_size: usize);
    fn return_contract(value_ptr: *const u8, value_size: usize);

    fn read_db(
        key_ptr: *const u8,
        key_size: usize,
        value_ptr: *mut u8,
        value_size: usize,
        offset: usize,
    ) -> i32;
    fn write_db(key_ptr: *const u8, key_size: usize, value_ptr: *const u8, value_size: usize);
    fn delete_db(key_ptr: *const u8, key_size: usize);
    fn send(to_ptr: *const u8, amount: u64) -> bool;
    fn get_creator(creator_ptr: *mut u8);
    fn get_invoker(invoker_ptr: *mut u8);
    fn get_time() -> u64;
    fn call_contract(addr_ptr: *const u8, input_ptr: *const u8, input_size: usize) -> i32;
}
