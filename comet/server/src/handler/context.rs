use session::ServerSession;
use protocol::buffer::Buffer;
use protocol::id::*;
use actix::{Actor, Handler, Message, Addr, Context};

pub trait BufferParser<T: Actor> {
    fn parse(&mut self, actor: Addr<T>, buffer: &mut Buffer);
}

impl Handler<ClientVersionMessage> for ServerSession {
    type Result = ();

    fn handle(&mut self, message: ClientVersionMessage, ctx: &mut Context<Self>) {
        if let Some(version) = message.version {
            debug!("Client version: {}", version);
        }
    }
}

//impl Handler<RoomCategoriesMessage> for game::player::Player {
//    type Result = ();
//
//    fn handle(&mut self, message: RoomCategoriesMessage, ctx: &mut Context<Self>) {
//        info!("received room categories message");
//    }
//}

#[derive(Message)]
pub struct AuthenticateMessage {
    pub ticket: Option<String>
}

#[derive(Message)]
pub struct ClientVersionMessage {
    pub version: Option<String>
}

#[derive(Message)]
pub struct RoomCategoriesMessage;

#[derive(Message)]
pub struct InitNavigatorMessage;

#[derive(Message)]
pub struct InfoRetrieveMessage;

impl BufferParser<Self> for ServerSession {
    fn parse(&mut self, actor: Addr<ServerSession>, message: &mut Buffer) {
        match message.id {
            CLIENT_VERSION_EVENT => actor.do_send(ClientVersionMessage {
                version: message.read_string()
            }),

            SSO_TICKET_EVENT => actor.do_send(AuthenticateMessage {
                ticket: message.read_string()
            }),

            _ => match &self.player {
                Some(ref player) => match message.id {
                    INFO_RETRIEVE_EVENT => player.do_send(InfoRetrieveMessage),
                    ROOM_CATEGORIES_EVENT => player.do_send(RoomCategoriesMessage),
                    INIT_NAVIGATOR_EVENT => player.do_send(InitNavigatorMessage),
                    _ => return
                },
                None => return
            }
        }
    }
}
