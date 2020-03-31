use super::errors::Result;
use super::types::Response;

use crate::types;
use crate::types::Storage;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

fn test_db() -> impl types::Storage {
    types::Store {
        prefix: String::from("test"),
    }
}

pub fn init(msg_ptr: *mut c_char) -> Result<Response> {
    let mut store = test_db();
    let c_str = unsafe { CStr::from_ptr(msg_ptr) };
    {
        let msg = c_str.to_str().unwrap();
        if msg == "get" {
            let res = store.get(msg.as_bytes());
            match res {
                Some(s) => {
                    let data: String = s.iter().map(|&c| c as char).collect::<String>();
                    return Ok(Response {data: data});
                },
                None => {
                    return Ok(Response {data: String::from("err")});
                }
            }
        } else if msg == "set" {
            store.set("get".as_bytes(), "ruok?".as_bytes());
        } else if msg == "delete" {
            store.delete("get".as_bytes());
        }
    }
    Ok(Response {
        data: String::from("result from init"),
    })
}

pub fn handle(msg: *mut c_char) -> Result<Response> {
    Ok(Response {
        data: String::from("result from handle"),
    })
}

pub fn query(msg: *mut c_char) -> Result<Response> {
    Ok(Response {
        data: String::from("result from query"),
    })
}
