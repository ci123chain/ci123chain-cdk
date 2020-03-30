use serde::{Serialize, Deserialize};
use serde_json;
use std::fmt::Display;
use std::os::raw::{c_char, c_void};
use crate::types::{Response, ContractResult};
use crate::errors::{Error, ParseErr, SerializeErr};
use std::ffi::{CStr, CString};


#[derive(Serialize, Deserialize)]
struct Params {
    Args: Vec<String>,
    Method: String
}


pub fn make_res_c_string(res: Response) -> *mut c_char {
    let output = serde_json::to_vec(&ContractResult::Ok(res)).unwrap();
    unsafe { CString::from_vec_unchecked(output) }.into_raw()
}

pub fn make_err_c_string<T: Display>(error: T) -> *mut c_char {
    let output = serde_json::to_vec(&ContractResult::Err(error.to_string())).unwrap()
    unsafe { CString::from_vec_unchecked(output) }.into_raw()
}

