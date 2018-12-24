use byteorder::{BigEndian, ByteOrder};
use bytes::{BytesMut};
use tokio_io::codec::{Decoder, Encoder};
use std::io;
use std::collections::HashMap;
use protocol::buffer::Buffer;

pub struct GameCodec;

impl Decoder for GameCodec {
    type Item = Buffer;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let size = {
            if src.len() < 2 {
                return Ok(None);
            }

            BigEndian::read_u32(src.as_ref()) as usize
        };

        if src.len() >= size + 2 {
            src.split_to(2);

            Ok(Some(parse_request(src.split_to(size)).unwrap()))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for GameCodec {
    type Item = Buffer;
    type Error = io::Error;

    fn encode(&mut self, item: Buffer, dst: &mut BytesMut) -> Result<(), <Self as Encoder>::Error> {
        unimplemented!()
    }
}

fn parse_request(buf: BytesMut) -> Option<Buffer> {
    let id = BigEndian::read_i16(buf.as_ref());
    println!("reading msg with id {}", id);

    Some(Buffer::new(id, buf.len(), buf))
}