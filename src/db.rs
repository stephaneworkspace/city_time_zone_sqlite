pub mod models;
pub mod repos;
pub mod schema;
/*
use diesel::prelude::*;
use dotenv::dotenv;
use models::*;

use schema::d01_citys;
use schema::d01_citys::dsl::*;
use schema::d02_time_zone_utc;
use schema::d02_time_zone_utc::dsl::*;
use std::env;
use uuid::Uuid;
*/
pub use self::repos::{Repo, TraitRepoD01, TraitRepoD02};
