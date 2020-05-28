pub mod errors;
pub mod runtime;
pub mod types;

use std::collections::HashMap;

#[no_mangle]
pub fn invoke() {
    let deps = runtime::make_dependencies();
    let param = deps.api.input();
    let mut msg = param.args;
    msg.push(param.method);
    let mut ret = String::from("msg:");
    for i in 0..msg.len() {
        ret += &msg[i];
    }
    let mut map = HashMap::new();
    map.insert(
        String::from("i guess"),
        runtime::ItemValue::String(String::from("time machine")),
    );
    map.insert(String::from("so answer"), runtime::ItemValue::Int64(765));
    runtime::notify(&runtime::Event::new(String::from("notiii"), map));
    runtime::ret(ret.as_bytes())
}
