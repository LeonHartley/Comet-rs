use actix::{Context, Handler, Message};
use actix::ActorFuture;
use actix::fut::{ok, WrapFuture};
use actix::prelude::*;
use db::query::player::PlayerByLoginTicket;
use game::player::Player;
use protocol::buffer::StreamMessage;
use session::PlayerContext;
use session::ServerSession;
use std::sync::Arc;

#[derive(Message)]
pub struct AuthenticateRequest(pub String);

impl Handler<AuthenticateRequest> for ServerSession {
    type Result = ();

    fn handle(&mut self, msg: AuthenticateRequest, ctx: &mut Context<Self>) {
        self.db.send(PlayerByLoginTicket(msg.0))
            .into_actor(self)
            .then(|res, act, ctx| {
                let p = match res {
                    Ok(p) => match p {
                        Some(p) => p,
                        None => {
                            act.stream.close();
                            return ok(());
                        }
                    }

                    _ => {
                        act.stream.close();
                        return ok(());
                    }
                };

                let player = Arc::new(p);
                let p = player.clone();
                let recipient = ctx.address().recipient::<StreamMessage>();
                act.set_player(PlayerContext {
                    addr: Player::create(move |_ctx| {
                        Player::new(recipient, player.clone())
                    }),
                    data: p,
                });

                ok(())
            }).spawn(ctx);
    }
}
