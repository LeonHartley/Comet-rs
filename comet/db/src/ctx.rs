use actix::{Actor, SyncContext};
use mysql;

pub struct DbContext(pub mysql::Pool);

impl Actor for DbContext {
    type Context = SyncContext<Self>;
}

impl DbContext {
    pub fn pool(&mut self) -> mysql::Pool {
        self.0.clone()
    }
}

impl Clone for DbContext {
    fn clone(&self) -> Self {
        DbContext(self.0.clone())
    }
}