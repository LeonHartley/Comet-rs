use actix::Actor;
use actix::Context;
use model::config::Game;
use actix::Arbiter;
use actix::msgs;

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

    pub fn start(&self) {
        let host = self.host.clone();
        let port = self.port;

        let addr = format!("{}:{}", &host, port);

        let server = Arbiter::start(move |_| Self { host, port });
        let srv = server.clone();

        Arbiter::new("tcp-server").do_send::<msgs::Execute>(msgs::Execute::new(move || {
            TcpServer::new(addr, srv);
            Ok(())
        }));
    }
}