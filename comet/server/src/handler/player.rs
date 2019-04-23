use actix::{Context, Addr};
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

pub fn info_retrieve(_: &mut Buffer, session: &mut ServerSession, _: &mut Context<ServerSession>) {
    let p = session.player().clone();
    let player = p.read().unwrap();

    let _ = session.compose_all(vec![
        credits_composer(player.balance.credits),
        messenger_config_composer(),
        points_balance_composer(&player.balance),
        achievement_points_composer(player.achievement_points),
        player_info_composer(&player.avatar),
        home_room_composer(player.settings.home_room)
    ]);
}
