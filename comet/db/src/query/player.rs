use actix::{Context, Handler, Message};
use actix::SyncContext;
use ctx::DbContext;
use Error;
use model::player::{Player, PlayerBalance, PlayerGender};
use mysql::{FromRowError, Row};
use mysql::prelude::FromRow;
use std::option::Option;

pub struct PlayerByLoginTicket(pub String);

impl Message for PlayerByLoginTicket {
    type Result = Option<Player>;
}

struct PlayerQueryResult {
    id: i64,
    name: String,
    figure: String,
    motto: String,
    gender: String,
    credits: i32,
    vip_points: i32,
    seasonal_points: i32,
    activity_points: i32,
}

impl Into<Player> for PlayerQueryResult {
    fn into(self) -> Player {
        Player {
            id: self.id,
            name: self.name,
            figure: self.figure,
            motto: self.motto,
            gender: self.gender.into(),
            balance: PlayerBalance {
                credits: self.credits,
                vip_points: self.vip_points,
                seasonal_points: self.seasonal_points,
                activity_points: self.activity_points,
            },
        }
    }
}

impl Handler<PlayerByLoginTicket> for DbContext {
    type Result = Option<Player>;

    fn handle(&mut self, msg: PlayerByLoginTicket, ctx: &mut SyncContext<Self>) -> Self::Result {
        println!("Getting player data by ticket: {}", msg.0);

        let result: Result<Vec<Player>, _> = self.pool()
            .prep_exec("SELECT id, username AS name, figure, motto, gender, credits, vip_points, seasonal_points, activity_points
                              FROM players WHERE auth_ticket = :ticket;",
                       params! {
"ticket" => msg.0,
})
            .map(|res| {
                res.map(|x| x.unwrap()).map(|row| {
                    let (id, name, figure, motto, gender, credits, vip_points, seasonal_points, activity_points) = mysql::from_row(row);
                    PlayerQueryResult { id, name, figure, motto, gender, credits, vip_points, seasonal_points, activity_points }.into()
                }).collect()
            });

        if let Ok(players) = result {
            players
                .into_iter()
                .next()
        } else {
            None
        }
    }
}