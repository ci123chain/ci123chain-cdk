use crate::errors::Error;
use crate::types::{consume_region, ContractResult, Extern, ExternalApi, Response, Store};
use serde::de;
use serde::{Deserialize, Serialize};
use serde_json;
use std::ffi::{CStr, CString};
use std::fmt::Display;
use std::os::raw::{c_char, c_void};

#[derive(Serialize, Deserialize)]
struct Params {
    args: Vec<String>,
    method: String,
}

pub fn do_init<T: de::DeserializeOwned>(
    init_fn: &dyn Fn(&mut Extern<Store, ExternalApi>, T) -> Result<Response, Error>,
    msg_ptr: *mut c_char,
) -> *mut c_char {
    match _do_init(init_fn, msg_ptr) {
        Ok(res) => make_res_c_string(res),
        Err(err) => make_err_c_string(err),
    }
}

pub fn do_handle<T: de::DeserializeOwned>(
    handle_fn: &dyn Fn(&mut Extern<Store, ExternalApi>, T) -> Result<Response, Error>,
    msg_ptr: *mut c_void,
) -> *mut c_char {
    match _do_handle(handle_fn, msg_ptr) {
        Ok(res) => make_res_c_string(res),
        Err(err) => make_err_c_string(err),
    }
}

pub fn do_query<T: de::DeserializeOwned>(
    query_fn: &dyn Fn(&Extern<Store, ExternalApi>, T) -> Result<Response, Error>,
    msg_ptr: *mut c_void,
) -> *mut c_char {
    match _do_query(query_fn, msg_ptr) {
        Ok(res) => make_res_c_string(res),
        Err(err) => make_err_c_string(err),
    }
}

fn _do_init<T: de::DeserializeOwned>(
    init_fn: &dyn Fn(&mut Extern<Store, ExternalApi>, T) -> Result<Response, Error>,
    msg_ptr: *mut c_char,
) -> Result<Response, Error> {
    let c_str = unsafe { CStr::from_ptr(msg_ptr) };
    let msg: Vec<u8> = c_str
        .to_str()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|&u| u as u8)
        .collect::<Vec<u8>>();
    let msg: T = serde_json::from_slice(&msg)?;
    let mut deps = make_dependencies();
    init_fn(&mut deps, msg)
}

fn _do_handle<T: de::DeserializeOwned>(
    handle_fn: &dyn Fn(&mut Extern<Store, ExternalApi>, T) -> Result<Response, Error>,
    msg_ptr: *mut c_void,
) -> Result<Response, Error> {
    let msg: Vec<u8> = unsafe { consume_region(msg_ptr)? };
    let msg: T = serde_json::from_slice(&msg)?;
    let mut deps = make_dependencies();
    handle_fn(&mut deps, msg)
}

fn _do_query<T: de::DeserializeOwned>(
    query_fn: &dyn Fn(&Extern<Store, ExternalApi>, T) -> Result<Response, Error>,
    msg_ptr: *mut c_void,
) -> Result<Response, Error> {
    let msg: Vec<u8> = unsafe { consume_region(msg_ptr)? };
    let msg: T = serde_json::from_slice(&msg)?;
    let deps = make_dependencies();
    query_fn(&deps, msg)
}

pub fn make_res_c_string(res: Response) -> *mut c_char {
    let output = serde_json::to_vec(&ContractResult::Ok(res)).unwrap();
    unsafe { CString::from_vec_unchecked(output) }.into_raw()
}

pub fn make_err_c_string<T: Display>(error: T) -> *mut c_char {
    let output = serde_json::to_vec(&ContractResult::Err(error.to_string())).unwrap();
    unsafe { CString::from_vec_unchecked(output) }.into_raw()
}

fn make_dependencies() -> Extern<Store, ExternalApi> {
    Extern {
        storage: Store::new(),
        api: ExternalApi::new(),
    }
}
