use bytes::BytesMut;

pub struct Buffer {
    index: usize,
    size: usize,
    inner: BytesMut
}

impl Buffer {
    pub fn new(size: usize, inner: BytesMut) -> Buffer {
        Buffer {
            index: 0,
            size,
            inner
        }
    }

    pub fn read_i32(&mut self) -> i32 {
        1231213
    }
}