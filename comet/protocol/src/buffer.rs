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
        BigEndian::read_i32(&self.inner.as_ref())
    }

    pub fn bytes(&self) -> &[u8] {
        &self.inner.as_ref()
    }
}