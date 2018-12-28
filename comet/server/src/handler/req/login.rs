use actix::{Context, Handler, Message};
use actix::ActorFuture;
use actix::fut::WrapFuture;
use actix::prelude::*;
use actix_web::actix;
use db::query::player::PlayerByLoginTicket;
use futures::future::Future;
use futures::Stream;
use session::ServerSession;

#[derive(Message)]
pub struct AuthenticateRequest(pub String);

impl Handler<AuthenticateRequest> for ServerSession {
    type Result = ();

    fn handle(&mut self, msg: AuthenticateRequest, ctx: &mut Context<Self>) {
        self.db.send(PlayerByLoginTicket(msg.0))
            .into_actor(self)
            .then(|res, mut act, ctx| {
                let p = match res {
                    Ok(p) => match p {
                        Some(p) => p,
                        None => {
                            return actix::fut::ok(());
                        }
                    }

                    _ => {
                        ctx.stop();
                        return actix::fut::ok(());
                    }
                };

                println!("{}", p.name);
                actix::fut::ok(())
            })
            .wait(ctx);
    }
}
