#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use serde::Serialize;
//use std::fs::File;
//use std::io::Read;

mod db;
use db::*;
