use protocol::buffer::Buffer;
use session::ServerSession;
use handler::req::navigator::RoomCategories;
use actix::{Context, Addr};
use handler::req::navigator::InitialiseNavigator;
use futures::sink::Sink;

pub fn room_categories_handler(buf: &mut Buffer, session: &mut ServerSession, _: &mut Context<ServerSession>) {
    if let Some(ref ctx) = session.player {
        ctx.addr.do_send(RoomCategories);
    }
}

pub fn initialise_handler(buf: &mut Buffer, session: &mut ServerSession, _: &mut Context<ServerSession>) {
    if let Some(ref ctx) = session.player {
        ctx.addr.do_send(InitialiseNavigator);
    }
}
