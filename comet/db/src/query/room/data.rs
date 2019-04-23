use ctx::DbContext;
use query::DbQueryExecutor;
use std::collections::HashMap;
use model::room::data::{Room, RoomStatus, RoomType, TradeStatus, AccessType};
use mysql::{FromRowError, Row};
use mysql::prelude::FromRow;

pub trait RoomDataRepository {
    fn get_player_rooms(&mut self, player_id: i64) -> Option<Vec<RoomQueryResult>>;
}

pub struct RoomQueryResult {
    pub id: i64,
    pub room_type: String,
    pub owner_id: i64,
    pub owner_name: String,
    pub group_id: Option<i64>,
    pub name: String,
    pub description: String,
    pub password: String,
    pub category_id: i32,
    pub score: i32,
    pub model_id: String,
    pub max_players: i32,
    pub trade_status: String,
    pub access_type: String,
    pub decorations: String,
    pub is_active: bool,
    pub players_now: Option<i32>,
}

impl Into<Room> for RoomQueryResult {
    fn into(self) -> Room {
        Room {
            id: self.id,
            room_type: self.room_type.into(),
            owner_id: self.owner_id,
            owner_name: self.owner_name,
            group_id: self.group_id,
            name: self.name,
            description: self.description,
            access_type: self.access_type.into(),
            password: self.password,
            category_id: self.category_id,
            score: self.score,
            model_id: self.model_id,
            max_players: self.max_players,
            trade_status: self.trade_status.into(),
            decorations: HashMap::new(),
            is_active: self.is_active,
            status: if self.is_active {
                Some(RoomStatus {
                    players_now: self.players_now.unwrap()
                })
            } else { None },
        }
    }
}

impl FromRow for RoomQueryResult {
    fn from_row(row: Row) -> Self {
        FromRow::from_row_opt(row)
            .ok()
            .expect("failed to deserialize row")
    }

    fn from_row_opt(mut row: Row) -> Result<Self, FromRowError> where
        Self: Sized {
        Ok(RoomQueryResult {
            id: row.take("id").unwrap(),
            room_type: row.take("room_type").unwrap(),
            owner_id: row.take("owner_id").unwrap(),
            owner_name: row.take("owner_name").unwrap(),
            group_id: row.take("group_id").unwrap(),
            name: row.take("name").unwrap(),
            description: row.take("description").unwrap(),
            password: row.take("password").unwrap(),
            category_id: row.take("category_id").unwrap(),
            score: row.take("score").unwrap(),
            model_id: row.take("model_id").unwrap(),
            max_players: row.take("max_players").unwrap(),
            trade_status: row.take("trade_status").unwrap(),
            access_type: row.take("access_type").unwrap(),
            decorations: row.take("decorations").unwrap(),
            is_active: row.take("is_active").unwrap(),
            players_now: row.take("players_now").unwrap()
        })
    }
}

impl RoomDataRepository for DbContext {
    fn get_player_rooms(&mut self, player_id: i64) -> Option<Vec<RoomQueryResult>> {
        self.exec_select(r"
        SELECT
            id, type AS room_type, group_id, owner_id, owner AS owner_name, name, description, access_type, model AS model_id
            access_type, password,  category AS category_id, max_users AS max_players, score, trade_state AS trade_status, decorations, NULL AS players_now
        FROM rooms WHERE owner = :player_id;", params! {"player_id" => player_id }, |row| {
            mysql::from_row::<RoomQueryResult>(row).into()
        })
    }
}
