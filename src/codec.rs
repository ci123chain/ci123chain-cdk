use types::Address;

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

    pub(crate) fn write_address(&mut self, addr: &Address) {
        self.write_byte(20);
        self.write_bytes(addr);
    }

    pub fn bytes(&self) -> &[u8] {
        &self.buf
    }

    pub fn into(self) -> Vec<u8> {
        self.buf
    }
}
