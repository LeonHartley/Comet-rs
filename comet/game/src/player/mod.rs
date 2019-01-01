use actix::{Actor, Context, Message, Recipient};
use model::player;
use protocol::{buffer::StreamMessage, composer::handshake::{auth_ok_composer, motd_composer}};
use protocol::composer::player::rights::*;
use std::sync::Arc;

pub struct Player {
    stream: Recipient<StreamMessage>,
    inner: Arc<player::Player>,
}

impl Player {
    pub fn new(stream: Recipient<StreamMessage>, inner: Arc<player::Player>) -> Player {
        Player { stream, inner }
    }

    pub fn data(&self) -> &player::Player {
        &self.inner
    }
}

impl Actor for Player {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("{} logged in", self.data().name);

        let _ = self.stream.do_send(StreamMessage::BufferedSend(vec![
            auth_ok_composer(),
            fuserights_composer(self.data().rank, true),
            allowances_composer(),
            motd_composer(format!("data: {:?}", self.data()))
        ]));
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!("{} logged out", self.data().name);
    }
}