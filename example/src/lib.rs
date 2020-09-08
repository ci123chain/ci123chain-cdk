extern crate c123chain_cdk as cdk;

use cdk::codec::Source;
use cdk::math;
use cdk::runtime;
use cdk::runtime::ItemValue::Str as IString;
use cdk::types::{Address, Response};

use c123chain_cdk::debug;

#[no_mangle]
pub fn invoke() {
    let deps = runtime::make_dependencies();
    let input = deps.api.input();
    let input = Source::new(&input);
    let method = input.read_str().unwrap();
    match method {
        "read_db" => {
            let key = input.read_str().unwrap();
            return_contract(Ok(Response {
                data: read_db(key).as_bytes(),
            }));
        }
        "write_db" => {
            let key = input.read_str().unwrap();
            let value = input.read_str().unwrap();
            write_db(key, value);
            return_contract(Ok(Response {
                data: "success".as_bytes(),
            }));
        }
        "delete_db" => {
            let key = input.read_str().unwrap();
            delete_db(key);
            return_contract(Ok(Response {
                data: "success".as_bytes(),
            }));
        }
        "send" => {
            let addr: Address = input.read_str().unwrap().into();
            let amount = input.read_u64().unwrap();
            let res = deps.api.send(&addr, amount);
            return_contract(Ok(Response {
                data: res.to_string().as_bytes(),
            }));
        }
        "get_creator" => {
            let creator = deps.api.get_creator();
            return_contract(Ok(Response {
                data: creator.to_string().as_bytes(),
            }));
        }
        "get_invoker" => {
            let invoker = deps.api.get_invoker();
            return_contract(Ok(Response {
                data: invoker.to_string().as_bytes(),
            }));
        }
        "self_address" => {
            let contract_address = deps.api.self_address();
            return_contract(Ok(Response {
                data: contract_address.to_string().as_bytes(),
            }));
        }
        "get_pre_caller" => {
            debug!("debug {} from contract", "message");

            let caller_address = deps.api.get_pre_caller();
            return_contract(Ok(Response {
                data: caller_address.to_string().as_bytes(),
            }));
        }
        "get_time" => {
            let time_stamp = deps.api.get_timestamp();
            return_contract(Ok(Response {
                data: time_stamp.to_string().as_bytes(),
            }));
        }
        "call_contract" => {
            let addr: Address = input.read_str().unwrap().into();
            let ret_input = input.read_bytes().unwrap();
            match deps.api.call_contract(&addr, &ret_input) {
                Some(res) => return_contract(Ok(Response { data: &res })),
                None => return_contract(Err("call contract error")),
            }
        }
        "destroy_contract" => {
            deps.api.destroy_contract();
            return_contract(Ok(Response {
                data: "success".as_bytes(),
            }));
        }
        "mul" => {
            let a = input.read_u128().unwrap();
            let b = input.read_u128().unwrap();
            let r = math::safe_mul(a, b);
            return_contract(Ok(Response {
                data: r.to_string().as_bytes(),
            }));
        }
        "notify" => {
            event("event type", "event msg");
            return_contract(Ok(Response {
                data: "success".as_bytes(),
            }));
        }
        _ => {
            // 返回Error
            return_contract(Err("invoke method not found"));
        }
    }
}

// subscribe 基础用法 query = "type.key = 'value'"
fn event(method: &str, msg: &str) {
    let mut event = runtime::Event::new(method);
    event.add("msg", IString(msg));
    runtime::make_dependencies().api.notify(&event);
}

fn read_db(key: &str) -> String {
    let val = runtime::make_dependencies()
        .storage
        .get(key.as_bytes())
        .unwrap();
    String::from_utf8(val).unwrap()
}

fn write_db(key: &str, value: &str) {
    runtime::make_dependencies()
        .storage
        .set(key.as_bytes(), value.as_bytes())
}

fn delete_db(key: &str) {
    runtime::make_dependencies().storage.delete(key.as_bytes())
}

fn return_contract<'a>(result: Result<Response, &'a str>) {
    runtime::make_dependencies().api.ret(result)
}
