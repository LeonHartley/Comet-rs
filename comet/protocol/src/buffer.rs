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
}