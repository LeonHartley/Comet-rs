extern crate actix;
extern crate chrono;
extern crate clap;
extern crate config;
extern crate conv;
extern crate db;
extern crate env_logger;
extern crate futures;
extern crate game;
#[macro_use]
extern crate log;
extern crate model;
extern crate mysql;
extern crate server;

use actix::SyncArbiter;
use chrono::Local;
use clap::Arg;
use db::ctx::DbContext;
use env_logger::Builder;
use game::ctx::GameContext;
use game::player::Logout;
use game::room::model::parser::ModelParser;
use log::LevelFilter;
use model::config::Config;
use model::room::map::Pos;
use mysql::Pool;
use server::core::Server;
use std::any::TypeId;
use std::io::Write;
use std::sync::Arc;

pub fn console() {
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
                     "{} [{}] {} - {}",
                     Local::now().format(&date_fmt),
                     record.level(),
                     record.target(),
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

    info!("Comet is starting");

    let system = actix::System::new("comet-server");

    let pool = Pool::new({ config.database.connection_string }).unwrap();

    let cloned_pool = pool.clone();
    let db = SyncArbiter::start(config.database.executors, move || DbContext(cloned_pool.clone()));

    Server::start(
        format!("{}:{}", config.game.host, config.game.port), db,
        Arc::new(
            GameContext::new()
                .init(DbContext(pool.clone()))
        ),
    );

    println!("this is run");
    let _ = system.run();
}
