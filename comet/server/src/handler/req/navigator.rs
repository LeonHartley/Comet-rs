use actix::{Context, Handler, Message};
use game::navigator::service::NavigatorService;
use game::player::Player;
use protocol::buffer::StreamMessage;
use protocol::composer::navigator::navigator_metadata_composer;
use protocol::composer::navigator::navigator_settings_composer;
use protocol::composer::navigator::room_categories_composer;
use session::ServerSession;
use actix::fut::WrapFuture;

#[derive(Message)]
pub struct RoomCategories;

#[derive(Message)]
pub struct InitialiseNavigator;

impl Handler<RoomCategories> for Player {
    type Result = ();

    fn handle(&mut self, msg: RoomCategories, ctx: &mut Context<Self>) {
        if let Ok(player) = self.inner.read() {
            let _ = self.stream.do_send(StreamMessage::Send(
                room_categories_composer(self.game.get_room_categories(), player.rank)));
        }
    }
}

impl Handler<InitialiseNavigator> for Player {
    type Result = ();

    fn handle(&mut self, msg: InitialiseNavigator, ctx: &mut Context<Self>) {
        if let Ok(player) = self.inner.read() {
            let _ = self.stream.do_send(StreamMessage::BufferedSend(vec![
                navigator_settings_composer(&player.settings.navigator),
                navigator_metadata_composer()]));
        }
    }
}
