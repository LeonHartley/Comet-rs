use actix::{Handler, Message};
use actix::SyncContext;
use ctx::DbContext;
use model::player::{Player, PlayerAvatar, PlayerBalance};
use model::player::messenger::PlayerFriend;
use model::player::settings::MessengerSettings;
use model::player::settings::NavigatorSettings;
use model::player::settings::PlayerSettings;
use mysql::{FromRowError, Row};
use mysql::prelude::FromRow;
use query::DbQueryExecutor;
use std::option::Option;
use std::time::Instant;

trait PlayerRepository {
    fn player_by_ticket(&mut self, ticket: String) -> Option<Player>;

    fn player_friends(&mut self, player_id: i64) -> Option<Vec<PlayerFriend>>;
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
    allow_trade: bool,
    player_invisible: bool,
    player_invisible_room: bool,
    home_room: i64,
    old_chat_style: bool,
    allow_follow: bool,
    allow_friend_requests: bool,
    navigator_x: i32,
    navigator_y: i32,
    navigator_height: i32,
    navigator_width: i32,
    navigator_searches_visible: bool,
}

impl FromRow for PlayerQueryResult {
    fn from_row(row: Row) -> Self {
        FromRow::from_row_opt(row)
            .ok()
            .expect("failed to deserialize row")
    }

    fn from_row_opt(mut row: Row) -> Result<Self, FromRowError> where
        Self: Sized {
        Ok(PlayerQueryResult {
            id: row.take("id").unwrap(),
            name: row.take("name").unwrap(),
            figure: row.take("figure").unwrap(),
            motto: row.take("motto").unwrap(),
            gender: row.take("gender").unwrap(),
            credits: row.take("credits").unwrap(),
            vip_points: row.take("vip_points").unwrap(),
            seasonal_points: row.take("seasonal_points").unwrap(),
            activity_points: row.take("activity_points").unwrap(),
            rank: row.take("rank").unwrap(),
            achievement_points: row.take("achievement_points").unwrap(),
            allow_trade: row.take("allow_trade").unwrap(),
            player_invisible: row.take("player_invisible").unwrap(),
            player_invisible_room: row.take("player_invisible_room").unwrap(),
            home_room: row.take("home_room").unwrap(),
            old_chat_style: row.take("old_chat_style").unwrap(),
            allow_follow: row.take("allow_follow").unwrap(),
            allow_friend_requests: row.take("allow_friend_requests").unwrap(),
            navigator_x: row.take("navigator_x").unwrap(),
            navigator_y: row.take("navigator_y").unwrap(),
            navigator_height: row.take("navigator_height").unwrap(),
            navigator_width: row.take("navigator_width").unwrap(),
            navigator_searches_visible: row.take("navigator_searches_visible").unwrap(),
        })
    }
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
            settings: PlayerSettings {
                allow_trade: self.allow_trade,
                player_invisible: self.player_invisible,
                player_invisible_room: self.player_invisible_room,
                home_room: self.home_room,
                old_chat_style: self.old_chat_style,
                messenger: MessengerSettings {
                    allow_follow: self.allow_follow,
                    allow_friend_requests: self.allow_friend_requests,
                },
                navigator: NavigatorSettings {
                    x: self.navigator_x,
                    y: self.navigator_y,
                    width: self.navigator_width,
                    height: self.navigator_height,
                    searches_visible: self.navigator_searches_visible,
                },
            },
        }
    }
}

struct PlayerFriendResult {
    avatar: PlayerAvatarResult,
    level: i16,
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

impl Into<PlayerFriend> for PlayerFriendResult {
    fn into(self) -> PlayerFriend {
        PlayerFriend {
            avatar: self.avatar.into(),
            level: self.level,
        }
    }
}

impl PlayerRepository for DbContext {
    fn player_by_ticket(&mut self, ticket: String) -> Option<Player> {
        self.exec_select(r"
            SELECT
                id, username AS name, figure, motto, gender, credits,
                vip_points, seasonal_points, activity_points, `rank`, achievement_points,
                s.allow_trade, s.hide_online AS player_invisible, s.hide_inroom AS player_invisible_room,
                s.home_room, s.chat_oldstyle AS old_chat_style, s.allow_follow, s.allow_friend_requests,
                s.navigator_x, s.navigator_y, s.navigator_height, s.navigator_width,
                s.navigator_show_searches AS navigator_searches_visible
            FROM players p
            JOIN player_settings s ON s.player_id = p.id
            WHERE auth_ticket = :ticket;", params! {"ticket" => ticket}, |row| {
            mysql::from_row::<PlayerQueryResult>(row).into()
        }).and_then(|p| p.into_iter().next())
    }

    fn player_friends(&mut self, player_id: i64) -> Option<Vec<PlayerFriend>> {
        self.exec_select(r"
            SELECT
                p.id, p.username as name, p.figure, p.motto, p.gender, f.friendship_level as level
            FROM messenger_friendships f
            JOIN players p ON p.id = f.user_two_id
            WHERE f.user_one_id = :player_id", params! { "player_id" => player_id }, |row| {
            let (id, name, figure, motto, gender, level) = mysql::from_row(row);
            PlayerFriendResult { level, avatar: PlayerAvatarResult { id, name, figure, motto, gender } }.into()
        })
    }
}

impl Handler<PlayerByLoginTicket> for DbContext {
    type Result = Option<Player>;

    fn handle(&mut self, msg: PlayerByLoginTicket, _ctx: &mut SyncContext<Self>) -> Self::Result {
        let time = Instant::now();
        let mut player = match self.player_by_ticket(msg.0) {
            Some(player) => player,
            None => return None
        };

        player.friends = match self.player_friends(player.avatar.id) {
            Some(friends) => friends,
            None => return None
        };

        debug!("Loaded player data in {}ms", time.elapsed().as_millis());
        Some(player)
    }
}