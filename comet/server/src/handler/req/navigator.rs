use actix::{Context, Handler, Message};
use game::player::Player;
use protocol::composer::player::{achievement_points_composer, credits_composer, messenger::messenger_config_composer, player_info_composer, points_balance_composer};
use session::ServerSession;


#[derive(Message)]
pub struct RoomCategories;

impl Handler<RoomCategories> for ServerSession {
    type Result = ();

    fn handle(&mut self, _: RoomCategories, _: &mut Context<Self>) {
        if let Some(ctx) = self.player {
            ctx.addr.do_send(RoomCategories);
        }
    }
}

impl Handler<RoomCategories> for Player {
    type Result = ();

    fn handle(&mut self, msg: RoomCategories, ctx: &mut Context<Self>) {

    }
}