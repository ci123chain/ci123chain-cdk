use crate::types::Error;
use crate::util::clone_into_array;

use crate::prelude::{str, Cell, Vec};

pub struct Sink {
    buf: Vec<u8>,
}

impl Sink {
    pub fn new(cap: usize) -> Self {
        Sink {
            buf: Vec::with_capacity(cap),
        }
    }

    pub fn write_byte(&mut self, b: u8) {
        self.buf.push(b)
    }

    pub fn write_bool(&mut self, b: bool) {
        if b {
            self.write_byte(1)
        } else {
            self.write_byte(0)
        }
    }

    pub fn write_bytes(&mut self, data: &[u8]) {
        self.write_usize(data.len());
        self.write_raw_bytes(data);
    }

    pub fn write_u32(&mut self, val: u32) {
        let buf = val.to_le_bytes();
        self.write_raw_bytes(&buf);
    }

    pub fn write_u64(&mut self, val: u64) {
        let buf = val.to_le_bytes();
        self.write_raw_bytes(&buf);
    }

    pub fn write_usize(&mut self, val: usize) {
        let buf = val.to_le_bytes();
        self.write_raw_bytes(&buf);
    }

    pub fn write_i32(&mut self, val: i32) {
        let buf = val.to_le_bytes();
        self.write_raw_bytes(&buf);
    }

    pub fn write_i64(&mut self, val: i64) {
        let buf = val.to_le_bytes();
        self.write_raw_bytes(&buf);
    }

    pub fn write_u128(&mut self, val: u128) {
        let buf = val.to_le_bytes();
        self.write_raw_bytes(&buf);
    }

    pub fn write_i128(&mut self, val: i128) {
        let buf = val.to_le_bytes();
        self.write_raw_bytes(&buf);
    }

    // #[allow(unused)]
    // pub(crate) fn write_address(&mut self, addr: &Address) {
    //     self.write_raw_bytes(addr.into_slice());
    // }

    pub fn write_str(&mut self, string: &str) {
        self.write_bytes(string.as_bytes());
    }

    pub(crate) fn write_raw_bytes(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf
    }

    pub fn into(self) -> Vec<u8> {
        self.buf
    }
}

pub struct Source<'a> {
    buf: &'a [u8],
    pos: Cell<usize>,
    size: usize,
}

impl<'a> Source<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let length = data.len();
        Self {
            buf: data,
            pos: Cell::new(0),
            size: length,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn is_eof(&self) -> bool {
        self.pos.get() >= self.size
    }

    pub fn read_byte(&self) -> Result<u8, Error> {
        let old_pos = self.pos.get();
        let new_pos = old_pos + 1;
        if old_pos >= self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos.set(new_pos);
        Ok(self.buf[old_pos])
    }

    pub fn read_bool(&self) -> Result<bool, Error> {
        if self.read_byte()? == 0 {
            return Ok(false);
        }
        Ok(true)
    }

    pub fn read_bytes(&self) -> Result<&[u8], Error> {
        let len = self.read_usize()?;
        self.read_raw_bytes(len)
    }

    pub fn read_u32(&self) -> Result<u32, Error> {
        let old_pos = self.pos.get();
        let new_pos = old_pos + 4;
        if new_pos > self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos.set(new_pos);
        Ok(u32::from_le_bytes(clone_into_array(
            &self.buf[old_pos..new_pos],
        )))
    }

    pub fn read_u64(&self) -> Result<u64, Error> {
        let old_pos = self.pos.get();
        let new_pos = old_pos + 8;
        if new_pos > self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos.set(new_pos);
        Ok(u64::from_le_bytes(clone_into_array(
            &self.buf[old_pos..new_pos],
        )))
    }

    pub fn read_usize(&self) -> Result<usize, Error> {
        Ok(self.read_u32()? as usize)
    }

    pub fn read_i32(&self) -> Result<i32, Error> {
        let old_pos = self.pos.get();
        let new_pos = old_pos + 4;
        if new_pos > self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos.set(new_pos);
        Ok(i32::from_le_bytes(clone_into_array(
            &self.buf[old_pos..new_pos],
        )))
    }

    pub fn read_i64(&self) -> Result<i64, Error> {
        let old_pos = self.pos.get();
        let new_pos = old_pos + 8;
        if new_pos > self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos.set(new_pos);
        Ok(i64::from_le_bytes(clone_into_array(
            &self.buf[old_pos..new_pos],
        )))
    }

    pub fn read_u128(&self) -> Result<u128, Error> {
        let old_pos = self.pos.get();
        let new_pos = old_pos + 16;
        if new_pos > self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos.set(new_pos);
        Ok(u128::from_le_bytes(clone_into_array(
            &self.buf[old_pos..new_pos],
        )))
    }

    pub fn read_i128(&self) -> Result<i128, Error> {
        let old_pos = self.pos.get();
        let new_pos = old_pos + 16;
        if new_pos > self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos.set(new_pos);
        Ok(i128::from_le_bytes(clone_into_array(
            &self.buf[old_pos..new_pos],
        )))
    }

    // pub fn read_address(&self) -> Result<Address, Error> {
    //     let bytes = self.read_raw_bytes(Address::len())?;
    //     Ok(Address::new(&clone_into_array(bytes)))
    // }

    pub fn read_str(&self) -> Result<&str, Error> {
        let bytes = self.read_bytes()?;
        match str::from_utf8(bytes) {
            Ok(s) => Ok(s),
            Err(_) => Err(Error::InvalidUtf8),
        }
    }

    pub(crate) fn read_raw_bytes(&self, len: usize) -> Result<&[u8], Error> {
        let old_pos = self.pos.get();
        let new_pos = old_pos + len;
        if new_pos > self.size {
            return Err(Error::UnexpectedEOF);
        }
        self.pos.set(new_pos);
        Ok(&self.buf[old_pos..new_pos])
    }
}

pub(crate) fn from_hex_u8(c: u8) -> Result<u8, Error> {
    if '0' as u8 <= c && c <= '9' as u8 {
        Ok(c - '0' as u8)
    } else if 'a' as u8 <= c && c <= 'f' as u8 {
        Ok(c - 'a' as u8 + 10)
    } else if 'A' as u8 <= c && c <= 'F' as u8 {
        Ok(c - 'A' as u8 + 10)
    } else {
        Err(Error::IrregularData)
    }
}
