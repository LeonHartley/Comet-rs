use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use actix::{Actor, ActorContext, AsyncContext, Context, Handler, Message, Recipient};
use container::{ComponentSet, Container};
use ctx::GameContext;
use model::player;
use player::service::PlayerService;
use protocol::buffer::StreamMessage;
use protocol::composer::{handshake::{auth_ok_composer, motd_composer}, player::rights::{allowances_composer, fuserights_composer}};

pub struct Player {
    pub inner: Arc<RwLock<player::Player>>,
    pub stream: Recipient<StreamMessage>,
    pub game: Arc<GameContext>,
    components: ComponentSet,
}

impl Player {
    pub fn new(game: Arc<GameContext>, stream: Recipient<StreamMessage>, inner: Arc<RwLock<player::Player>>) -> Player {
        Player { game, stream, inner, components: ComponentSet::new() }
    }
}

impl Container for Player {
    fn components(&self) -> &ComponentSet { &self.components }

    fn components_mut(&mut self) -> &mut ComponentSet { &mut self.components }
}

impl Actor for Player {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        if let Ok(player) = self.inner.read() {
            info!("{} logged in", player.avatar.name);

            self.game.add_online_player(ctx.address(), player.avatar.id, player.avatar.name.clone());

            let _ = self.stream.do_send(StreamMessage::BufferedSend(vec![
                auth_ok_composer(),
                fuserights_composer(player.rank, true),
                allowances_composer(),
                motd_composer(format!("data: {:?}", player))
            ]));
        }
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        if let Ok(player) = self.inner.read() {
            info!("{} logged out", player.avatar.name);

            self.game.remove_online_player(player.avatar.id, player.avatar.name.clone());

            // Distribute any messages to notify friends/rooms
        }
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