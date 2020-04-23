use super::errors::{contract_err, Error, Result};
use super::types::Response;
use crate::types::{Api, Extern, Storage};

pub const CONFIG_STORE: &str = "cs:";   // (cs:attributeName,attributeValue)
pub const BALANCE_STORE: &str = "bs:";  // (bs:addr,amount)
pub const ALLOWANCE_STORE: &str = "as:";// (as:owner:spender,amount)

pub const FN_APPROVE: &str = "approve";
pub const FN_TRANSFER: &str = "transfer";
pub const FN_TRANSFER_FROM: &str = "transferFrom";

pub const FN_QUERY_BALANCE: &str = "balance";
pub const FN_QUERY_ALLOWANCE: &str = "allowance";

pub const SUCCESS: &str = "Success";

//init args["tokenName","address","amount"]
pub fn init<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    method: String,
    args: Vec<String>,
) -> Result<Response, Error> {
    //check params number
    if args.len() != 3 {
        return contract_err("params error");
    }
    //set initAddress-balance
    deps.storage.set((BALANCE_STORE.to_owned() + args[1].as_str()).as_bytes() , args[2].as_bytes());
    //set tokenName
    deps.storage.set((CONFIG_STORE.to_owned() + "tokenName").as_bytes(),args[0].as_bytes());

    let a = deps.api.get_timestamp().unwrap();
    let s1 = String::from_utf8(a).expect("Found invalid UTF-8");
    let b = deps.api.get_creator().unwrap();
    let s2 = String::from_utf8(b).expect("Found invalid UTF-8");
    let c = deps.api.get_invoker().unwrap();
    let s3 = String::from_utf8(c).expect("Found invalid UTF-8");

    let res:Vec<u8> = SUCCESS.as_bytes().iter().cloned().collect();

    let rtr;
    match deps.api.transfer(s3.as_str().as_bytes(),s2.as_str().as_bytes(),args[2].as_bytes()) {
        Ok(res) =>  rtr = res,
        Err(res) => rtr = res,
    }
    let s4 = rtr.to_string();

    let s = format!("time:{}\ncreator:{}\ninvoker:{}\ntransfer:{}", s1, s2, s3, s4);
    Ok(Response {
        data: s.as_str().as_bytes().iter().cloned().collect(),
    })
}

//handle args["params"...]
pub fn handle<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    method: String,
    args: Vec<String>,
) -> Result<Response, Error> {
    //match method
    match method.as_str() {
        FN_TRANSFER => try_transfer(deps, args),
        FN_APPROVE => try_approve(deps, args),
        FN_TRANSFER_FROM => try_transfer_from(deps, args),
        _ => contract_err("method error"),
    }
}

//query args["params"...]
pub fn query<S: Storage, A: Api>(
    deps: &Extern<S, A>,
    method: String,
    args: Vec<String>,
) -> Result<Response, Error> {
    //match method
    match method.as_str() {
        FN_QUERY_BALANCE => {
            let res = read_balance(deps, args)?;
            let res:Vec<u8> = res.to_string().as_str().as_bytes().iter().cloned().collect();
            Ok(Response {
                data: res,
            })
        },
        FN_QUERY_ALLOWANCE => {
            let res = read_allowance(deps, args)?;
            let res:Vec<u8> = res.to_string().as_str().as_bytes().iter().cloned().collect();
            Ok(Response {
                data: res,
            })
        },
        _ => contract_err("method error"),
    }
}

//transfer params["from","to","amount"]
//由from向to发起，额度为amount的普通转账
fn try_transfer<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    params: Vec<String>,
) -> Result<Response, Error> {
    if params.len() != 3 {
        return contract_err("params number error");
    } else {
        perform_transfer(deps, params[0].as_str(),params[1].as_str(),params[2].as_str())
    }
}

