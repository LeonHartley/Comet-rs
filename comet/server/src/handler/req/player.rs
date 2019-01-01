use actix::{Context, Handler, Message};
use protocol::composer::player::{credits_composer, player_info_composer};
use protocol::composer::player::messenger::messenger_config_composer;
use session::ServerSession;

#[derive(Message)]
pub struct InfoRetrieve;

impl Handler<InfoRetrieve> for ServerSession {
    type Result = ();

    fn handle(&mut self, _: InfoRetrieve, _: &mut Context<Self>) {
        let player = match self.player_data() {
            Some(p) => p,
            _ => return
        };

        let _ = self.compose_all(vec![
            credits_composer(player.balance.credits),
            messenger_config_composer(),
            player_info_composer(player.as_ref())
        ]);
    }
}