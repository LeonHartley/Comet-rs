//use actix::*;
//use protocol::buffer::Buffer;
//use session::ServerSession;
//use std::collections::HashMap;
//use std::fs::File;
//use std::io::Read;
//use std::time::Instant;
//
//mod handshake;
mod player;
mod navigator;
pub mod context;
pub mod handshake;
//
//lazy_static! {
//    static ref EVENT_NAMES: HashMap<i16, String> = {
//        load_identifiers()
//    };
//}
//
//type HandlerFunc = Fn(&mut Buffer, &mut ServerSession, &mut Context<ServerSession>);
//type HandlerMap = HashMap<i16, Box<HandlerFunc>>;
//
//pub struct MessageHandler {
//    handlers: HandlerMap
//}
//
//impl MessageHandler {
//    pub fn new() -> MessageHandler {
//        MessageHandler {
//            handlers: register_message_handlers(HashMap::new())
//        }
//    }
//
//    pub fn handle(&self, header: i16, buffer: &mut Buffer,
//                  session: &mut ServerSession, context: &mut Context<ServerSession>) {
//        let handler = match self.handlers.get(&header) {
//            Some(handler) => handler.as_ref(),
//            None => {
//                if let Some(event) = EVENT_NAMES.get(&header) {
//                    debug!("{} / {} unhandled - {:?}", &header, event, buffer.inner);
//                } else {
//                    debug!("{} unhandled - {:?}", &header, buffer.inner);
//                }
//                return;
//            }
//        };
//
//        let time = Instant::now();
//        handler(buffer, session, context);
//        debug!("{} / {} event handled in {} ms ", match EVENT_NAMES.get(&header) {
//            Some(h) => h.as_ref(),
//            _ => "Unknown"
//        }, header, time.elapsed().as_millis());
//    }
//}
//
//fn register_message_handlers(mut map: HandlerMap) -> HandlerMap {
//    message_handlers()
//        .into_iter()
//        .collect()
//}
//
//fn message_handlers() -> Vec<(i16, Box<HandlerFunc>)> {
//    vec![(protocol::id::CLIENT_VERSION_EVENT, Box::new(handshake::client_version_handler)),
//         (protocol::id::SSO_TICKET_EVENT, Box::new(handshake::authentication_handler)),
//         (protocol::id::INFO_RETRIEVE_EVENT, Box::new(player::info_retrieve)),
//         (protocol::id::ROOM_CATEGORIES_EVENT, Box::new(navigator::room_categories_handler)),
//         (protocol::id::INIT_NAVIGATOR_EVENT, Box::new(navigator::initialise_handler))]
//}
//
//fn load_identifiers() -> HashMap<i16, String> {
//    if let Ok(mut file) = File::open("dev/event_id.json") {
//        let mut data = String::new();
//
//        match file.read_to_string(&mut data) {
//            Ok(_) =>
//                match serde_json::from_str(&data) {
//                    Ok(data) => data,
//                    _ => HashMap::new()
//                },
//            _ => HashMap::new()
//        }
//    } else {
//        debug!("Event name resolution is disabled, dev/event_id.json does not exist or has invalid (non-JSON) format");
//        HashMap::new()
//    }
//}
