use actix::{Actor, ActorContext, AsyncContext, Context, Handler, Message, Recipient};
use container::{ComponentSet, Container};
use ctx::GameContext;
use model::player;
use player::service::PlayerService;
use protocol::buffer::StreamMessage;
use protocol::composer::{handshake::{auth_ok_composer, motd_composer}, player::rights::{allowances_composer, fuserights_composer}};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use protocol::composer::handshake::availability_status_composer;

pub struct Player {
    pub inner: player::Player,
    pub stream: Recipient<StreamMessage>,
    pub game: Arc<GameContext>,
    components: ComponentSet,
}

impl Player {
    pub fn new(game: Arc<GameContext>, stream: Recipient<StreamMessage>, inner: player::Player) -> Player {
        Player { game, stream, inner, components: ComponentSet::new() }
    }
}

impl Actor for Player {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("{} logged in", self.inner.avatar.name);

        self.game.add_online_player(ctx.address(), self.inner.avatar.id, self.inner.avatar.name.clone());

        let _ = self.stream.do_send(StreamMessage::BufferedSend(vec![
            auth_ok_composer(),
            availability_status_composer(),
            fuserights_composer(self.inner.rank, true),
            allowances_composer(),
            motd_composer(format!("data: {:?}", self.inner))
        ]));
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("{} logged out", self.inner.avatar.name);

        self.game.remove_online_player(self.inner.avatar.id, self.inner.avatar.name.clone());

        // Distribute any messages to notify friends/rooms
    }
}

#[derive(Message)]
pub struct Logout;

impl Handler<Logout> for Player {
    type Result = ();

    fn handle(&mut self, msg: Logout, ctx: &mut Context<Player>) {
        self.stream.do_send(StreamMessage::Close);
        ctx.stop();
    }
}
