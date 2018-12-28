use actix::{Context, Handler, Message};
use actix::ActorFuture;
use actix::fut::{ok, WrapFuture};
use actix::prelude::*;
use db::query::player::PlayerByLoginTicket;
use protocol::buffer::StreamMessage;
use protocol::composer::player::credits_composer;
use protocol::composer::player::player_info_composer;
use session::ServerSession;

#[derive(Message)]
pub struct InfoRetrieve;

impl Handler<InfoRetrieve> for ServerSession {
    type Result = ();

    fn handle(&mut self, msg: InfoRetrieve, ctx: &mut Context<Self>) {
        let player = match self.player_data() {
            Some(p) => p,
            _ => return
        };

        self.compose_all(vec![
            credits_composer(player.balance.credits),
            player_info_composer(player.as_ref())
        ]);
    }
}