extern crate model;

use model::config::Game;

pub struct Server {
    host: String,
    port: i16
}

impl Server {
    pub fn new(config: &Game) -> Self {
        Server {
            host: config.host.clone(),
            port: config.port
        }
    }

    pub fn start(&self) {

    }
}