extern crate c123chain_cdk as cdk;

use cdk::runtime;
use cdk::runtime::ItemValue::Str as IString;
use cdk::types::Response;

#[no_mangle]
pub fn invoke() {
    let deps = runtime::make_dependencies();
    let input = deps.api.input();
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
            let addr = input.read_address().unwrap();
            let amount = input.read_u64().unwrap();
            let res = deps.api.send(&addr, amount);
            return_contract(Ok(Response {
                data: res.to_string().as_bytes(),
            }));
        }
        "get_creator" => {
            let creator = deps.api.get_creator();
            return_contract(Ok(Response {
                data: creator.to_hex_string().as_bytes(),
            }));
        }
        "get_invoker" => {
            let invoker = deps.api.get_invoker();
            return_contract(Ok(Response {
                data: invoker.to_hex_string().as_bytes(),
            }));
        }
        "get_time" => {
            let time_stamp = deps.api.get_timestamp();
            return_contract(Ok(Response {
                data: time_stamp.to_string().as_bytes(),
            }));
        }
        "call_contract" => {
            let addr = input.read_address().unwrap();
            let input_size = input.read_usize().unwrap();
            let ret_input = input.read_bytes(input_size).unwrap();
            match deps.api.call_contract(&addr, &ret_input) {
                Some(res) => return_contract(Ok(Response { data: &res })),
                None => return_contract(Err("call contract error")),
            }
        }
        "destroy_contract" => {
            let addr = input.read_address().unwrap();
            deps.api.destroy_contract(&addr);
        }
        "migrate_contract" => {
            let code_size = input.read_usize().unwrap();
            let code = input.read_bytes(code_size).unwrap();
            let name = input.read_str().unwrap();
            let version = input.read_str().unwrap();
            let author = input.read_str().unwrap();
            let email = input.read_str().unwrap();
            let desc = input.read_str().unwrap();
            match deps
                .api
                .migrate_contract(code, name, version, author, email, desc)
            {
                Some(addr) => return_contract(Ok(Response {
                    data: addr.into_slice(),
                })),
                None => return_contract(Err("migrate contract error")),
            }
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
