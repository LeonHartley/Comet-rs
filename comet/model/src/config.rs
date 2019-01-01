use serde;
#[macro_use]
use serde_derive;

#[derive(Debug, Deserialize)]
pub struct Logging {
    pub date_fmt: String,
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub connection_string: String,
    pub executors: usize,
}

#[derive(Debug, Deserialize)]
pub struct Game {
    pub host: String,
    pub port: i16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: Database,
    pub game: Game,
    pub logging: Logging,
}
