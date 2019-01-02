use std::collections::HashMap;
use std::sync::Mutex;

use container::{Component, Container};
use ctx::GameContext;
use player::Player;
use core::borrow::BorrowMut;
use std::sync::Arc;

pub trait PlayerService {
    fn is_online(&self, player_id: i64) -> bool;

    fn on_player_online(&mut self, player: &Player);
}

pub struct PlayerServiceContext {
    online_players: Mutex<HashMap<i64, String>>
}

impl Component for PlayerServiceContext {}

impl PlayerService for PlayerServiceContext {
    fn is_online(&self, player_id: i64) -> bool {
        self.online_players
            .lock()
            .expect("Failed to gain lock")
            .contains_key(&player_id)
    }

    fn on_player_online(&mut self, player: &Player) {
        println!("LOL");
    }
}

impl PlayerService for GameContext {
    fn is_online(&self, player_id: i64) -> bool {
        unimplemented!()
    }

    fn on_player_online(&mut self, player: &Player) {
        unimplemented!()
    }
}