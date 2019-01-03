use actix::Addr;
use container::{Component, Container};
use core::borrow::BorrowMut;
use ctx::GameContext;
use player::Player;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

pub trait PlayerService {
    fn is_player_online(&self, player_id: i64) -> bool;

    fn add_online_player(&self, player: Addr<Player>, id: i64, name: String);

    fn remove_online_player(&self, id: i64, name: String);
}

pub struct PlayerServiceContext {
    online_players: Mutex<OnlinePlayersMap>
}

struct OnlinePlayersMap {
    online_players_id: HashMap<i64, Addr<Player>>,
    online_players_name: HashMap<String, Addr<Player>>,
}

impl PlayerServiceContext {
    pub fn new() -> PlayerServiceContext {
        PlayerServiceContext {
            online_players: Mutex::new(OnlinePlayersMap {
                online_players_id: HashMap::new(),
                online_players_name: HashMap::new(),
            })
        }
    }
}

impl Component for PlayerServiceContext {}

impl PlayerService for PlayerServiceContext {
    fn is_player_online(&self, player_id: i64) -> bool {
        self.online_players
            .lock()
            .expect("Failed to gain lock")
            .online_players_id
            .contains_key(&player_id)
    }

    fn add_online_player(&self, player: Addr<Player>, id: i64, name: String) {
        let mut players = self.online_players
            .lock()
            .expect("Failed to gain lock");
        println!("Adding player {} {}", id, name);
        players.online_players_id.insert(id, player.clone());
        players.online_players_name.insert(name, player.clone());
    }

    fn remove_online_player(&self, id: i64, name: String) {
        let mut players = self.online_players
            .lock()
            .expect("Failed to gain lock");

        println!("Removing player {} {}", id, name);
        players.online_players_id.remove(&id);
        players.online_players_name.remove(&name);
    }
}

impl PlayerService for GameContext {
    fn is_player_online(&self, player_id: i64) -> bool {
        self.component::<PlayerServiceContext>()
            .is_player_online(player_id)
    }

    fn add_online_player(&self, player: Addr<Player>, id: i64, name: String) {
        self.component::<PlayerServiceContext>()
            .add_online_player(player, id, name)
    }

    fn remove_online_player(&self, id: i64, name: String) {
        self.component::<PlayerServiceContext>()
            .remove_online_player(id, name)
    }
}