pub mod codec;
pub mod math;
pub mod runtime;
pub mod types;
pub mod util;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate alloc;
pub mod prelude {
    pub use alloc::boxed::Box;
    pub use alloc::str;
    pub use alloc::string::{self, String, ToString};
    pub use alloc::vec::Vec;
    pub use alloc::{format, vec};
    pub use core::cmp;
    pub use core::convert::AsMut;
    pub use core::fmt::Write;
    pub use core::prelude::v1::*;
    pub use std::cell::Cell;
    pub use std::panic;
}
