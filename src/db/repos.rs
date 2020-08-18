use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use super::models::*;
use super::schema::d01_citys;
use super::schema::d01_citys::dsl::*;
use super::schema::d02_time_zone_utc;
// use super::schema::d02_time_zone_utc::dsl::*;
use uuid::Uuid;

pub struct RepoD01 {}

pub trait TraitRepoD01 {
    fn insert(&self, country: &str, name: &str, lat: f32, lng: f32) -> String;
    fn read_all(&self) -> Vec<D01Citys>;
}

impl TraitRepoD01 for RepoD01 {
    fn insert(
        &self,
        i_country: &str,
        i_name: &str,
        i_lat: f32,
        i_lng: f32,
    ) -> String {
        let connection = establish_connection();

        let uuid = Uuid::new_v4().to_hyphenated().to_string();

        let new_d01 = InsertD01 {
            id: &uuid,
            country: i_country,
            name: i_name,
            lat: i_lat,
            lng: i_lng,
        };

        diesel::insert_into(d01_citys::table)
            .values(&new_d01)
            .execute(&connection)
            .expect("Error saving record d01_citys");

        uuid
    }

    fn read_all(&self) -> Vec<D01Citys> {
        let connection = establish_connection();
        d01_citys
            //.filter(country.eq("Switzerland"))
            //.limit(5)
            .load::<D01Citys>(&connection)
            .expect("Error query d01_city")
    }
}
pub struct RepoD02 {}

pub trait TraitRepoD02 {
    fn insert(&self, name: &str) -> String;
}

impl TraitRepoD02 for RepoD02 {
    fn insert(&self, i_name: &str) -> String {
        let connection = establish_connection();

        let uuid = Uuid::new_v4().to_hyphenated().to_string();

        let new_d02 = InsertD02 { name: i_name };

        diesel::insert_into(d02_time_zone_utc::table)
            .values(&new_d02)
            .execute(&connection)
            .expect("Error saving record d02_time_zone_utc");

        uuid
    }
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
