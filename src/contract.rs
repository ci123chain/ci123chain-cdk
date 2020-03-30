use super::errors::Result;
use super::types::Response;


pub fn init() -> Result<Response> {
    Ok(Response{
        data: String::from("result from init"),
    })
}

pub fn handle() -> Result<Response> {
    Ok(Response{
        data: String::from("result from handle"),
    })
}

pub fn query() -> Result<Response> {
    Ok(Response{
        data: String::from("result from query"),
    })
}