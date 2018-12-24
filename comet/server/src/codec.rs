use byteorder::{BigEndian, ByteOrder};
use bytes::{BytesMut, BufMut};
use tokio_io::codec::{Decoder, Encoder};
use std::io;
use std::collections::HashMap;
use protocol::buffer::Buffer;
use std::option::Option;

pub struct GameCodec;

pub enum IncomingMessage {
    Policy,
    Event(Buffer)
}

impl Decoder for GameCodec {
    type Item = IncomingMessage;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.first() == Some(&b'<') {
            src.clear();

            return Ok(Some(IncomingMessage::Policy));
        }

        let size = {
            if src.len() < 4 {
                return Ok(None);
            }

            BigEndian::read_u32(src.as_ref()) as usize
        };

        if src.len() >= size + 4 {
            src.split_to(4);
            let mut buf = src.split_to(size);

            src.clear();

            Ok(Some(IncomingMessage::Event(parse_request(buf).unwrap())))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for GameCodec {
    type Item = Buffer;
    type Error = io::Error;

    fn encode(&mut self, item: Buffer, dst: &mut BytesMut) -> Result<(), <Self as Encoder>::Error> {
        let bytes = item.bytes();
        dst.reserve(bytes.len());
        dst.put_slice(bytes);

        Ok(())
    }
}

fn parse_request(mut buf: BytesMut) -> Option<Buffer> {
    let id = BigEndian::read_i16(buf.as_ref());
    buf.advance(2);

    Some(Buffer::new(id, buf.len(), buf))
}