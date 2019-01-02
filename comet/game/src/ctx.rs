use container::{ComponentSet, Container};

pub struct GameContext {
    components: ComponentSet
}

impl GameContext {
    pub fn new() -> GameContext {
        GameContext { components: ComponentSet::new() }
    }
}

impl Container for GameContext {
    fn components(&self) -> &ComponentSet { &self.components }

    fn components_mut(&mut self) -> &mut ComponentSet { &mut self.components }
}