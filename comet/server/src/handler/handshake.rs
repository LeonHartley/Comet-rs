use protocol::buffer::{Buffer, StreamMessage};
use session::{PlayerContext, ServerSession};
use actix::*;
use std::sync::{Arc, RwLock};
use game::player::Player;
use db::query::player::PlayerByLoginTicket;

pub fn client_version_handler(buf: &mut Buffer, _: &mut ServerSession,
                              _: &mut Context<ServerSession>) {
    match buf.read_string() {
        Some(s) => debug!("client version: {}", s),
        _ => return
    };
}

pub fn authentication_handler(buf: &mut Buffer, session: &mut ServerSession,
                              ctx: &mut Context<ServerSession>) {
    if let Some(ticket) = buf.read_string() {
        session.db
            .send(PlayerByLoginTicket(ticket))
            .into_actor(session)
            .then(|res, act, ctx| {
                handle_authentication(res, act, ctx);
                actix::fut::ok(())
            })
            .spawn(ctx);
    }
}

fn handle_authentication(
    res: Result<Option<model::player::Player>, MailboxError>,
    act: &mut ServerSession,
    ctx: &mut Context<ServerSession>) {
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
