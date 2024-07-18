#[macro_use]
extern crate schemars;

pub use iso8601_timestamp::Timestamp;
pub use sea_orm;

pub mod util;

pub use util::result::{Error, Result};
