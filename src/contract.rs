use super::errors::{Error, Result};
use super::types::Response;
use serde::{Deserialize, Serialize};

use crate::types::{Api, Extern, Storage};

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QueryMsg;

pub fn init<S: Storage, A: Api>(deps: &mut Extern<S, A>, msg: InitMsg) -> Result<Response, Error> {
    deps.storage
        .set("count".as_bytes(), msg.count.to_string().as_bytes());
    Ok(Response {
        messages: vec![],
        log: vec![],
        data: serde_json::to_vec(&msg).unwrap(),
        // data: msg.to_str().unwrap()
        //     .as_bytes()
        //     .iter()
        //     .map(|&u| u as u8)
        //     .collect::<Vec<u8>>(),
    })
}

pub fn handle<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    msg: HandleMsg,
) -> Result<Response, Error> {
    match msg {
        HandleMsg::Increment {} => {
            let res = deps.storage.get("count".as_bytes());
            match res {
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
        }
        HandleMsg::Reset { count } => {
            deps.storage
                .set("count".as_bytes(), count.to_string().as_bytes());
        }
    }
    Ok(Response {
        messages: vec![],
        log: vec![],
        data: serde_json::to_vec(&msg).unwrap(),
    })
}

pub fn query<S: Storage, A: Api>(_: &Extern<S, A>, _: QueryMsg) -> Result<Response, Error> {
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
