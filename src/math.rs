use crate::runtime::panic;

pub fn safe_add<T: Copy>(a: impl SafeAdd<T>, b: T) -> T {
    match a.checked_add(b) {
        Some(v) => v,
        None => {
            panic("add overflow");
            b
        }
    }
}

pub fn safe_sub<T: Copy>(a: impl SafeSub<T>, b: T) -> T {
    match a.checked_sub(b) {
        Some(v) => v,
        None => {
            panic("sub overflow");
            b
        }
    }
}

pub fn safe_mul<T: Copy>(a: impl SafeMul<T>, b: T) -> T {
    match a.checked_mul(b) {
        Some(v) => v,
        None => {
            panic("mul overflow");
            b
        }
    }
}

pub fn safe_div<T: Copy>(a: impl SafeDiv<T>, b: T) -> T {
    match a.checked_div(b) {
        Some(v) => v,
        None => {
            panic("rhs is zero or div overflow");
            b
        }
    }
}

pub trait SafeAdd<T> {
    fn checked_add(&self, v: T) -> Option<T>;
}

pub trait SafeSub<T> {
    fn checked_sub(&self, v: T) -> Option<T>;
}

pub trait SafeMul<T> {
    fn checked_mul(&self, v: T) -> Option<T>;
}

pub trait SafeDiv<T> {
    fn checked_div(&self, v: T) -> Option<T>;
}
