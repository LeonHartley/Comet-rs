use actix::{Actor, Message, Recipient, Context};
use model::player;
use protocol::buffer::StreamMessage;

pub struct Player {
    stream: Recipient<StreamMessage>,
    inner: player::Player,
}

impl Player {
    pub fn new(stream: Recipient<StreamMessage>, inner: player::Player) -> Player {
        Player { stream, inner }
    }

    pub fn data_mut(&mut self) -> &mut player::Player {
        &mut self.inner
    }

    pub fn data(&self) -> &player::Player {
        &self.inner
    }
}

impl Actor for Player {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Player {} logged in", self.data().name);
    }
}