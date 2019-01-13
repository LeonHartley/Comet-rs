use ctx::DbContext;
use model::room::map::RoomModel;
use query::DbQueryExecutor;

pub trait RoomModelRepository {
    fn get_room_models(&mut self) -> Option<Vec<ModelQueryResult>>;
}

pub struct ModelQueryResult {
    pub id: String,
    pub door_x: i32,
    pub door_y: i32,
    pub door_z: f64,
    pub door_dir: i32,
    pub model_data: String,
}

impl RoomModelRepository for DbContext {
    fn get_room_models(&mut self) -> Option<Vec<ModelQueryResult>> {
        self.exec_select(r"
        SELECT
            id, door_x, door_y, door_z, door_dir, heightmap as model_data
        FROM room_models;", (), |row| {
            let (id, door_x, door_y, door_z, door_dir, model_data) = mysql::from_row(row);
            ModelQueryResult { id, door_x, door_y, door_z, door_dir, model_data }
        })
    }
}