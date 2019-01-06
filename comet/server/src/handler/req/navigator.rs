use actix::{Context, Handler, Message};
use game::navigator::service::NavigatorService;
use game::player::Player;
use protocol::buffer::StreamMessage;
use protocol::composer::navigator::room_categories_composer;
use session::ServerSession;

#[derive(Message)]
pub struct RoomCategories;

impl Handler<RoomCategories> for ServerSession {
    type Result = ();

    fn handle(&mut self, _: RoomCategories, _: &mut Context<Self>) {
        if let Some(ref ctx) = self.player {
            ctx.addr.do_send(RoomCategories);
        }
    }
}

impl Handler<RoomCategories> for Player {
    type Result = ();

    fn handle(&mut self, msg: RoomCategories, ctx: &mut Context<Self>) {
        if let Ok(player) = self.inner.read() {
            let _ = self.stream.do_send(StreamMessage::Send(
                room_categories_composer(self.game.get_room_categories(), player.rank)));
        }
    }
}