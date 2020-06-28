extern crate c123chain_cdk as cdk;

use cdk::errors;
use cdk::runtime;
use cdk::runtime::ItemValue::String as IString;
use cdk::types::{Address, Param, Response};

#[no_mangle]
pub fn invoke() {
    let deps = runtime::make_dependencies();
    let param = deps.api.input();
    match param.method.as_str() {
        "read_db" => {
            event(param.method, read_db(param.args[0].as_str()));
        }
        "write_db" => {
            write_db(param.args[0].as_str(), param.args[1].as_str());
            event(param.method, param.args[0].clone());
        }
        "delete_db" => {
            delete_db(param.args[0].as_str());
            event(param.method, param.args[0].clone());
        }
        "send" => {
            let mut addr = [0; 20];
            for i in 0..20 {
                addr[i] = param.args[0].as_bytes()[i];
            }
            let res = deps
                .api
                .send(&Address::new(&addr), param.args[1].parse().unwrap());
            event(param.method, res.unwrap().to_string());
        }
        "get_creator" => {
            let creator = deps.api.get_creator();
            event(param.method, creator.to_hex_string());
        }
        "get_invoker" => {
            let invoker = deps.api.get_invoker();
            event(param.method, invoker.to_hex_string());
        }
        "get_time" => {
            let time_stamp = deps.api.get_timestamp();
            event(param.method, time_stamp.unwrap().to_string());
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
            event(param.method, res.to_string());
        }
        _ => {
            // 返回指定类型的Error
            deps.api.ret(Err(errors::Error::NotFound {
                kind: String::from("invoke method"),
            }));
        }
    }
    deps.api.ret(Ok(Response {
        data: "success".as_bytes().iter().cloned().collect(),
    }))
}

// subscribe 基础用法 query = "type.key = 'value'"
fn event(method: String, msg: String) {
    let mut event = runtime::Event::new(method);
    event.add("msg".to_string(), IString(msg));
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
