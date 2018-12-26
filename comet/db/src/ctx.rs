use mysql;
use actix::{Actor, SyncContext};

pub struct DbContext(pub mysql::Pool);

impl Actor for DbContext {
    type Context = SyncContext<Self>;
}