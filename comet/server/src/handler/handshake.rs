use protocol::buffer::Buffer;
use session::ServerSession;
use protocol::composer::handshake::motd_composer;
use actix::Addr;
use session::ComposeMessage;
use protocol::composer::handshake::auth_ok_composer;

pub fn client_version_handler(buf: &mut Buffer, _: Addr<ServerSession>) {
    match buf.read_string() {
        Some(s) => println!("client version: {}", s),
        _ => return
    };
}

pub fn authentication_handler(buf: &mut Buffer, session: Addr<ServerSession>) {
    match buf.read_string() {
        Some(s) => {
            session.do_send(ComposeMessage(auth_ok_composer()));
            session.do_send(ComposeMessage(motd_composer(format!("got ticket: {}", s))));
        }
        _ => return
    };
}