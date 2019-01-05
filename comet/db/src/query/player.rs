use actix::{Handler, Message};
use actix::SyncContext;
use ctx::DbContext;
use model::player::{Player, PlayerAvatar, PlayerBalance};
use query::DbQueryExecutor;
use std::option::Option;

trait PlayerRepository {
    fn player_by_ticket(&mut self, ticket: String) -> Option<Player>;

    fn player_friends(&mut self, player_id: i64) -> Option<Vec<PlayerAvatar>>;
}

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
    rank: i16,
    achievement_points: i32,

}

impl Into<Player> for PlayerQueryResult {
    fn into(self) -> Player {
        Player {
            avatar: PlayerAvatar {
                id: self.id,
                name: self.name,
                figure: self.figure,
                motto: self.motto,
                gender: self.gender.into(),
            },
            friends: vec![],
            rank: self.rank,
            achievement_points: self.achievement_points,
            balance: PlayerBalance {
                credits: self.credits,
                vip_points: self.vip_points,
                seasonal_points: self.seasonal_points,
                activity_points: self.activity_points,
            },
        }
    }
}

impl PlayerRepository for DbContext {
    fn player_by_ticket(&mut self, ticket: String) -> Option<Player> {
        let res = self.exec_select(r"
            SELECT
                id, username AS name, figure, motto, gender, credits,
                vip_points, seasonal_points, activity_points, `rank`, achievement_points
            FROM players
            WHERE auth_ticket = :ticket;", params! {"ticket" => ticket}, |row| {
            let (id, name, figure, motto, gender, credits, vip_points, seasonal_points, activity_points, rank, achievement_points) = mysql::from_row(row);
            PlayerQueryResult { id, name, figure, motto, gender, credits, vip_points, seasonal_points, activity_points, rank, achievement_points }.into()
        });

        if let Some(res) = res {
            res
                .into_iter()
                .next()
        } else {
            None
        }
    }

    fn player_friends(&mut self, player_id: i64) -> Option<Vec<PlayerAvatar>> {
        Some(vec![])
    }
}

impl Handler<PlayerByLoginTicket> for DbContext {
    type Result = Option<Player>;

    fn handle(&mut self, msg: PlayerByLoginTicket, _ctx: &mut SyncContext<Self>) -> Self::Result {
        if let Some(mut player) = self.player_by_ticket(msg.0) {
            if let Some(friends) = self.player_friends(player.avatar.id) {
                player.friends = friends
            }

            return Some(player);
        }
        None
    }
}