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
use protocol::handshake::policy_file;
use handler::MessageHandler;

pub enum SessionStatus {
    Idle,
    Active,
}

type NetworkStream = actix::io::FramedWrite<WriteHalf<TcpStream>, GameCodec>;

pub struct ServerSession {
    server: Addr<Server>,
    status: SessionStatus,
    stream: NetworkStream,
    handler: MessageHandler
}

impl ServerSession {
    pub fn new(server: Addr<Server>, stream: NetworkStream) -> Self {
        Self {
            server,
            status: SessionStatus::Idle,
            stream,
            handler: MessageHandler::new()
        }
    }
}

impl actix::io::WriteHandler<io::Error> for ServerSession {}

impl Actor for ServerSession {
    type Context = Context<Self>;
}

impl StreamHandler<IncomingMessage, io::Error> for ServerSession {
    fn handle(&mut self, item: IncomingMessage, _ctx: &mut Context<Self>) {
        match item {
            IncomingMessage::Policy => {
                self.stream.write(policy_file());
                self.stream.close();
            }

            IncomingMessage::Event(mut buffer) => {
                self.handler.handle(buffer.id, &mut buffer, &self);
            }
        }
    }
}