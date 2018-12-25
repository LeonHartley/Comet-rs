use codec::GameCodec;
use actix::Addr;
use actix_web::actix;
use tokio_io::io::WriteHalf;
use tokio_tcp::TcpStream;
use actix::StreamHandler;
use std::io;
use actix::Context;
use core::Server;
use actix::Actor;
use codec::IncomingMessage;
use protocol::event::handshake::policy_file;
use handler::MessageHandler;
use protocol::event;
use protocol::buffer::Buffer;
use actix::Handler;
use actix::prelude::*;
use protocol::buffer::StreamMessage;

pub enum SessionStatus {
    Idle,
    Active,
}

type NetworkStream = actix::io::FramedWrite<WriteHalf<TcpStream>, GameCodec>;

pub struct ServerSession {
    server: Addr<Server>,
    status: SessionStatus,
    stream: NetworkStream,
    handler: MessageHandler,
}

impl ServerSession {
    pub fn new(server: Addr<Server>, stream: NetworkStream) -> Self {
        Self {
            server,
            status: SessionStatus::Idle,
            stream,
            handler: MessageHandler::new(),
        }
    }

    pub fn compose(&mut self, buf: Buffer) {
        self.stream.write(buf);
    }
}

impl actix::io::WriteHandler<io::Error> for ServerSession {}

impl Actor for ServerSession {
    type Context = Context<Self>;
}

impl Handler<StreamMessage> for ServerSession {
    type Result = ();

    fn handle(&mut self, msg: StreamMessage, _: &mut Context<Self>) {
        match msg {
            StreamMessage::Send(buf) => {
                self.compose(buf);
            }

            StreamMessage::SendMultiple(buffers) => {
                for buf in buffers.into_iter() {}
            }

            StreamMessage::Close => {
                self.stream.close();
            }
        }
    }
}

impl StreamHandler<IncomingMessage, io::Error> for ServerSession {
    fn handle(&mut self, item: IncomingMessage, ctx: &mut Context<Self>) {
        match item {
            IncomingMessage::Policy => {
                self.stream.write(event::handshake::policy_file());
                self.stream.close();
            }

            IncomingMessage::Event(mut buffer) => {
                self.handler.handle(buffer.id, &mut buffer, ctx.address());
            }
        }
    }
}
