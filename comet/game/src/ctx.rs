use container::{ComponentSet, Container};
use player::service::PlayerServiceContext;
use std::sync::{Arc, Mutex};

pub struct GameContext {
    components: ComponentSet
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext { components: ComponentSet::new() }
    }

    pub fn init(mut self) -> GameContext {
        self.add_component(PlayerServiceContext::new());

        self
    }
}

impl Container for GameContext {
    fn components(&self) -> &ComponentSet { &self.components }

    fn components_mut(&mut self) -> &mut ComponentSet { &mut self.components }
}