extern crate db;
extern crate model;
extern crate clap;
extern crate config;

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
        .merge(config::File::with_name("Comet"))
        .unwrap();

    let config = settings
        .try_into::<Config>()
        .unwrap();

    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%Y-%m-%d %H:%M:%S"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    debug!(target: "boot", "Comet is running");

    println!("{:?}", config);
}