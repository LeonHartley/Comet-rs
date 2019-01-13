use std::sync::{Arc, Mutex};

use container::{ComponentSet, Container};
use db::ctx::DbContext;
use navigator::service::NavigatorServiceContext;
use player::service::PlayerServiceContext;
use room::model::service::ModelServiceContext;

pub struct GameContext {
    components: ComponentSet
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext { components: ComponentSet::new() }
    }

    pub fn init(mut self, db: DbContext) -> GameContext {
        self.add_component(PlayerServiceContext::new(db.clone()));
        self.add_component(NavigatorServiceContext::new(db.clone()));
        self.add_component(ModelServiceContext::new(db));

        self
    }
}

impl Container for GameContext {
    fn components(&self) -> &ComponentSet { &self.components }

    fn components_mut(&mut self) -> &mut ComponentSet { &mut self.components }
}