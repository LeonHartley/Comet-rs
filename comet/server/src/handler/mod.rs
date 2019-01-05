use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use actix::Addr;
use protocol::buffer::Buffer;
use session::ServerSession;

mod req;
mod handshake;
mod player;
mod navigator;

lazy_static! {
    static ref EVENT_NAMES: HashMap<i16, String> = {
        load_identifiers()
    };
}

type HandlerFunc = Fn(&mut Buffer, Addr<ServerSession>);
type HandlerMap = HashMap<i16, Box<HandlerFunc>>;

const CLIENT_VERSION_EVENT: i16 = 4000;
const SSO_TICKET_EVENT: i16 = 286;
const INFO_RETRIEVE_EVENT: i16 = 2401;

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
                if let Some(event) = EVENT_NAMES.get(&header) {
                    debug!("{} / {} unhandled - {:?}", &header, event, buffer.inner);
                } else {
                    debug!("{} unhandled - {:?}", &header, buffer.inner);
                }
                return;
            }
        };

        handler(buffer, session);
    }
}

fn register_message_handlers(mut map: HandlerMap) -> HandlerMap {
    map.insert(CLIENT_VERSION_EVENT, Box::new(handshake::client_version_handler));
    map.insert(SSO_TICKET_EVENT, Box::new(handshake::authentication_handler));
    map.insert(INFO_RETRIEVE_EVENT, Box::new(player::info_retrieve));

    map
}

fn load_identifiers() -> HashMap<i16, String> {
    if let Ok(mut file) = File::open("dev/event_id.json") {
        let mut data = String::new();

        match file.read_to_string(&mut data) {
            Ok(_) =>
                match serde_json::from_str(&data) {
                    Ok(data) => data,
                    _ => HashMap::new()
                },
            _ => HashMap::new()
        }
    } else {
        debug!("Event name resolution is disabled, dev/event_id.json does not exist or has invalid (non-JSON) format");
        HashMap::new()
    }
}