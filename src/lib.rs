#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod cfg;
mod db;
pub use db::*;
