use super::errors::{Error, Result};
use super::types::Response;
use serde::{Deserialize, Serialize};

use crate::types;
use crate::types::Storage;

use std::ffi::CStr;
use std::os::raw::c_char;

fn test_db() -> impl types::Storage {
    types::Store {
        prefix: String::from("test"),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InitMsg {
    pub count: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HandleMsg {
    Increment {},
    Reset { count: i32 },
}

pub fn init(msg_ptr: *mut c_char) -> Result<Response, Error> {
    let mut store = test_db();
    let c_str = unsafe { CStr::from_ptr(msg_ptr) };
    {
        let msg = c_str.to_str().unwrap();
        let init_msg: InitMsg = serde_json::from_slice(msg.as_bytes()).unwrap();
        // let init_msg = InitMsg{count: 5};
        store.set("count".as_bytes(), init_msg.count.to_string().as_bytes());
    }
    Ok(Response {
        messages: vec![],
        log: vec![],
        data: c_str.to_str().unwrap()
            .as_bytes()
            .iter()
            .map(|&u| u as u8)
            .collect::<Vec<u8>>(),
    })
}

pub fn handle(msg_ptr: *mut c_char) -> Result<Response> {
    let mut store = test_db();
    let c_str = unsafe { CStr::from_ptr(msg_ptr) };
    {
        let msg = c_str.to_str().unwrap();
        let handle_msg: HandleMsg = serde_json::from_slice(msg.as_bytes()).unwrap();
        match handle_msg {
            HandleMsg::Increment {} => {
                let res = store.get("count".as_bytes());
                match res {
                    Some(s) => {
                        let data: String = s.iter().map(|&c| c as char).collect::<String>();
                        let count: i32 = data.parse().unwrap();
                        store.set("count".as_bytes(), (count + 1).to_string().as_bytes());
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
                        return Err(Error::NotFound { kind: "db error" });
                    }
                }
            }
            HandleMsg::Reset { count } => {
                store.set("count".as_bytes(), count.to_string().as_bytes());
            }
        }
    }
    Ok(Response {
        messages: vec![],
        log: vec![],
        data: "result from handle"
            .as_bytes()
            .iter()
            .map(|&u| u as u8)
            .collect::<Vec<u8>>(),
    })
}

pub fn query(msg_ptr: *mut c_char) -> Result<Response> {
    Ok(Response {
        messages: vec![],
        log: vec![],
        data: "result from query"
            .as_bytes()
            .iter()
            .map(|&u| u as u8)
            .collect::<Vec<u8>>(),
    })
}
