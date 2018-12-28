use actix::Actor;
use actix::Addr;
use actix::Context;
use actix::Handler;
use actix::prelude::*;
use actix::StreamHandler;
use actix_web::actix;
use codec::GameCodec;
use codec::IncomingMessage;
use core::Server;
use db::ctx::DbContext;
use game::player::Player;
use handler::MessageHandler;
use protocol::buffer::Buffer;
use protocol::buffer::StreamMessage;
use protocol::composer;
use std::borrow::Borrow;
use std::io;
use std::sync::Arc;
use tokio_io::io::WriteHalf;
use tokio_tcp::TcpStream;

pub enum SessionStatus {
    Idle,
    Active,
}

pub struct PlayerContext {
    pub addr: Addr<Player>,
    pub data: Arc<model::player::Player>,
}

type NetworkStream = actix::io::FramedWrite<WriteHalf<TcpStream>, GameCodec>;

pub struct ServerSession {
    pub server: Addr<Server>,
    pub db: Addr<DbContext>,
    pub stream: NetworkStream,
    player: Option<PlayerContext>,
    handler: MessageHandler,
}

impl ServerSession {
    pub fn new(server: Addr<Server>, db: Addr<DbContext>, stream: NetworkStream) -> Self {
        Self {
            server,
            db,
            stream,
            handler: MessageHandler::new(),
            player: None,
        }
    }

    pub fn compose(&mut self, buf: Buffer) {
        self.stream.write(buf);
    }

    pub fn compose_all(&mut self, buffers: Vec<Buffer>) {
        for buf in buffers.into_iter() {
            self.compose(buf);
        }
    }

    pub fn player(&self) -> Option<Addr<Player>> {
        match self.player {
            Some(ref ctx) => Some(ctx.addr.clone()),
            None => None
        }
    }

    pub fn player_data(&self) -> Option<Arc<model::player::Player>> {
        match self.player {
            Some(ref ctx) => Some(ctx.data.clone()),
            _ => None
        }
    }

    pub fn player_balance(&self) -> Option<model::player::PlayerBalance> {
        match self.player {
            Some(ref ctx) => Some(ctx.data.balance),
            _ => None
        }
    }

    pub fn set_player(&mut self, ctx: PlayerContext) {
        self.player = Some(ctx);
    }

    pub fn status(&self) -> SessionStatus {
        match self.player {
            Some(_) => SessionStatus::Active,
            _ => SessionStatus::Idle
        }
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

            StreamMessage::BufferedSend(buffers) => {
                for buf in buffers.into_iter() {
                    self.stream.write(buf)
                }
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
                self.stream.write(composer::handshake::policy_file());
                self.stream.close();
            }

            IncomingMessage::Event(mut buffer) => {
                self.handler.handle(buffer.id, &mut buffer, ctx.address());
            }
        }
    }
}
