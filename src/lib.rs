pub mod contract;
pub mod errors;
pub mod export;
pub mod types;

pub mod wasm {
    use super::contract;
    use super::export;
    use super::types;

    use std::os::raw::{c_char, c_void};

    #[no_mangle]
    pub extern "C" fn init(msg: *mut c_char) -> *mut c_char {
        export::do_init(&contract::init::<types::Store, types::ExternalApi>, msg)
    }

    #[no_mangle]
    pub extern "C" fn handle(msg: *mut c_char) -> *mut c_char {
        export::do_handle(&contract::handle::<types::Store, types::ExternalApi>, msg)
    }

    #[no_mangle]
    pub extern "C" fn query(msg: *mut c_char) -> *mut c_char {
        export::do_query(&contract::query::<types::Store, types::ExternalApi>, msg)
    }

    #[no_mangle]
    pub extern "C" fn allocate(size: usize) -> *mut c_void {
        types::allocate(size)
    }

    #[no_mangle]
    pub extern "C" fn deallocate(pointer: *mut c_void, capacity: usize) {
        types::deallocate(pointer, capacity)
    }
}
