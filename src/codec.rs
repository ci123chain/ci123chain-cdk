use crate::types::Address;

use crate::prelude::{String, Vec};

pub struct Sink {
    buf: Vec<u8>,
}

impl Sink {
    pub fn new(cap: usize) -> Self {
        Sink {
            buf: Vec::with_capacity(cap),
        }
    }

    pub(crate) fn write_byte(&mut self, b: u8) {
        self.buf.push(b)
    }

    pub(crate) fn write_bool(&mut self, b: bool) {
        if b {
            self.write_byte(1)
        } else {
            self.write_byte(0)
        }
    }

    pub(crate) fn write_bytes(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data)
    }

    #[allow(unused)]
    pub(crate) fn write_u32(&mut self, val: u32) {
        let buf = val.to_le_bytes();
        self.write_bytes(&buf)
    }

    pub(crate) fn write_usize(&mut self, val: usize) {
        let buf = val.to_le_bytes();
        self.write_bytes(&buf)
    }

    pub(crate) fn write_i64(&mut self, val: i64) {
        let buf = val.to_le_bytes();
        self.write_bytes(&buf);
    }

    #[allow(unused)]
    pub(crate) fn write_address(&mut self, addr: &Address) {
        self.write_byte(20);
        self.write_bytes(addr.into_slice());
    }

    pub(crate) fn write_string(&mut self, string: &str) {
        self.write_usize(string.len());
        self.write_bytes(string.as_bytes());
    }

    pub fn bytes(&self) -> &[u8] {
        &self.buf
    }

    pub fn into(self) -> Vec<u8> {
        self.buf
    }
}

pub struct Source<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Source<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { buf: data, pos: 0 }
    }

    #[allow(unused)]
    pub(crate) fn read_byte(&mut self) -> u8 {
        self.pos += 1;
        self.buf[self.pos - 1]
    }

    #[allow(unused)]
    pub(crate) fn read_bool(&mut self) -> bool {
        if self.read_byte() == 0 {
            return false;
        }
        true
    }

    pub(crate) fn read_bytes(&mut self, len: usize) -> &'a [u8] {
        self.pos += len;
        &self.buf[self.pos - len..self.pos]
    }

    pub(crate) fn read_u32(&mut self) -> u32 {
        self.pos += 4;
        u32::from_le_bytes(clone_into_array(&self.buf[self.pos - 4..self.pos]))
    }

    pub(crate) fn read_usize(&mut self) -> usize {
        self.read_u32() as usize
    }

    #[allow(unused)]
    pub(crate) fn read_i64(&mut self) -> i64 {
        self.pos += 8;
        i64::from_le_bytes(clone_into_array(&self.buf[self.pos - 8..self.pos]))
    }

    #[allow(unused)]
    pub(crate) fn read_address(&mut self) -> Address {
        let bytes = self.read_bytes(20);
        Address::new(&clone_into_array(bytes))
    }

    pub(crate) fn read_string(&mut self) -> String {
        let size = self.read_usize();
        let bytes = self.read_bytes(size);
        unsafe { String::from_utf8_unchecked(bytes.iter().cloned().collect()) }
    }
}

fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}
