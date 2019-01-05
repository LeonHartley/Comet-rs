use protocol::buffer::Buffer;
use session::ServerSession;
use handler::req::navigator::RoomCategories;
use actix::Addr;

pub fn room_categories_handler(buf: &mut Buffer, session: Addr<ServerSession>) {
    session.do_send(RoomCategories);
}