//approve params["owner","spender","amount"]
//owner向spender的授权操作，授权额度为amount
fn try_approve<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    params: Vec<String>,
) -> Result<Response, Error> {
    if params.len() != 3 {
        return contract_err("params number error");
    }
    //set
    deps.storage.set((ALLOWANCE_STORE.to_owned() + params[0].as_str() + ":" + params[1].as_str()).as_bytes(),params[2].as_str().as_bytes());

    let res:Vec<u8> = SUCCESS.as_bytes().iter().cloned().collect();
    Ok(Response {
        data: res,
    })
}

//transfer_from params["owner","from","to","amount"]
//由from向to发起，额度为amount的，在owner处的，已授权转账
fn try_transfer_from<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    params: Vec<String>,
) -> Result<Response, Error> {
    if params.len() != 4 {
        return contract_err("params number error");
    }
    let mut allowance = read_allowance(deps, vec![params[0].clone(),params[1].clone()])?;
    let amount = parse_u128(params[3].as_str())?;
    if allowance < amount {
        return contract_err("Insufficient allowance")
    } else {
        //allowance enough
        allowance -= amount;
        deps.storage.set((ALLOWANCE_STORE.to_owned() + params[0].as_str() + ":" + params[1].as_str()).as_bytes(),allowance.to_string().as_str().as_bytes());
        return perform_transfer(deps,params[0].as_str(),params[2].as_str(),params[3].as_str());
    }
}

//read_balance params["addr"]
//查询addr余额
fn read_balance<S: Storage, A: Api>(
    deps: &Extern<S, A>,
    params: Vec<String>,
) -> Result<u128> {
    if params.len() != 1 {
        return contract_err("params number error");
    }
    return read_u128(deps,params);
}

//read_allowance params["owner","addr"]
//查询addr在owner处授权额度
fn read_allowance<S: Storage, A: Api>(
    deps: &Extern<S, A>,
    params: Vec<String>,
) -> Result<u128> {
    if params.len() != 2 {
        return contract_err("params number error");
    }
    return read_u128(deps,params);
}

//实际转账操作
fn perform_transfer<S: Storage, A: Api>(
    deps: &mut Extern<S, A>,
    from: &str,
    to: &str,
    amount: &str,
) -> Result<Response, Error> {
    let mut from_balance = read_u128(deps,vec![String::from(from)])?;
    let amount_raw = parse_u128(amount)?;
    //check
    if from_balance < amount_raw {
        return contract_err("Insufficient funds");
    }
    //send
    from_balance -= amount_raw;
    let mut to_balance = read_u128(deps,vec![String::from(to)])?;
    let amount_raw2 = parse_u128(amount)?;
    to_balance += amount_raw2;
    //set
    deps.storage.set((BALANCE_STORE.to_owned() + from).as_bytes() , from_balance.to_string().as_str().as_bytes());
    deps.storage.set((BALANCE_STORE.to_owned() + to).as_bytes() , to_balance.to_string().as_str().as_bytes());

    let res:Vec<u8> = SUCCESS.as_bytes().iter().cloned().collect();
    Ok(Response {
        data: res,
    })
}

pub fn read_u128<S: Storage, A: Api>(
    deps: &Extern<S, A>,
    addr: Vec<String>,
) -> Result<u128> {
    let key:String;
    if addr.len() == 1{
        key = BALANCE_STORE.to_owned() + addr[0].as_str();
    } else {
        key = ALLOWANCE_STORE.to_owned() + addr[0].as_str() + ":" + addr[1].as_str();
    }

    let addr_balance_op = deps.storage.get(key.as_bytes());
    match addr_balance_op {
        Some(addr_balance_bytes) =>{
            let addr_balance_string = String::from_utf8(addr_balance_bytes).expect("Found invalid UTF-8");
            let addr_balance: u128 = addr_balance_string.parse().unwrap();
            return Ok(addr_balance);
        }
        None =>{
            Ok(0u128)
        }
    }
}

pub fn parse_u128(source: &str) -> Result<u128> {
    match source.parse::<u128>() {
        Ok(value) => Ok(value),
        Err(_) => contract_err("Error while parsing string to u128"),
    }
}