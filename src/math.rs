use crate::runtime::panic;

pub trait OverflowingAdd<T> {
    fn try_overflowing_add(&self, _: T) -> (T, bool);
}

pub trait OverflowingSub<T> {
    fn try_overflowing_sub(&self, _: T) -> (T, bool);
}

pub trait OverflowingMul<T> {
    fn try_overflowing_mul(&self, _: T) -> (T, bool);
}

pub trait OverflowingDiv<T> {
    fn try_overflowing_div(&self, _: T) -> (T, bool);
}

macro_rules! impl_overflowing_add {
    ($t:ty) => {
        impl OverflowingAdd<$t> for $t {
            fn try_overflowing_add(&self, n: $t) -> ($t, bool) {
                self.overflowing_add(n)
            }
        }
    };
}

macro_rules! impl_overflowing_sub {
    ($t:ty) => {
        impl OverflowingSub<$t> for $t {
            fn try_overflowing_sub(&self, n: $t) -> ($t, bool) {
                self.overflowing_sub(n)
            }
        }
    };
}

macro_rules! impl_overflowing_mul {
    ($t:ty) => {
        impl OverflowingMul<$t> for $t {
            fn try_overflowing_mul(&self, n: $t) -> ($t, bool) {
                self.overflowing_mul(n)
            }
        }
    };
}

macro_rules! impl_overflowing_div {
    ($t:ty) => {
        impl OverflowingDiv<$t> for $t {
            fn try_overflowing_div(&self, n: $t) -> ($t, bool) {
                self.overflowing_div(n)
            }
        }
    };
}

macro_rules! impl_overflowing {
    ($t:ty) => {
        impl_overflowing_add!($t);
        impl_overflowing_sub!($t);
        impl_overflowing_mul!($t);
        impl_overflowing_div!($t);
    };
}

impl_overflowing!(u8);
impl_overflowing!(i8);
impl_overflowing!(u32);
impl_overflowing!(i32);
impl_overflowing!(u64);
impl_overflowing!(i64);
impl_overflowing!(u128);
impl_overflowing!(i128);

pub fn safe_add<T>(x: T, y: T) -> T
where
    T: Copy + OverflowingAdd<T>,
{
    let (result, overflow) = x.try_overflowing_add(y);
    if overflow {
        panic("add overflow");
    }
    result
}

pub fn safe_sub<T>(x: T, y: T) -> T
where
    T: Copy + OverflowingSub<T>,
{
    let (result, overflow) = x.try_overflowing_sub(y);
    if overflow {
        panic("subtract overflow");
    }
    result
}

pub fn safe_mul<T>(x: T, y: T) -> T
where
    T: Copy + OverflowingMul<T>,
{
    let (result, overflow) = x.try_overflowing_mul(y);
    if overflow {
        panic("multiply overflow");
    }
    result
}

pub fn safe_div<T>(x: T, y: T) -> T
where
    T: Copy + Default + PartialEq + OverflowingDiv<T>,
{
    if T::default().eq(&y) {
        panic("divide by zero");
    }
    let (result, overflow) = x.try_overflowing_div(y);
    if overflow {
        panic("divide overflow");
    }
    result
}
