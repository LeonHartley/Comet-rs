extern crate model;
extern crate actix;
extern crate protocol;
extern crate bytes;
extern crate byteorder;
extern crate tokio_io;

mod codec;
mod handler;

use model::config::{Game};
use actix::{Context, Actor, Arbiter, msgs};

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

        let server = Arbiter::start(move |_| Self { host, port });
        let srv = server.clone();

        Arbiter::new("tcp-server").do_send::<msgs::Execute>(msgs::Execute::new(move || {

            Ok(())
        }));
    }
}