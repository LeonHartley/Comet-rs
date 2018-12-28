#[macro_use]
extern crate mysql;
extern crate actix;
extern crate model;

pub mod ctx;
pub mod query;

#[derive(Debug)]
pub struct Error(String);