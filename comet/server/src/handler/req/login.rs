use actix::{Context, Handler, Message};
use actix::ActorFuture;
use actix::fut::{ok, WrapFuture};
use actix::prelude::*;
use db::query::player::PlayerByLoginTicket;
use futures::Stream;
use session::ServerSession;
use actix;

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

                println!("{}", p.name);
                ok(())
            })
            .wait(ctx);
    }
}
