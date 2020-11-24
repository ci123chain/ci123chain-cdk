extern crate c123chain_cdk as cdk;
extern crate c123chain_cdk_proc as cdk_proc;

use cdk::math;
use cdk::runtime;
use cdk::runtime::ItemValue::Str as IString;
use cdk::types::{Address, ContractResult};
use cdk_proc::external_fn;

use c123chain_cdk::debug;

#[external_fn]
fn read_db(key: &str) -> ContractResult {
    ContractResult::Ok(read(key).into_bytes())
}

#[external_fn]
fn write_db(key: &str, value: &str) -> ContractResult {
    write(key, value);
    debug!("{}: {}", key, value);
    ContractResult::Ok("success".to_string().into_bytes())
}

#[external_fn]
fn delete_db(key: &str) -> ContractResult {
    delete(key);
    ContractResult::Ok("success".to_string().into_bytes())
}

#[external_fn]
fn send(addr_str: &str, amount: u64) -> ContractResult {
    ContractResult::Ok(
        api()
            .send(&addr_str.into(), amount)
            .to_string()
            .into_bytes(),
    )
}

#[external_fn]
fn get_creator() -> ContractResult {
    let creator = api().get_creator();
    ContractResult::Ok(creator.to_string().into_bytes())
}

#[external_fn]
fn get_invoker() -> ContractResult {
    let invoker = api().get_invoker();
    ContractResult::Ok(invoker.to_string().into_bytes())
}

#[external_fn]
fn self_address() -> ContractResult {
    let contract_address = api().self_address();
    ContractResult::Ok(contract_address.to_string().into_bytes())
}

#[external_fn]
fn get_pre_caller() -> ContractResult {
    let caller_address = api().get_pre_caller();
    ContractResult::Ok(caller_address.to_string().into_bytes())
}

#[external_fn]
fn get_block_header() -> ContractResult {
    let block = api().get_block_header();
    ContractResult::Ok(
        format!("height: {}, timestamp: {}", block.height, block.timestamp).into_bytes(),
    )
}

#[external_fn]
fn call_contract(addr_str: &str, ret_input: &[u8]) -> ContractResult {
    match api().call_contract(&addr_str.into(), ret_input) {
        Some(res) => ContractResult::Ok(res),
        None => ContractResult::Err("call contract error".to_string()),
    }
}

#[external_fn]
fn new_contract(code_hash: &[u8], input: &[u8]) -> ContractResult {
    let new_addr = api().new_contract(code_hash, input);
    ContractResult::Ok(new_addr.to_string().into_bytes())
}

#[external_fn]
fn mul(x: u128, y: u128) -> ContractResult {
    ContractResult::Ok(math::safe_mul(x, y).to_string().into_bytes())
}

#[external_fn]
fn notify() -> ContractResult {
    event("event type", "event msg");
    ContractResult::Ok("success".to_string().into_bytes())
}

#[external_fn]
fn get_validator_power() -> ContractResult {
    let validators = [
        &Address::default(),
        &Address::default(),
        &Address::default(),
    ];
    let power = api().get_validator_power(&validators);
    ContractResult::Ok(format!("{:?}", power).into_bytes())
}

#[external_fn]
fn total_power() -> ContractResult {
    let total_power = api().total_power();
    ContractResult::Ok(total_power.to_string().into_bytes())
}

fn api() -> runtime::ExternalApi {
    runtime::make_dependencies().api
}

fn event(method: &str, msg: &str) {
    let mut event = runtime::Event::new(method);
    event.add("msg", IString(msg));
    api().notify(&event);
}

fn read(key: &str) -> String {
    let val = runtime::make_dependencies()
        .storage
        .get(key.as_bytes())
        .unwrap();
    String::from_utf8(val).unwrap()
}

fn write(key: &str, value: &str) {
    runtime::make_dependencies()
        .storage
        .set(key.as_bytes(), value.as_bytes())
}

fn delete(key: &str) {
    runtime::make_dependencies().storage.delete(key.as_bytes())
}
