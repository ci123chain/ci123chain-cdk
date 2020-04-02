pub mod contract;
pub mod types;
pub mod errors;
pub mod export;

pub mod wasm {
    use super::contract;
    use super::export;
    use super::types;

    use std::os::raw::{c_char, c_void};

    #[no_mangle]
    pub extern fn init (msg: *mut c_char) -> *mut c_char {
        export::do_init(
            &contract::init::<types::Store, types::ExternalApi>,
            msg,
        )
    }

    #[no_mangle]
    pub extern fn handle(msg: *mut c_char) -> *mut c_char {
        match contract::handle(msg) {
            Ok(res) => export::make_res_c_string(res),
            Err(err) => export::make_err_c_string(err),
        }
    }

    #[no_mangle]
    pub extern fn query(msg: *mut c_char) -> *mut c_char {
        match contract::query(msg) {
            Ok(res) => export::make_res_c_string(res),
            Err(err) => export::make_err_c_string(err),
        }
    }

    #[no_mangle]
    pub extern fn allocate(size: usize) -> *mut c_void {
        types::allocate(size)
    }

    #[no_mangle]
    pub extern fn deallocate(pointer: *mut c_void, capacity: usize) {
        types::deallocate(pointer, capacity)
    }

}