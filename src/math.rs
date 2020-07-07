use crate::runtime::panic;

pub fn safe_add_u128(a: u128, b: u128) -> u128 {
    let (result, overflow) = a.overflowing_add(b);
    if overflow {
        panic("add overflow");
    }
    result
}

pub fn safe_sub_u128(a: u128, b: u128) -> u128 {
    let (result, overflow) = a.overflowing_sub(b);
    if overflow {
        panic("subtract overflow");
    }
    result
}

pub fn safe_mul_u128(a: u128, b: u128) -> u128 {
    let (result, overflow) = a.overflowing_mul(b);
    if overflow {
        panic("multiply overflow");
    }
    result
}

pub fn safe_div_u128(a: u128, b: u128) -> u128 {
    if b == 0 {
        panic("divide by zero");
    }
    a / b
}

pub fn safe_add_i128(a: i128, b: i128) -> i128 {
    let (result, overflow) = a.overflowing_add(b);
    if overflow {
        panic("add overflow");
    }
    result
}

pub fn safe_sub_i128(a: i128, b: i128) -> i128 {
    let (result, overflow) = a.overflowing_sub(b);
    if overflow {
        panic("subtract overflow");
    }
    result
}

pub fn safe_mul_i128(a: i128, b: i128) -> i128 {
    let (result, overflow) = a.overflowing_mul(b);
    if overflow {
        panic("multiply overflow");
    }
    result
}

pub fn safe_div_i128(a: i128, b: i128) -> i128 {
    if b == 0 {
        panic("divide by zero");
    }
    a / b
}
