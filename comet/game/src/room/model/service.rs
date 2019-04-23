use container::Component;
use container::Container;
use ctx::GameContext;
use db::ctx::DbContext;
use db::query::room::model::RoomModelRepository;
use model::room::map::Pos;
use model::room::map::RoomModel;
use room::model::parser::ModelParser;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub trait ModelService {
    fn get_room_model(&self, id: String) -> Option<Arc<RoomModel>>;
    fn reload_models(&self) {}
}

pub struct ModelServiceContext {
    models: RwLock<HashMap<String, Arc<RoomModel>>>
}

impl Component for ModelServiceContext {
    fn registered(&self) {
        info!("Loaded {} room models", self.models.read().expect("models lock").len());
    }
}

impl ModelServiceContext {
    pub fn new(mut db: DbContext) -> ModelServiceContext {
        let models = RwLock::new(db.get_room_models()
            .expect("load room models")
            .into_iter()
            .map(|m| (m.id, Arc::new(m.model_data.into_tile_map(Pos { x: m.door_x, y: m.door_y, z: m.door_z }))))
            .collect());

        ModelServiceContext { models }
    }
}

impl ModelService for ModelServiceContext {
    fn get_room_model(&self, id: String) -> Option<Arc<RoomModel>> {
        self.models
            .read()
            .expect("models lock")
            .get(&id)
            .cloned()
    }
}

impl ModelService for GameContext {
    fn get_room_model(&self, id: String) -> Option<Arc<RoomModel>> {
        self.component::<ModelServiceContext>()
            .get_room_model(id)
    }
}
