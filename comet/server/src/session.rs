use codec::GameCodec;
use actix::Addr;
use actix_web::actix;
use tokio_io::io::WriteHalf;
use tokio_tcp::TcpStream;
use protocol::buffer::Buffer;
use actix::StreamHandler;
use std::io;
use actix::Context;
use core::Server;
use actix::Actor;

pub enum SessionStatus {
    Idle,
    Active
}

type NetworkWriter = actix::io::FramedWrite<WriteHalf<TcpStream>, GameCodec>;

pub struct ServerSession {
    id: usize,
    server: Addr<Server>,
    status: SessionStatus,
    writer: NetworkWriter
}

impl ServerSession {
    pub fn new(id: usize, server: Addr<Server>, writer: NetworkWriter) -> Self {
        Self {
            id,
            server,
            status: SessionStatus::Idle,
            writer
        }
    }
}

impl actix::io::WriteHandler<io::Error> for ServerSession {}

impl Actor for ServerSession {
    type Context = Context<Self>;

}

impl StreamHandler<Buffer, io::Error> for ServerSession {
    fn handle(&mut self, item: Buffer, ctx: &mut Context<Self>) {
        println!("buf id: {}", item.id);
    }
}