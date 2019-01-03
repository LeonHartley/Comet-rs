use actix::Actor;
use actix::Addr;
use actix::Context;
use actix::prelude::*;
use actix_web::actix;
use codec::GameCodec;
use core::Server;
use db::ctx::DbContext;
use futures::Stream;
use game::ctx::GameContext;
use session::ServerSession;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use tokio_io::_tokio_codec::FramedRead;
use tokio_io::AsyncRead;
use tokio_tcp::{TcpListener, TcpStream};

pub struct TcpServer {
    server: Addr<Server>,
    db: Addr<DbContext>,
    game: Arc<GameContext>,
}

impl TcpServer {
    pub fn new(addr: String, server: Addr<Server>, db: Addr<DbContext>, game: Arc<GameContext>) {
        let addr = SocketAddr::from_str(&addr).unwrap();
        let listener = TcpListener::bind(&addr).unwrap();

        TcpServer::create(move |ctx| {
            ctx.add_message_stream(
                listener
                    .incoming()
                    .map_err(|_| ())
                    .map(|s| TcpConnect(s)),
            );

            info!("Server started on addr: {}", &addr);
            TcpServer { server, db, game }
        });
    }
}


impl Actor for TcpServer {
    type Context = Context<Self>;
}

#[derive(Message)]
struct TcpConnect(TcpStream);

impl Handler<TcpConnect> for TcpServer {
    type Result = ();

    fn handle(&mut self, msg: TcpConnect, _: &mut Context<Self>) {
        let server = self.server.clone();
        let db = self.db.clone();
        let game = self.game.clone();

        ServerSession::create(|ctx| {
            let (r, w) = msg.0.split();

            ServerSession::add_stream(FramedRead::new(r, GameCodec), ctx);
            ServerSession::new(game, server, db, actix::io::FramedWrite::new(w, GameCodec, ctx))
        });
    }
}
