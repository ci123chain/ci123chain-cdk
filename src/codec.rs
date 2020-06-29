use crate::types::{Address, Error};

use crate::prelude::{str, Vec};

pub(crate) struct Sink {
    buf: Vec<u8>,
}

impl Sink {
    pub(crate) fn new(cap: usize) -> Self {
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

    pub(crate) fn write_str(&mut self, string: &str) {
        self.write_usize(string.len());
        self.write_bytes(string.as_bytes());
    }

    #[allow(unused)]
    pub(crate) fn bytes(&self) -> &[u8] {
        &self.buf
    }

    pub(crate) fn into(self) -> Vec<u8> {
        self.buf
    }
}

pub struct Source {
    buf: Vec<u8>,
    pos: usize,
    size: usize,
}

impl Source {
    pub(crate) fn new(data: Vec<u8>) -> Self {
        let length = data.len();
        Self {
            buf: data,
            pos: 0,
            size: length,
        }
    }

    #[allow(unused)]
    pub fn read_byte(&mut self) -> Result<u8, Error> {
        if self.pos >= self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos += 1;
        Ok(self.buf[self.pos - 1])
    }

    #[allow(unused)]
    pub fn read_bool(&mut self) -> Result<bool, Error> {
        if self.read_byte()? == 0 {
            return Ok(false);
        }
        Ok(true)
    }

    #[allow(unused)]
    pub fn read_bytes(&mut self, len: usize) -> Result<&[u8], Error> {
        if self.pos + len > self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos += len;
        Ok(&self.buf[self.pos - len..self.pos])
    }

    #[allow(unused)]
    pub fn read_u32(&mut self) -> Result<u32, Error> {
        if self.pos + 4 > self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos += 4;
        Ok(u32::from_le_bytes(clone_into_array(
            &self.buf[self.pos - 4..self.pos],
        )))
    }

    #[allow(unused)]
    pub fn read_u64(&mut self) -> Result<u64, Error> {
        if self.pos + 8 > self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos += 8;
        Ok(u64::from_le_bytes(clone_into_array(
            &self.buf[self.pos - 8..self.pos],
        )))
    }

    #[allow(unused)]
    pub fn read_usize(&mut self) -> Result<usize, Error> {
        Ok(self.read_u32()? as usize)
    }

    #[allow(unused)]
    pub fn read_i32(&mut self) -> Result<i32, Error> {
        if self.pos + 4 > self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos += 4;
        Ok(i32::from_le_bytes(clone_into_array(
            &self.buf[self.pos - 4..self.pos],
        )))
    }

    #[allow(unused)]
    pub fn read_i64(&mut self) -> Result<i64, Error> {
        if self.pos + 8 > self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos += 8;
        Ok(i64::from_le_bytes(clone_into_array(
            &self.buf[self.pos - 8..self.pos],
        )))
    }

    #[allow(unused)]
    pub fn read_address(&mut self) -> Result<Address, Error> {
        let bytes = self.read_bytes(20)?;
        Ok(Address::new(&clone_into_array(bytes)))
    }

    #[allow(unused)]
    pub fn read_str(&mut self) -> Result<&str, Error> {
        let size = self.read_usize()?;
        let bytes = self.read_bytes(size)?;
        match str::from_utf8(bytes) {
            Ok(s) => Ok(s),
            Err(_) => Err(Error::InvalidUtf8),
        }
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
