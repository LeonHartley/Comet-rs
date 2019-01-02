use actix::{Actor, Context, Recipient};
use core::{ComponentSet, Container};
use model::player;
use protocol::buffer::StreamMessage;
use protocol::composer::{handshake::{auth_ok_composer, motd_composer}, player::rights::{allowances_composer, fuserights_composer}};
use std::sync::Arc;

pub mod component;

pub struct Player {
    stream: Recipient<StreamMessage>,
    inner: Arc<player::Player>,
    components: ComponentSet,
}

impl Player {
    pub fn new(stream: Recipient<StreamMessage>, inner: Arc<player::Player>) -> Player {
        Player { stream, inner, components: ComponentSet::new() }
    }

    pub fn data(&self) -> &player::Player { &self.inner }
}

impl Container for Player {
    fn components(&self) -> &ComponentSet { &self.components }

    fn components_mut(&mut self) -> &mut ComponentSet { &mut self.components }
}

impl Actor for Player {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("{} logged in", self.data().name);

        let _ = self.stream.do_send(StreamMessage::BufferedSend(vec![
            auth_ok_composer(),
            fuserights_composer(self.data().rank, true),
            allowances_composer(),
            motd_composer(format!("data: {:?}", self.data()))
        ]));
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("{} logged out", self.data().name);
        // Distribute any messages to notify friends/rooms
    }
}