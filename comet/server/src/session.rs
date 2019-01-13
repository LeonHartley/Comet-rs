use actix::{Actor, Addr, Context, Handler, io::FramedWrite, io::WriteHandler, prelude::*, StreamHandler};
use codec::{GameCodec, IncomingMessage};
use core::Server;
use db::ctx::DbContext;
use game::ctx::GameContext;
use game::player::{Logout, Player};
use handler::MessageHandler;
use protocol::buffer::{Buffer, StreamMessage};
use protocol::composer;
use std::{io, sync::Arc};
use std::sync::Mutex;
use std::sync::RwLock;
use tokio_io::io::WriteHalf;
use tokio_tcp::TcpStream;

pub enum SessionStatus {
    Idle,
    Active,
}

pub struct PlayerContext {
    pub addr: Addr<Player>,
    pub data: Arc<RwLock<model::player::Player>>,
}

type NetworkStream = FramedWrite<WriteHalf<TcpStream>, GameCodec>;

pub struct ServerSession {
    pub server: Addr<Server>,
    pub db: Addr<DbContext>,
    pub game: Arc<GameContext>,
    pub stream: NetworkStream,
    pub player: Option<PlayerContext>,
    handler: MessageHandler,
}

impl ServerSession {
    pub fn new(game: Arc<GameContext>, server: Addr<Server>, db: Addr<DbContext>, stream: NetworkStream) -> Self {
        Self {
            server,
            db,
            game,
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

    pub fn player_actor(&self) -> Option<Addr<Player>> {
        match self.player {
            Some(ref ctx) => Some(ctx.addr.clone()),
            None => None
        }
    }

    pub fn player(&mut self) -> &Arc<RwLock<model::player::Player>> {
        if let Some(ref ctx) = self.player {
            &ctx.data
        } else {
            panic!("Player not initialised")
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

impl WriteHandler<io::Error> for ServerSession {}

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

            IncomingMessage::Event(buffers) => {
                for mut buffer in buffers.into_iter() {
                    self.handler.handle(buffer.id, &mut buffer, ctx.address());
                }
            }
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        if let Some(ref ctx) = self.player {
            ctx.addr.do_send(Logout)
        }

        ctx.stop();
    }
}
