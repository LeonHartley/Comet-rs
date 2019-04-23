use protocol::buffer::{Buffer, StreamMessage};
use session::ServerSession;
use actix::{Context, Addr, AsyncContext};
use game::navigator::service::NavigatorService;
use protocol::composer::navigator::{
    room_categories_composer,
    navigator_metadata_composer,
    navigator_settings_composer,
};

pub fn room_categories_handler(buf: &mut Buffer, session: &mut ServerSession, context: &mut Context<ServerSession>) {
    if let Some(ctx) = &session.player {
        if let Ok(player) = ctx.data.read() {
            let _ = context.address().do_send(StreamMessage::Send(
                room_categories_composer(session.game.get_room_categories(), player.rank)));
        }
    }
}

pub fn initialise_handler(buf: &mut Buffer, session: &mut ServerSession, context: &mut Context<ServerSession>) {
    if let Some(ctx) = &session.player {
        if let Ok(player) = ctx.data.read() {
            let _ = context.address().do_send(StreamMessage::BufferedSend(vec![
                navigator_settings_composer(&player.settings.navigator),
                navigator_metadata_composer()]));
        }
    }
}
