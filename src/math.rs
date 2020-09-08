use crate::runtime::panic;

pub trait SafeAdd<T> {
    fn safe_add(&self, _: T) -> T;
}

pub trait SafeSub<T> {
    fn safe_sub(&self, _: T) -> T;
}

pub trait SafeMul<T> {
    fn safe_mul(&self, _: T) -> T;
}

pub trait SafeDiv<T> {
    fn safe_div(&self, _: T) -> T;
}

macro_rules! impl_safe_add {
    ($t:ty) => {
        impl SafeAdd<$t> for $t
        where
            $t: Default + PartialEq,
        {
            fn safe_add(&self, n: $t) -> $t {
                let (result, overflow) = self.overflowing_add(n);
                if overflow {
                    panic("add overflow");
                }
                result
            }
        }
    };
}

macro_rules! impl_safe_sub {
    ($t:ty) => {
        impl SafeSub<$t> for $t
        where
            $t: Default + PartialEq,
        {
            fn safe_sub(&self, n: $t) -> $t {
                let (result, overflow) = self.overflowing_sub(n);
                if overflow {
                    panic("subtract overflow");
                }
                result
            }
        }
    };
}

macro_rules! impl_safe_mul {
    ($t:ty) => {
        impl SafeMul<$t> for $t
        where
            $t: Default + PartialEq,
        {
            fn safe_mul(&self, n: $t) -> $t {
                let (result, overflow) = self.overflowing_mul(n);
                if overflow {
                    panic("multiply overflow");
                }
                result
            }
        }
    };
}

macro_rules! impl_safe_div {
    ($t:ty) => {
        impl SafeDiv<$t> for $t
        where
            $t: Default + PartialEq,
        {
            fn safe_div(&self, n: $t) -> $t {
                if <$t>::default().eq(&n) {
                    panic("divide by zero");
                }
                let (result, overflow) = self.overflowing_div(n);
                if overflow {
                    panic("divide overflow");
                }
                result
            }
        }
    };
}

macro_rules! impl_safe {
    ($t:ty) => {
        impl_safe_add!($t);
        impl_safe_sub!($t);
        impl_safe_mul!($t);
        impl_safe_div!($t);
    };
}

impl_safe!(u8);
impl_safe!(i8);
impl_safe!(u32);
impl_safe!(i32);
impl_safe!(u64);
impl_safe!(i64);
impl_safe!(u128);
impl_safe!(i128);

pub fn safe_add<T>(x: T, y: T) -> T
where
    T: Copy + SafeAdd<T>,
{
    x.safe_add(y)
}

pub fn safe_sub<T>(x: T, y: T) -> T
where
    T: Copy + SafeSub<T>,
{
    x.safe_sub(y)
}

pub fn safe_mul<T>(x: T, y: T) -> T
where
    T: Copy + SafeMul<T>,
{
    x.safe_mul(y)
}

pub fn safe_div<T>(x: T, y: T) -> T
where
    T: Copy + SafeDiv<T>,
{
    x.safe_div(y)
}
