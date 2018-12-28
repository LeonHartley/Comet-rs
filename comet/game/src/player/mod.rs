use actix::Recipient;
use model::player;
use protocol::buffer::StreamMessage;

pub struct Player {
    stream: Recipient<StreamMessage>,
    inner: player::Player,
}

impl Player {
    pub fn data_mut(&mut self) -> &mut player::Player {
        &mut self.inner
    }

    pub fn data(&self) -> &player::Player {
        &self.inner
    }
}