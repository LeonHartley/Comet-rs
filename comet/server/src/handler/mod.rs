use std::collections::HashMap;
use protocol::buffer::Buffer;
use session::ServerSession;

mod handshake;

type HandlerFunc = Fn(&mut Buffer, &ServerSession);
type HandlerMap = HashMap<i16, Box<HandlerFunc>>;

const CLIENT_VERSION_EVENT: i16 = 4000;

pub struct MessageHandler {
    handlers: HandlerMap
}

impl MessageHandler {
    fn new() -> MessageHandler {
        MessageHandler {
            handlers: register_message_handlers(HashMap::new())
        }
    }

    fn handle(&self, header: i16, buffer: &mut Buffer, session: &ServerSession) {
        let handler = self.handlers.get(&header)
            .unwrap()
            .as_ref();

        handler(buffer, session);
    }
}

fn register_message_handlers(mut map: HandlerMap) -> HandlerMap {
    map.insert(CLIENT_VERSION_EVENT, Box::new(handshake::client_version_handler));

    map
}
