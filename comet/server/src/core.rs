use actix::{Actor, Addr, Arbiter, Context, msgs};
use db::ctx::DbContext;
use game::ctx::GameContext;
use model::config::Game;
use std::rc::Rc;
use std::sync::Arc;
use tcp::TcpServer;

pub struct Server {
    host: String,
    port: i16,
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Server {
    pub fn new(config: &Game) -> Self {
        Server {
            host: config.host.clone(),
            port: config.port,
        }
    }

    pub fn start(&self, db: Addr<DbContext>, game: Arc<GameContext>) {
        let host = self.host.clone();
        let port = self.port;
        let addr = format!("{}:{}", &host, port);

        let server = Arbiter::start(move |_| Self { host, port });

        Arbiter::new("tcp-server").do_send::<msgs::Execute>(msgs::Execute::new(move || {
            TcpServer::new(addr, server, db, game);
            Ok(())
        }));
    }
}