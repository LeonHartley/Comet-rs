use bytes::BytesMut;
use bytes::ByteOrder;
use byteorder::BigEndian;

pub struct Buffer {
    pub id: i16,
    pub size: usize,
    pub inner: BytesMut,
}

impl Buffer {
    pub fn new(id: i16, size: usize, inner: BytesMut) -> Buffer {
        Buffer {
            id,
            size,
            inner,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Buffer {
        Buffer {
            id: 0,
            size: bytes.len(),
            inner: BytesMut::from(bytes)
        }
    }

    pub fn read_i32(&mut self) -> i32 {
        let i = BigEndian::read_i32(&self.inner.as_ref());

        self.inner.advance(4);

        i
    }

    pub fn read_i16(&mut self) -> i16 {
        let i = BigEndian::read_i16(&self.inner.as_ref());

        self.inner.advance(2);

        i
    }

    pub fn read_string(&mut self) -> Option<String> {
        let len = self.read_i16() as usize;
        let buf = self.inner.split_to(len);

        match String::from_utf8(buf.as_ref().to_vec()) {
            Ok(s) => Some(s),
            _ => None
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.inner.as_ref()
    }
}