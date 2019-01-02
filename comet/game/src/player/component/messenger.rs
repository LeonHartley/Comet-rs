use core::{Component, Container};
use player::Player;
use model::player::PlayerAvatar;

pub struct Messenger {
    friends: Vec<PlayerAvatar>
}

impl Messenger {
    pub fn new(friends: Vec<PlayerAvatar>) -> Messenger {
        Messenger {
            friends
        }
    }
}

impl Component for Messenger {}

pub trait MessengerComponent {
    fn friends(&self) -> Vec<PlayerAvatar>;
}

impl MessengerComponent for Messenger {
    fn friends(&self) -> Vec<PlayerAvatar> {
        self.friends.clone()
    }
}

impl MessengerComponent for Player {
    fn friends(&self) -> Vec<PlayerAvatar> {
        self.component::<Messenger>().friends()
    }
}