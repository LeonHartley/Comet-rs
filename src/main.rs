extern crate actix;
extern crate chrono;
extern crate clap;
extern crate config;
extern crate conv;
extern crate db;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate log;
extern crate model;
extern crate mysql;
extern crate server;

use actix::SyncArbiter;
use chrono::Local;
use clap::Arg;
use conv::*;
use db::ctx::DbContext;
use env_logger::Builder;
use log::LevelFilter;
use model::config::Config;
use mysql::Pool;
use server::core::Server;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::io::Write;
use std::mem::transmute;
use std::sync::Arc;

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

    let db = SyncArbiter::start(config.database.executors, move || DbContext(pool.clone()));

    kek();

    Server::new(&config.game)
        .start(db);

    info!(target: "boot", "Comet is starting");

    let _ = system.run();
}

type ComponentSet = HashMap<TypeId, Arc<Component>>;

struct GameCtx {
    components: ComponentSet
}

trait Container {
    fn components(&self) -> &ComponentSet;
    fn components_mut(&mut self) -> &mut ComponentSet;

    fn add_component(&mut self, component: Arc<Component>) {
        self.components_mut()
            .insert(component.typeid(), component);
    }

    fn component<T: 'static>(&self) -> Arc<T> where T: Component {
        let component = Box::new(self
            .components()
            .get(&TypeId::of::<T>())
            .expect("Components are not configured!")
            .clone());

        unsafe {
            *transmute::<Box<Arc<Component>>, Box<Arc<T>>>(component)
        }
    }
}

impl Container for GameCtx {
    fn components(&self) -> &ComponentSet {
        &self.components
    }

    fn components_mut(&mut self) -> &mut ComponentSet {
        &mut self.components
    }
}

trait Component: Any {
    fn typeid(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn kek(&self) -> String;
}

struct Players {
    top_kek: String
}

struct Rooms;

impl Players {
    fn top_kek(&self) {
        println!("{}", self.top_kek);
    }
}

impl Rooms {
    fn rooms(&self) {
        println!("getting rooms");
    }
}

impl Component for Players {
    fn kek(&self) -> String {
        String::from("loool")
    }
}

impl Component for Rooms {
    fn kek(&self) -> String {
        String::from("rewms")
    }
}

pub fn kek() {
    let mut ctx = GameCtx {
        components: ComponentSet::new()
    };

    ctx.add_component(Arc::new(Players {
        top_kek: String::from("lol ;d")
    }));

    ctx.add_component(Arc::new(Rooms));

    let players = ctx.component::<Players>();
    let rooms = ctx.component::<Rooms>();

    println!("players {}", players.kek());
    println!("rooms {}", rooms.kek());
    players.top_kek();
    rooms.rooms();

    println!("eof");
}

