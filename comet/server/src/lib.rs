#![feature(duration_as_u128)]

extern crate model;
extern crate db;
extern crate actix;
extern crate protocol;
extern crate bytes;
extern crate byteorder;
extern crate tokio_io;
extern crate actix_web;
extern crate tokio_tcp;
extern crate futures;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate game;

pub mod codec;

pub mod handler;
pub mod session;
pub mod core;
pub mod tcp;