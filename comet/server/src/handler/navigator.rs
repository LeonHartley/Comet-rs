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
//    let data = match &session.player {
//        Some(ctx) => match ctx.data.read() {
//            Ok(data) => data.clone(),
//            Err(_) => return
//        },
//        None => return
//    };
//
//    let game = session.game.clone();
//
//    let _ = session.compose(
//        room_categories_composer(game.get_room_categories(), data.rank));
}


pub fn initialise_handler(buf: &mut Buffer, session: &mut ServerSession, context: &mut Context<ServerSession>) {
//    if let Some(ctx) = &session.player {
//        if let Ok(player) = ctx.data.read() {
//            let _ = session.compose_all(vec![
//                navigator_settings_composer(&player.settings.navigator),
//                navigator_metadata_composer()]);
//        }
//    }
}
