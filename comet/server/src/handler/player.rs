use actix::{Context, Addr, Message, Handler};
use protocol::buffer::Buffer;
use session::ServerSession;
use protocol::composer::navigator::home_room_composer;
use protocol::composer::player::{
    achievement_points_composer,
    credits_composer,
    messenger::messenger_config_composer,
    player_info_composer,
    points_balance_composer,
};
use game::player::Player;
use handler::context::InfoRetrieveMessage;

impl Handler<InfoRetrieveMessage> for Player {
    type Result = ();

    fn handle(&mut self, msg: InfoRetrieveMessage, ctx: &mut Context<Player>) {
        let player = self.inner.clone();

        self.compose_all(vec![
            credits_composer(player.balance.credits),
            messenger_config_composer(),
            points_balance_composer(&player.balance),
            achievement_points_composer(player.achievement_points),
            player_info_composer(&player.avatar),
            home_room_composer(player.settings.home_room)
        ]);
    }
}
