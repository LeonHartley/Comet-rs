use ctx::DbContext;
use actix::{Handler, Message, Context};
use model::player::Player;
use std::option::Option;
use actix::SyncContext;
use Error;

pub struct PlayerByLoginTicket(pub String);

impl Message for PlayerByLoginTicket {
    type Result = Result<Option<Player>, Error>;
}

impl Handler<PlayerByLoginTicket> for DbContext {
    type Result = Result<Option<Player>, Error>;

    fn handle(&mut self, msg: PlayerByLoginTicket, ctx: &mut SyncContext<Self>) -> <Self as Handler<PlayerByLoginTicket>>::Result {
        println!("Getting player data by ticket: {}", msg.0);

        Ok(Some(Player {
            id: 0,
            name: String::from("yo"),
            figure: String::new(),
            motto: String::new(),
        }))
    }
}