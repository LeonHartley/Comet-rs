use byteorder::{BigEndian, ByteOrder};
use bytes::{BufMut, BytesMut};
use tokio_io::codec::{Decoder, Encoder};
use std::io;
use std::collections::HashMap;

pub struct GameCodec {
    decoders: HashMap<i16, String>
}

impl Decoder for GameCodec {
    type Item = u16;
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

fn parse_request(buf: BytesMut) -> Option<u16> {
    let id = BigEndian::read_u16(buf.as_ref());
    println!("reading msg with id {}", id);

    Some(id)
}