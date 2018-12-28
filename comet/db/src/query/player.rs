use actix::{Context, Handler, Message};
use actix::SyncContext;
use ctx::DbContext;
use Error;
use model::player::Player;
use std::option::Option;

pub struct PlayerByLoginTicket(pub String);

impl Message for PlayerByLoginTicket {
    type Result = Option<Player>;
}

impl Handler<PlayerByLoginTicket> for DbContext {
    type Result = Option<Player>;

    fn handle(&mut self, msg: PlayerByLoginTicket, ctx: &mut SyncContext<Self>) -> Self::Result {
        println!("Getting player data by ticket: {}", msg.0);

        let mut r: Result<Vec<Player>, _> = self.0.prep_exec("SELECT id, username AS name, figure, motto
        FROM players WHERE auth_ticket = :ticket;", params! {
            "ticket" => msg.0,
        }).map(|res| {
            res.map(|x| x.unwrap()).map(|row| {
                let (id, name, figure, motto) = mysql::from_row(row);

                Player {
                    id,
                    name,
                    figure,
                    motto,
                }
            }).collect()
        });

        if let Ok(players) = r {
            players
                .into_iter()
                .next()
        } else {
            None
        }
    }
}