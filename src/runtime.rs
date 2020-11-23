use crate::codec::{Sink, Source};
use crate::types::{Address, BlockHeader, ContractResult};

use crate::prelude::{panic, vec, Vec};

const INPUT_TOKEN: i32 = 0;

static mut PANIC_HOOK: bool = false;

pub fn make_dependencies() -> Dependencies {
    unsafe {
        if PANIC_HOOK == false {
            panic::set_hook(Box::new(|panic_info| {
                panic(&panic_info.to_string());
            }));
            PANIC_HOOK = true;
        }
    };
    Dependencies {
        storage: Store::new(),
        api: ExternalApi::new(),
    }
}

pub(crate) fn panic(data: &str) {
    unsafe {
        panic_contract(data.as_ptr(), data.len());
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
        Store { prefix: "" }
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

        let size = unsafe { read_db(key_ptr, key_size, value_ptr, value_size, 0) };
        if size < 0 {
            return None;
        }
        let size = size as usize;

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

    // 获取合约输入
    pub fn input(&self) -> Vec<u8> {
        self.get_input(INPUT_TOKEN)
    }

    // 代币转账
    pub fn send(&self, to: &Address, amount: u64) -> bool {
        unsafe { send(to.as_ptr(), amount) }
    }

    // 获取合约创建者(用户地址)
    pub fn get_creator(&self) -> Address {
        let creator = Address::default();
        unsafe { get_creator(creator.as_ptr() as *mut u8) };
        creator
    }

    // 获取合约调用者(用户地址)
    pub fn get_invoker(&self) -> Address {
        let invoker = Address::default();
        unsafe { get_invoker(invoker.as_ptr() as *mut u8) };
        invoker
    }

    // 获取合约调用者(用户地址或合约地址)
    pub fn get_pre_caller(&self) -> Address {
        let caller = Address::default();
        unsafe { get_pre_caller(caller.as_ptr() as *mut u8) };
        caller
    }

    // 获取当前合约地址
    pub fn self_address(&self) -> Address {
        let contract = Address::default();
        unsafe { self_address(contract.as_ptr() as *mut u8) };
        contract
    }

    // 获取当前block header
    pub fn get_block_header(&self) -> BlockHeader {
        let mut block_bytes = [0u8; 8 * 2];
        unsafe { get_block_header(block_bytes.as_mut_ptr() as *mut u8) };
        let source = Source::new(&block_bytes);
        let height = source.read_u64().unwrap();
        let timestamp = source.read_u64().unwrap();
        BlockHeader {
            height: height,
            timestamp: timestamp,
        }
    }

    // 合约返回
    pub fn ret(&self, result: ContractResult) {
        let output = result.to_vec();
        unsafe { return_contract(output.as_ptr(), output.len()) };
    }

    // 事件通知
    pub fn notify(&self, event: &Event) {
        let raw = event.to_vec();
        unsafe { notify_contract(raw.as_ptr(), raw.len()) };
    }

    // 调用合约
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

    // // 销毁本合约
    // pub fn destroy_contract(&self) {
    //     unsafe { destroy_contract() }
    // }

    // 获取指定验证者的权益
    pub fn get_validator_power(&self, validators: &[&Address]) -> Vec<u128> {
        let mut sink = Sink::new(0);
        sink.write_usize(validators.len());
        for &validator in validators.iter() {
            sink.write_bytes(validator.as_bytes());
        }
        let mut power = vec![0u128; validators.len()];
        unsafe {
            get_validator_power(
                sink.as_bytes().as_ptr(),
                sink.as_bytes().len(),
                power.as_mut_ptr() as *mut u8,
            )
        };
        power
    }

    // 获取所有验证者的权益之和
    pub fn total_power(&self) -> u128 {
        let mut power_bytes = [0u8; 16];
        unsafe { total_power(power_bytes.as_mut_ptr() as *mut u8) };
        let source = Source::new(&power_bytes);
        source.read_u128().unwrap()
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
    fn self_address(contract_ptr: *mut u8);
    fn get_pre_caller(caller_ptr: *mut u8);
    fn get_block_header(value_ptr: *mut u8);
    fn call_contract(addr_ptr: *const u8, input_ptr: *const u8, input_size: usize) -> i32;
    // fn destroy_contract();
    fn panic_contract(data_ptr: *const u8, data_size: usize);
    fn get_validator_power(data_ptr: *const u8, data_size: usize, value_ptr: *mut u8);
    fn total_power(value_ptr: *mut u8);

    #[cfg(debug_assertions)]
    pub fn debug_print(str_ptr: *const u8, str_size: usize);
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            use c123chain_cdk::runtime::debug_print;
            let s = format!($($arg)*);
            unsafe { debug_print(s.as_ptr(), s.len()) };
        }
    }
}
