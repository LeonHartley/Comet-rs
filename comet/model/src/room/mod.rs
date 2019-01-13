use std::collections::HashMap;

pub mod map;

#[derive(Debug)]
pub enum RoomType {
    Public,
    Private,
    Custom,
    /* ^ ;) */
}

#[derive(Debug)]
pub enum AccessType {
    Password,
    Doorbell,
    Open,
    Invisible,
}

#[derive(Debug)]
pub enum TradeStatus {
    Disabled,
    Enabled,
    OwnerOnly,
}

#[derive(Debug)]
pub struct Room {
    pub id: i64,
    pub room_type: RoomType,
    pub access_type: String,
    pub owner_id: i64,
    pub owner_name: String,
    pub group_id: Option<i64>,
    pub name: String,
    pub description: String,
    pub password: String,
    pub category_id: i32,
    pub score: i32,
    pub model_id: i32,
    pub max_players: i32,
    pub trade_status: TradeStatus,
    pub decorations: HashMap<String, String>,
}
