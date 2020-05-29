pub mod errors;
pub mod runtime;
pub mod types;
use runtime::ItemValue::String as IString;
// use runtime::ItemValue::Int64 as IInt64;

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
    
}

fn event() {
    let map = hashmap!["A".to_string() => IString(String::from("time machine"))];
    runtime::notify(&runtime::Event::new("notify_name".to_string(), map));
    runtime::ret("success".as_bytes())
}

fn get_address() {

}

fn time_stamp() -> u64 {
    let deps = runtime::make_dependencies();
    deps.api.get_timestamp().unwrap()
}

fn read_db(key: String) -> String {
    let val = runtime::make_dependencies().storage.get(key.as_bytes()).unwrap();
    String::from_utf8(val).unwrap()
}

fn write_db(key: String, value: String) {
    runtime::make_dependencies().storage.set(key.as_bytes(), value.as_bytes())
}