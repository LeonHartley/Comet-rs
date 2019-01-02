use core::{Component, Container};
use model::player::PlayerFriend;
use player::Player;

pub struct Messenger {
    friends: Vec<PlayerFriend>
}

impl Messenger {
    pub fn new(friends: Vec<PlayerFriend>) -> Messenger {
        Messenger {
            friends
        }
    }
}

impl Component for Messenger {}

pub trait MessengerComponent {
    fn friends(&self) -> Vec<PlayerFriend>;
}

impl MessengerComponent for Messenger {
    fn friends(&self) -> Vec<PlayerFriend> {
        self.friends.clone()
    }
}

impl MessengerComponent for Player {
    fn friends(&self) -> Vec<PlayerFriend> {
        self.component::<Messenger>().friends()
    }
}