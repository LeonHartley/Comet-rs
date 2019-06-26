use db::ctx::DbContext;
use game::ctx::GameContext;
use model::config::Game;
use std::rc::Rc;
use std::sync::Arc;
use actix::*;

use tcp::TcpServer;

pub struct Server {
    address: String
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Server {
    fn new(address: String) -> Self {
        Self {
            address
        }
    }

    pub fn start(addr: String, db: Addr<DbContext>, game: Arc<GameContext>) {
        let server = Server::new(addr.clone()).start();
        TcpServer::new(addr, server, db, game);
//        let server_addr = server.clone();
//        Arbiter::new().exec(move || {
//            println!("hmmm");
//            Ok::<_, ()>(())
//        });
    }
}
