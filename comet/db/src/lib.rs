extern crate mysql;
extern crate actix;
extern crate model;

pub mod ctx;
pub mod query;

pub struct Error(String);