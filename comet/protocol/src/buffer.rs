use bytes::{BytesMut, BufMut};
use bytes::ByteOrder;
use byteorder::BigEndian;
use actix::prelude::*;

#[derive(Message)]
pub enum StreamMessage {
    Send(Buffer),
    SendMultiple(Vec<Buffer>),
    Close
}

pub struct Buffer {
    pub id: i16,
    pub size: usize,
    pub inner: BytesMut
}

impl Buffer {
    pub fn new(id: i16, size: usize, inner: BytesMut) -> Buffer {
        Buffer {
            id,
            size,
            inner
        }
    }

    pub fn empty(id: i16) -> Buffer {
        Buffer {
            id,
            size: 1024,
            inner: BytesMut::new()
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

    pub fn write_i32(&mut self, i: i32) {
        self.inner.reserve(4);
        self.inner.put_i32_be(i);
    }

    pub fn write_string(&mut self, s: String) {
        self.inner.reserve(2 + s.len());
        self.inner.put_i16_be(s.len() as i16);
        self.inner.put_slice(s.as_bytes());
    }

    pub fn compose_to(&self, buf: &mut BytesMut) {
        buf.reserve(6 + self.inner.len());
        buf.put_i32_be((self.inner.len() as i32) + 2);
        buf.put_i16_be(self.id);
        buf.put_slice(self.inner.as_ref());
    }

    pub fn bytes(&self) -> &[u8] {
        &self.inner.as_ref()
    }
}