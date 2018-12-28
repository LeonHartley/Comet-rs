use mysql;
use actix::{Actor, SyncContext};

pub struct DbContext(pub mysql::Pool);

impl Actor for DbContext {
    type Context = SyncContext<Self>;
}

impl DbContext {
    pub fn pool(&mut self) -> mysql::Pool {
        self.0.clone()
    }
}