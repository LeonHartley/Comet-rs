use bytes::BytesMut;

pub struct Buffer {
    pub id: i16,
    pub size: usize,
    index: usize,
    inner: BytesMut
}

impl Buffer {
    pub fn new(id: i16, size: usize, inner: BytesMut) -> Buffer {
        Buffer {
            id,
            index: 2,
            size,
            inner
        }
    }
}