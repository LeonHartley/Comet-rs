use player::Player;
use actix::Addr;
use model::room::map::Pos;

#[Message]
pub enum ItemEvent {
    Interact { player: Option<Addr<Player>>, wired: bool },
    Move { new_position: Pos }
}