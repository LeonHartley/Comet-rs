extern crate db;
extern crate model;
extern crate clap;
extern crate config;
extern crate server;

#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;

use clap::Arg;
use db::ctx::DbContext;
use std::io::Write;
use model::config::Config;
use env_logger::Builder;
use log::LevelFilter;
use chrono::Local;
use server::Server;

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

    Server::new(&config.game)
        .start();

    debug!(target: "boot", "Comet is running");
}