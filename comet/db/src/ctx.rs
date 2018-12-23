use mysql;

pub struct DbContext {
    pub conn: mysql::Pool
}

impl DbContext {
    pub fn new(conn_string: &str) -> DbContext {
        DbContext {
            conn: mysql::Pool::new({ conn_string }).unwrap()
        }
    }
}