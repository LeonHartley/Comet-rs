use byteorder::{BigEndian, ByteOrder};
use bytes::{BufMut, BytesMut};
use protocol::buffer::Buffer;
use std::io;
use std::option::Option;
use tokio_io::codec::{Decoder, Encoder};

pub struct GameCodec;

pub enum IncomingMessage {
    Policy,
    Event(Vec<Buffer>),
}

impl Decoder for GameCodec {
    type Item = IncomingMessage;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        debug!("received data: {:?}", src);

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

        let mut buffers = vec![];

        while src.len() >= size + 4 {
            src.split_to(4);
            let mut buf = src.split_to(size);
            src.clear();

            let id = BigEndian::read_i16(buf.as_ref());
            buf.advance(2);

            buffers.push(Buffer::new(id, buf))
        }

        Ok(Some(IncomingMessage::Event(buffers)))
    }
}

impl Encoder for GameCodec {
    type Item = Buffer;
    type Error = io::Error;

    fn encode(&mut self, item: Buffer, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item.id {
            0 => {
                let bytes = item.bytes();
                dst.reserve(bytes.len());
                dst.put_slice(bytes);
            }

            _ => {
                item.compose_to(dst);
                debug!("Composing {} bytes", dst.len());
            }
        }

        Ok(())
    }
}