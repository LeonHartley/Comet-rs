use protocol::buffer::{Buffer, StreamMessage};
use session::ServerSession;
use actix::*;
use actix::fut::*;
use std::sync::{Arc, RwLock};
use game::player::Player;
use db::query::player::PlayerByLoginTicket;
use handler::context::AuthenticateMessage;


impl Handler<AuthenticateMessage> for ServerSession {
    type Result = ();

    fn handle(&mut self, message: AuthenticateMessage, ctx: &mut Context<Self>) {
        match message.ticket {
            Some(ticket) => self.db
                .send(PlayerByLoginTicket(ticket))
                .into_actor(self)
                .then(|res, act, ctx| {
                    handle_authentication(res, act, ctx);
                    ok(())
                })
                .spawn(ctx),
            None => return
        }
    }
}

fn handle_authentication(
    res: Result<Option<model::player::Player>, MailboxError>,
    act: &mut ServerSession,
    ctx: &mut Context<ServerSession>) {
    match res.expect("error loading player") {
        Some(p) => {
            let game = act.game.clone();
            let recipient = ctx.address().recipient::<StreamMessage>();

            act.set_player(Player::create(move |_ctx| {
                Player::new(game, recipient, p)
            }))
        }
        None => act.stream.close()
    }
}
