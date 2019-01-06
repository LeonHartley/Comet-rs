use protocol::buffer::Buffer;
use session::ServerSession;
use handler::req::navigator::RoomCategories;
use actix::Addr;
use handler::req::navigator::InitialiseNavigator;

pub fn room_categories_handler(buf: &mut Buffer, session: Addr<ServerSession>) {
    session.do_send(RoomCategories);
}

pub fn initialise_handler(buf: &mut Buffer, session: Addr<ServerSession>) {
    session.do_send(InitialiseNavigator);
}