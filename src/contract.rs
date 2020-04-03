use super::errors::{Error, Result};
use super::types::Response;

use crate::types::{Api, Extern, Storage};

pub fn init<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    method: String,
    args: Vec<String>,
) -> Result<Response, Error> {
    deps.storage.set("count".as_bytes(), args[0].as_bytes());

    let mut args = args;
    args.push(method);
    Ok(Response {
        messages: vec![],
        log: vec![],
        data: serde_json::to_vec(&args).unwrap(),
    })
}

pub fn handle<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    method: String,
    args: Vec<String>,
) -> Result<Response, Error> {
    if method == String::from("inc") {
        let value = deps.storage.get("count".as_bytes());
        match value {
            Some(s) => {
                let data: String = s.iter().map(|&c| c as char).collect::<String>();
                let count: i32 = data.parse().unwrap();
                deps.storage
                    .set("count".as_bytes(), (count + 1).to_string().as_bytes());
                return Ok(Response {
                    messages: vec![],
                    log: vec![],
                    data: data
                        .as_bytes()
                        .iter()
                        .map(|&u| u as u8)
                        .collect::<Vec<u8>>(),
                });
            }
            None => {
                return Err(Error::NotFound {
                    kind: "db error".to_string(),
                });
            }
        }
    } else if method == String::from("reset") {
        deps.storage.set("count".as_bytes(), args[0].as_bytes());
    }

    let mut args = args;
    args.push(method);
    Ok(Response {
        messages: vec![],
        log: vec![],
        data: serde_json::to_vec(&args).unwrap(),
    })
}

pub fn query<S: Storage, A: Api>(
    _: &Extern<S, A>,
    method: String,
    args: Vec<String>,
) -> Result<Response, Error> {
    let mut args = args;
    args.push(method);
    args.push(String::from("query"));
    Ok(Response {
        messages: vec![],
        log: vec![],
        data: serde_json::to_vec(&args).unwrap(),
    })
}
