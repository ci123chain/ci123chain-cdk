extern crate c123chain_cdk as cdk;

use cdk::runtime;
use cdk::runtime::ItemValue::Str as IString;
use cdk::types::Response;

#[no_mangle]
pub fn invoke() {
    let deps = runtime::make_dependencies();
    let mut input = deps.api.input();
    let method = input.read_str().unwrap();
    match method {
        "read_db" => {
            let key = input.read_str().unwrap();
            return_contract(Ok(Response {
                data: read_db(key).into_bytes(),
            }));
        }
        "write_db" => {
            let key = input.read_str().unwrap().to_owned();
            let value = input.read_str().unwrap();
            write_db(&key, value);
            return_contract(Ok(Response {
                data: "success".as_bytes().iter().cloned().collect(),
            }));
        }
        "delete_db" => {
            let key = input.read_str().unwrap();
            delete_db(key);
            return_contract(Ok(Response {
                data: "success".as_bytes().iter().cloned().collect(),
            }));
        }
        "send" => {
            let addr = input.read_address().unwrap();
            let amount = input.read_u64().unwrap();
            let res = deps.api.send(&addr, amount);
            return_contract(Ok(Response {
                data: res.to_string().into_bytes(),
            }));
        }
        "get_creator" => {
            let creator = deps.api.get_creator();
            return_contract(Ok(Response {
                data: creator.to_hex_string().into_bytes(),
            }));
        }
        "get_invoker" => {
            let invoker = deps.api.get_invoker();
            return_contract(Ok(Response {
                data: invoker.to_hex_string().into_bytes(),
            }));
        }
        "get_time" => {
            let time_stamp = deps.api.get_timestamp();
            return_contract(Ok(Response {
                data: time_stamp.to_string().into_bytes(),
            }));
        }
        "call_contract" => {
            let addr = input.read_address().unwrap();
            let input_size = input.read_usize().unwrap();
            let ret_input = input.read_bytes(input_size).unwrap();
            let res = deps.api.call_contract(&addr, &ret_input);
            return_contract(Ok(Response {
                data: res.to_string().into_bytes(),
            }));
        }
        "notify" => {
            event("event type", "event msg");
            return_contract(Ok(Response {
                data: "success".as_bytes().iter().cloned().collect(),
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
