use actix::{Actor, Context, Recipient};
use actix::AsyncContext;
use container::{ComponentSet, Container};
use ctx::GameContext;
use model::player;
use player::service::PlayerService;
use protocol::buffer::StreamMessage;
use protocol::composer::{handshake::{auth_ok_composer, motd_composer}, player::rights::{allowances_composer, fuserights_composer}};
use std::sync::Arc;

pub struct Player {
    game: Arc<GameContext>,
    stream: Recipient<StreamMessage>,
    inner: Arc<player::Player>,
    components: ComponentSet,
}

impl Player {
    pub fn new(game: Arc<GameContext>, stream: Recipient<StreamMessage>, inner: Arc<player::Player>) -> Player {
        Player { game, stream, inner, components: ComponentSet::new() }
    }

    pub fn data(&self) -> &player::Player { &self.inner }

    pub fn game(&self) -> &GameContext { self.game.as_ref() }

    pub fn game_mut(&mut self) -> &mut GameContext { Arc::get_mut(&mut self.game).expect("No game context") }
}

impl Container for Player {
    fn components(&self) -> &ComponentSet { &self.components }

    fn components_mut(&mut self) -> &mut ComponentSet { &mut self.components }
}

impl Actor for Player {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let avatar = &self.data().avatar;

        self.game_mut().add_online_player(
            ctx.address(),
            avatar.id,
            avatar.name.clone());

        let _ = self.stream.do_send(StreamMessage::BufferedSend(vec![
            auth_ok_composer(),
            fuserights_composer(self.data().rank, true),
            allowances_composer(),
            motd_composer(format!("data: {:?}", self.data()))
        ]));
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("{} logged out", self.data().avatar.name);
        // Distribute any messages to notify friends/rooms
    }
}