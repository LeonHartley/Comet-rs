use std::collections::HashMap;
use protocol::buffer::Buffer;
use session::ServerSession;
use actix::Context;
use actix::Addr;

mod req;
mod handshake;

type HandlerFunc = Fn(&mut Buffer, Addr<ServerSession>);
type HandlerMap = HashMap<i16, Box<HandlerFunc>>;

const CLIENT_VERSION_EVENT: i16 = 4000;
const SSO_TICKET_EVENT: i16 = 286;

pub struct MessageHandler {
    handlers: HandlerMap
}

impl MessageHandler {
    pub fn new() -> MessageHandler {
        MessageHandler {
            handlers: register_message_handlers(HashMap::new())
        }
    }

    pub fn handle(&self, header: i16, buffer: &mut Buffer, session: Addr<ServerSession>) {
        let handler = match self.handlers.get(&header) {
            Some(handler) => handler.as_ref(),
            None => {
                debug!(target: "io", "unhandled msg {}", header);
                return;
            }
        };

        handler(buffer, session);
    }
}

fn register_message_handlers(mut map: HandlerMap) -> HandlerMap {
    map.insert(CLIENT_VERSION_EVENT, Box::new(handshake::client_version_handler));
    map.insert(SSO_TICKET_EVENT, Box::new(handshake::authentication_handler));

    map
}
