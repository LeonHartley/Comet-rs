use actix::{Handler, Message};
use actix::SyncContext;
use ctx::DbContext;
use model::player::{Player, PlayerAvatar, PlayerBalance};
use query::DbQueryExecutor;
use std::option::Option;
use std::time::Instant;

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

struct PlayerAvatarResult {
    id: i64,
    name: String,
    figure: String,
    motto: String,
    gender: String,
}

impl Into<PlayerAvatar> for PlayerAvatarResult {
    fn into(self) -> PlayerAvatar {
        PlayerAvatar {
            id: self.id,
            name: self.name,
            figure: self.figure,
            motto: self.motto,
            gender: self.gender.into(),
        }
    }
}

impl PlayerRepository for DbContext {
    fn player_by_ticket(&mut self, ticket: String) -> Option<Player> {
        self.exec_select(r"
            SELECT
                id, username AS name, figure, motto, gender, credits,
                vip_points, seasonal_points, activity_points, `rank`, achievement_points
            FROM players
            WHERE auth_ticket = :ticket;", params! {"ticket" => ticket}, |row| {
            let (id, name, figure, motto, gender, credits, vip_points, seasonal_points, activity_points, rank, achievement_points) = mysql::from_row(row);
            PlayerQueryResult { id, name, figure, motto, gender, credits, vip_points, seasonal_points, activity_points, rank, achievement_points }.into()
        }).and_then(|p| p.into_iter().next())
    }

    fn player_friends(&mut self, player_id: i64) -> Option<Vec<PlayerAvatar>> {
        self.exec_select(r"
            SELECT
                p.id, p.username as name, p.figure, p.motto, p.gender
            FROM messenger_friendships f
            JOIN players p ON p.id = f.user_two_id
            WHERE f.user_one_id = :player_id", params! { "player_id" => player_id }, |row| {
            let (id, name, figure, motto, gender) = mysql::from_row(row);
            PlayerAvatarResult { id, name, figure, motto, gender }.into()
        })
    }
}

impl Handler<PlayerByLoginTicket> for DbContext {
    type Result = Option<Player>;

    fn handle(&mut self, msg: PlayerByLoginTicket, _ctx: &mut SyncContext<Self>) -> Self::Result {
        let time = Instant::now();
        let mut player = self.player_by_ticket(msg.0).or_else(|| { return None; });
        player.friends = match self.player_friends(player.avatar.id)
            .or_else(|| { return None; });

        debug!("Loaded player data in {}ms", time.elapsed().as_millis());
        Some(player)
    }
}