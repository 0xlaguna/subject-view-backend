#[macro_use]
extern crate schemars;
#[macro_use]
extern crate lazy_static;

pub use iso8601_timestamp::Timestamp;
pub use sea_orm;

pub mod util;

pub mod r#impl;

pub mod auth;

pub mod models;

pub mod derive;

pub use util::result::{Error, Result};
