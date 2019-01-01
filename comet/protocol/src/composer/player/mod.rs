use buffer::Buffer;
use model::player::{Player, PlayerBalance};

pub mod rights;
pub mod messenger;

pub fn player_info_composer(player: &Player) -> Buffer {
    Buffer::empty(1513)
        .write_i32(player.id as i32)
        .write_str(&player.name)
        .write_str(&player.figure)
        .write_str(&player.gender.into())
        .write_str(&player.motto)
        .write_str(&player.name.to_lowercase())
        .write_bool(true)
        .write_i32(8)
        .write_i32(3)
        .write_i32(3)
        .write_bool(true)
        .write_str(&format!("{}", 2018)) // last activity
        .write_bool(false) // can change username
        .write_bool(false)
}

pub fn credits_composer(credits: i32) -> Buffer {
    Buffer::empty(1556)
        .write_str(&format!("{}.0", credits))
}

pub fn points_balance_composer(balance: &PlayerBalance) -> Buffer {
    let mut buf = Buffer::empty(3304);



    buf
}