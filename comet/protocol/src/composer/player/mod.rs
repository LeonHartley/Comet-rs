use buffer::Buffer;
use model::player::Player;

pub fn player_info_composer(player: &Player) -> Buffer {
    Buffer::empty(1513)
        .write_i32(player.id as i32)
        .write_string(&player.name)
        .write_string(&player.figure)
        .write_string(&player.gender.into())
        .write_string(&player.motto)
        .write_string(&player.name.to_lowercase())
        .write_bool(true)
        .write_i32(8)
        .write_i32(3)
        .write_i32(3)
        .write_bool(true)
        .write_string(&format!("{}", 2018)) // last activity
        .write_bool(false) // can change username
        .write_bool(false)
}

pub fn credits_composer(credits: i32) -> Buffer {
    Buffer::empty(1556)
        .write_string(&format!("{}.0", credits))
}