use std::collections::HashMap;

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
pub struct RoomStatus {
    pub players_now: i32,
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
    pub model_id: String,
    pub max_players: i32,
    pub trade_status: TradeStatus,
    pub decorations: HashMap<String, String>,
    pub is_active: bool,
    pub status: Option<RoomStatus>,
}

impl From<String> for RoomType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_ref() {
            "public" => RoomType::Public,
            "custom" => RoomType::Custom,
            _ => RoomType::Private
        }
    }
}

impl From<String> for AccessType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_ref() {
            "password" => AccessType::Password,
            "doorbell" => AccessType::Doorbell,
            "invisible" => AccessType::Invisible,
            _ => AccessType::Open,
        }
    }
}

impl From<String> for TradeStatus {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_ref() {
            "enabled" => TradeStatus::Enabled,
            "owner" => TradeStatus::OwnerOnly,
            _ => TradeStatus::Disabled
        }
    }
}
