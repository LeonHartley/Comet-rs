use ctx::DbContext;

pub mod player;
pub mod navigator;
pub mod room;

pub trait DbQueryExecutor {
    fn exec_select<T, Q, F, P>(&mut self, query: Q, params: P, reader: F) -> Option<Vec<T>>
        where Q: AsRef<str>, F: Fn(mysql::Row) -> T, P: Into<mysql::Params>;
}

impl DbQueryExecutor for DbContext {
    fn exec_select<T, Q, F, P>(&mut self, query: Q, params: P, reader: F) -> Option<Vec<T>>
        where Q: AsRef<str>, F: Fn(mysql::Row) -> T, P: Into<mysql::Params> {
        Some(self.pool().prep_exec(query, params).map(|res| {
            res.map(|x| x.unwrap()).map(reader).collect()
        }).expect("Failed to select"))
    }
}
