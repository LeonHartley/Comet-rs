#![feature(duration_as_u128)]

extern crate actix;
#[macro_use]
extern crate log;
extern crate model;
#[macro_use]
extern crate mysql;

pub mod ctx;
pub mod query;

#[derive(Debug)]
pub struct Error(String);