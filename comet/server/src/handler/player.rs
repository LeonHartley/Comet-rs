use actix::Addr;
use handler::req::player::InfoRetrieve;
use protocol::buffer::Buffer;
use session::ServerSession;

pub fn info_retrieve(_: &mut Buffer, session: Addr<ServerSession>) {
    session.do_send(InfoRetrieve);
}