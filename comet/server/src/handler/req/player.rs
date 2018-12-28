use actix::{Context, Handler, Message};
use actix::ActorFuture;
use actix::fut::{ok, WrapFuture};
use actix::prelude::*;
use db::query::player::PlayerByLoginTicket;
use game::player::Player;
use protocol::buffer::StreamMessage;
use session::ServerSession;

#[derive(Message)]
pub struct InfoRetrieve;

impl Handler<InfoRetrieve> for ServerSession {
    type Result = ();

    fn handle(&mut self, msg: InfoRetrieve, ctx: &mut Context<Self>) {
        if let Some(ref player) = self.player_data() {
            println!("{:?}", player);
        } else {
            println!("oops");
        }
    }
}