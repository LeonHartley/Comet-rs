use actix::Addr;
use protocol::buffer::Buffer;
use session::ServerSession;
use handler::req::player::InfoRetrieve;

pub fn info_retrieve(buffer: &mut Buffer, session: Addr<ServerSession>) {
    session.do_send(InfoRetrieve);
}