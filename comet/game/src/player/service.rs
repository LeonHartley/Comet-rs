use container::{Component, Container};
use core::borrow::BorrowMut;
use ctx::GameContext;
use player::Player;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

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
    fn is_player_online(&self, player_id: i64) -> bool {
        self.component::<PlayerServiceContext>()
            .is_online(player_id)
    }


    fn on_player_online(&mut self, player: &Player) {
        self.component_mut::<PlayerServiceContext>()
            .on_player_online(&player)
    }
}