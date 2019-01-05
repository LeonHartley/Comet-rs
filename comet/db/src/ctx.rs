use actix::{Actor, SyncContext};
use mysql;

pub struct DbContext(pub mysql::Pool);

pub trait DbQueryExecutor {
    fn exec_select<T, Q, F, P>(&mut self, query: Q, params: P, reader: F) -> Option<Vec<T>>
        where Q: AsRef<str>, F: Fn(mysql::Row) -> T, P: Into<mysql::Params>;
}

impl Actor for DbContext {
    type Context = SyncContext<Self>;
}

impl DbContext {
    pub fn pool(&mut self) -> mysql::Pool {
        self.0.clone()
    }
}

impl DbQueryExecutor for DbContext {
    fn exec_select<T, Q, F, P>(&mut self, query: Q, params: P, reader: F) -> Option<Vec<T>>
        where Q: AsRef<str>, F: Fn(mysql::Row) -> T, P: Into<mysql::Params> {
        Some(self.pool().prep_exec(query, params).map(|res| {
            res.map(|x| x.unwrap()).map(reader).collect()
        }).expect("Failed to select"))
    }
}

impl Clone for DbContext {
    fn clone(&self) -> Self {
        DbContext(self.0.clone())
    }
}