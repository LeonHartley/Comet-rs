use bytes::BytesMut;
use bytes::ByteOrder;
use byteorder::BigEndian;

pub struct Buffer {
    pub id: i16,
    pub size: usize,
    index: usize,
    inner: BytesMut,
}

impl Buffer {
    pub fn new(id: i16, size: usize, inner: BytesMut) -> Buffer {
        Buffer {
            id,
            index: 2,
            size,
            inner,
        }
    }

    pub fn read_i32(&mut self) -> i32 {
        BigEndian::read_i32(&self.inner.as_ref())
    }
}