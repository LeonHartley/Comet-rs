use protocol::buffer::{Buffer, StreamMessage};
use session::ServerSession;
use actix::{Context, Addr, AsyncContext, Handler, Message};
use game::navigator::service::NavigatorService;
use protocol::composer::navigator::{
    room_categories_composer,
    navigator_metadata_composer,
    navigator_settings_composer,
};
use handler::context::{RoomCategoriesMessage, InitNavigatorMessage};
use game::player::Player;

impl Handler<RoomCategoriesMessage> for Player {
    type Result = ();

    fn handle(&mut self, msg: RoomCategoriesMessage, ctx: &mut Context<Player>) {
        let rank = self.inner.rank;
        let categories = self.game.get_room_categories();

        self.compose(room_categories_composer(categories, rank))
    }
}

impl Handler<InitNavigatorMessage> for Player {
    type Result = ();

    fn handle(&mut self, msg: InitNavigatorMessage, ctx: &mut Context<Player>) {
        let settings = self.inner.settings.navigator.clone();
        self.compose_all(vec![
            navigator_settings_composer(settings),
            navigator_metadata_composer()]);
    }
}
