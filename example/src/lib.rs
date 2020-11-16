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
    let method = input.read().unwrap();
    match method {
        "read_db" => {
            let key = input.read().unwrap();
            return_contract(Ok(Response {
                data: read_db(key).as_bytes(),
            }));
        }
        "write_db" => {
            let (key, value) = (input.read().unwrap(), input.read().unwrap());
            write_db(key, value);
            return_contract(Ok(Response {
                data: "success".as_bytes(),
            }));
        }
        "delete_db" => {
            let key = input.read().unwrap();
            delete_db(key);
            return_contract(Ok(Response {
                data: "success".as_bytes(),
            }));
        }
        "send" => {
            let addr: Address = input.read::<&str>().unwrap().into();
            let amount = input.read().unwrap();
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
        "get_block_header" => {
            let block = deps.api.get_block_header();
            return_contract(Ok(Response {
                data: format!("height: {}, timestamp: {}", block.height, block.timestamp)
                    .as_bytes(),
            }));
        }
        "call_contract" => {
            let addr: Address = input.read::<&str>().unwrap().into();
            let ret_input = input.read().unwrap();
            match deps.api.call_contract(&addr, ret_input) {
                Some(res) => return_contract(Ok(Response { data: &res })),
                None => return_contract(Err("call contract error")),
            }
        }
        // "destroy_contract" => {
        //     deps.api.destroy_contract();
        //     return_contract(Ok(Response {
        //         data: "success".as_bytes(),
        //     }));
        // }
        "mul" => {
            let (a, b) = input.read().unwrap();
            let r: u128 = math::safe_mul(a, b);
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
        "get_validator_power" => {
            let validators = [
                &Address::default(),
                &Address::default(),
                &Address::default(),
            ];
            let power = deps.api.get_validator_power(&validators);
            return_contract(Ok(Response {
                data: format!("{:?}", power).as_bytes(),
            }));
        }
        "total_power" => {
            let total_power = deps.api.total_power();
            return_contract(Ok(Response {
                data: total_power.to_string().as_bytes(),
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
