use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Params {
    Args: Vec<String>,
    Method: String
}


use crate::types::{Response, ContractResult};
use crate::serde::{from_slice, to_vec};
use crate::errors::{Error, ParseErr, SerializeErr};

pub fn make_res_c_string(res: Response) -> *mut c_char {
    to_vec(&ContractResult::Ok(res)).context(SerializeErr{
        kind: "ContractResult",
    })
}