use core::{Component, Container};
use model::player::PlayerFriend;
use player::Player;
use std::sync::Arc;

struct MessengerComponent {
    friends: Vec<PlayerFriend>
}

impl Component for MessengerComponent {}

pub trait Messenger {
    fn friends(&self) -> Vec<PlayerFriend>;
}

impl Messenger for MessengerComponent {
    fn friends(&self) -> Vec<PlayerFriend> {
        self.friends.clone()
    }
}

impl Messenger for Player {
    fn friends(&self) -> Vec<PlayerFriend> {
        self.component::<MessengerComponent>().friends()
    }
}