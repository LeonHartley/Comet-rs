use protocol::buffer::{Buffer, StreamMessage};
use session::ServerSession;
use protocol::composer::handshake::motd_composer;
use protocol::composer::handshake::auth_ok_composer;
use actix::Addr;

pub fn client_version_handler(buf: &mut Buffer, _: Addr<ServerSession>) {
    match buf.read_string() {
        Some(s) => println!("client version: {}", s),
        _ => return
    };
}

pub fn authentication_handler(buf: &mut Buffer, session: Addr<ServerSession>) {
    let ticket = match buf.read_string() {
        Some(s) => s,
        None => return
    };

    session.do_send(StreamMessage::BufferedSend(vec![
        auth_ok_composer(),
        motd_composer(format!("got ticket: {}", ticket))
    ]));
}