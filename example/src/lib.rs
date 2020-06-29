extern crate c123chain_cdk as cdk;

use cdk::errors;
use cdk::runtime;
use cdk::runtime::ItemValue::Str as IString;
use cdk::types::{Address, Param, Response};

#[no_mangle]
pub fn invoke() {
    let deps = runtime::make_dependencies();
    let param = deps.api.input();
    match param.method.as_str() {
        "read_db" => {
            return_contract(Ok(Response {
                data: read_db(&param.args[0]).into_bytes(),
            }));
        }
        "write_db" => {
            write_db(param.args[0].as_str(), param.args[1].as_str());
            return_contract(Ok(Response {
                data: "success".as_bytes().iter().cloned().collect(),
            }));
        }
        "delete_db" => {
            delete_db(param.args[0].as_str());
            return_contract(Ok(Response {
                data: "success".as_bytes().iter().cloned().collect(),
            }));
        }
        "send" => {
            let mut addr = [0; 20];
            for i in 0..20 {
                addr[i] = param.args[0].as_bytes()[i];
            }
            let res = deps
                .api
                .send(&Address::new(&addr), param.args[1].parse().unwrap());
            return_contract(Ok(Response {
                data: res.unwrap().to_string().into_bytes(),
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
                data: time_stamp.unwrap().to_string().into_bytes(),
            }));
        }
        "call_contract" => {
            let mut addr = [0; 20];
            for i in 0..20 {
                addr[i] = param.args[0].as_bytes()[i];
            }
            let ret_param = Param {
                method: param.args[1].clone(),
                args: vec![param.args[2].clone()],
            };
            let res = deps.api.call_contract(&Address::new(&addr), &ret_param);
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
            // 返回指定类型的Error
            return_contract(Err(errors::Error("invoke method not found".to_string())));
        }
    }
}

// subscribe 基础用法 query = "type.key = 'value'"
fn event(method: &'static str, msg: &'static str) {
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

fn return_contract(result: Result<Response, errors::Error>) {
    runtime::make_dependencies().api.ret(result)
}
