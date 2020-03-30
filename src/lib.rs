pub mod contract;
pub mod types;
pub mod errors;
pub mod export;

mod wasm {
    use super::contract;
    use super::export;

    use std::ffi::{CStr, CString};
    use std::mem;
    use std::os::raw::{c_char, c_void};


    pub extern fn init (subject: *mut c_char) -> *mut c_char {
        match contract::init() {
            Ok(res) => export::make_res_c_string(res),
            Err(err) => export::make_err_c_string(err),
        }
    }

    pub extern fn handle(subject: *mut c_char) -> *mut c_char {
        match contract::handle() {
            Ok(res) => export::make_res_c_string(res),
            Err(err) => export::make_err_c_string(err),
        }
    }

    pub extern fn query(subject: *mut c_char) -> *mut c_char {
        match contract::query() {
            Ok(res) => export::make_res_c_string(res),
            Err(err) => export::make_err_c_string(err),
        }
    }

    #[no_mangle]
    pub extern fn allocate(size: usize) -> *mut c_void {
        let mut buffer = Vec::with_capacity(size);
        let pointer = buffer.as_mut_ptr();
        mem::forget(buffer);
        pointer as *mut c_void
    }

    #[no_mangle]
    pub extern fn deallocate(pointer: *mut c_void, capacity: usize) {
        unsafe {
            let _ = Vec::from_raw_parts(pointer, 0, capacity);
        }
    }

    #[no_mangle]
    pub extern fn greet(subject: *mut c_char) -> *mut c_char {
        let subject = unsafe { CStr::from_ptr(subject).to_bytes().to_vec() };
        let mut output = b"Hello, ".to_vec();
        output.extend(&subject);
        output.extend(&[b'!']);

        unsafe { CString::from_vec_unchecked(output) }.into_raw()
    }


}