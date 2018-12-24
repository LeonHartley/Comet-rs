use protocol::buffer::Buffer;
use session::ServerSession;

pub fn client_version_handler(buf: &mut Buffer, session: &ServerSession) {
    match buf.read_string() {
        Some(s) => println!("client version: {}", s),
        _ => return
    };
}

pub fn authentication_handler(buf: &mut Buffer, session: &ServerSession) {
    match buf.read_string() {
        Some(s) => println!("sso ticket: {}", s),
        _ => return
    };
}