extern crate db;
extern crate model;
extern crate server;

#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;
extern crate config;
extern crate clap;
extern crate actix;
extern crate mysql;
extern crate futures;

use clap::Arg;
use std::io::Write;
use model::config::Config;
use env_logger::Builder;
use log::LevelFilter;
use chrono::Local;
use server::core::Server;
use mysql::Pool;
use actix::SyncArbiter;
use db::ctx::DbContext;
use db::Error;

pub fn main() {
    let matches = clap::App::new("Comet Server")
        .arg(
            Arg::with_name("config_profile")
                .takes_value(true)
                .value_name("CONFIG PROFILE")
                .index(1)
                .required(true),
        ).get_matches();

    let mut settings = config::Config::default();

    settings
        .merge(config::File::with_name(matches.value_of("config_profile").unwrap()))
        .unwrap();

    let config = settings
        .try_into::<Config>()
        .unwrap();

    let date_fmt = config.logging.date_fmt.clone();

    Builder::new()
        .format(move |buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format(&date_fmt),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, match config.logging.level.as_ref() {
            "Info" => LevelFilter::Info,
            "Error" => LevelFilter::Error,
            "Debug" => LevelFilter::Debug,
            "Warn" => LevelFilter::Warn,

            _ => LevelFilter::Trace
        })
        .init();

    let system = actix::System::new("comet-server");

    let pool = Pool::new({ config.database.connection_string }).unwrap();

    let db = SyncArbiter::start(2, move || DbContext(pool.clone()));

    Server::new(&config.game)
        .start(db);

    info!(target: "boot", "Comet is starting");

    let _ = system.run();
}