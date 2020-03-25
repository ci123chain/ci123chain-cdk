pub mod contract;
pub mod types;

mod wasm {
    use std::ffi::c_void;
    use super::contract;

    pub extern "C" fn init (params_ptr: *mut c_void) -> *mut c_void {
        contract::init()
    }

    pub extern "C" fn handle(params_ptr: *mut c_void) -> *mut c_void {
        contract::handle()
    }

    pub extern "C" fn query(params_ptr: *mut c_void) -> *mut c_void {
        contract::query()
    }
}