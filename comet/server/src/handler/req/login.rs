use actix::{AsyncContext, Context, Handler, Message};
use actix::ActorFuture;
use actix::fut::{ok, WrapFuture};
use actix::prelude::*;
use actix_web::actix;
use db::query::player::PlayerByLoginTicket;
use game::player::Player;
use protocol::buffer::StreamMessage;
use session::PlayerContext;
use session::ServerSession;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Message)]
pub struct AuthenticateRequest(pub String);

impl Handler<AuthenticateRequest> for ServerSession {
    type Result = ();

    fn handle(&mut self, msg: AuthenticateRequest, ctx: &mut Context<Self>) {
        self.db
            .send(PlayerByLoginTicket(msg.0))
            .into_actor(self)
            .then(|res, act, ctx| {
                handle_authentication(res, act, ctx);
                actix::fut::ok(())
            })
            .spawn(ctx);
    }
}

fn handle_authentication(res: Result<Option<model::player::Player>, MailboxError>, mut act: &mut ServerSession, ctx: &mut Context<ServerSession>) {
    match res.expect("error loading player") {
        Some(p) => {
            let player = Arc::new(RwLock::new(p));

            let game = act.game.clone();
            let data = player.clone();
            let recipient = ctx.address().recipient::<StreamMessage>();

            act.set_player(PlayerContext {
                addr: Player::create(move |_ctx| {
                    Player::new(game, recipient, player.clone())
                }),
                data,
            });
        }
        None => {
            act.stream.close();
        }
    }
}