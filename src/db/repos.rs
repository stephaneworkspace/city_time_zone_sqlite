use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use super::models::*;
use super::schema::d01_citys;
use super::schema::d01_citys::dsl::*;
use super::schema::d02_time_zone_utc;
use super::schema::d03_time_zone_info;
use super::schema::d04_link_d02_d03;
use super::schema::d05_link_d01_d02;
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

        let new_d02 = InsertD02 {
            id: &uuid,
            name: i_name,
        };

        diesel::insert_into(d02_time_zone_utc::table)
            .values(&new_d02)
            .execute(&connection)
            .expect("Error saving record d02_time_zone_utc");

        uuid
    }
}

pub struct RepoD03 {}

pub trait TraitRepoD03 {
    fn insert(&self, offset: f32, text: &str) -> String;
}

impl TraitRepoD03 for RepoD03 {
    fn insert(&self, i_offset: f32, i_text: &str) -> String {
        let connection = establish_connection();

        let uuid = Uuid::new_v4().to_hyphenated().to_string();

        let new_d03 = InsertD03 {
            id: &uuid,
            offset: i_offset,
            text: i_text,
        };

        diesel::insert_into(d03_time_zone_info::table)
            .values(&new_d03)
            .execute(&connection)
            .expect("Error saving record d03_time_zone_info");

        uuid
    }
}

pub struct RepoD04 {}

pub trait TraitRepoD04 {
    fn insert(
        &self,
        d02_time_zone_utc_id: &str,
        d03_time_zone_info_id: &str,
    ) -> String;
}

impl TraitRepoD04 for RepoD04 {
    fn insert(
        &self,
        i_d02_time_zone_utc_id: &str,
        i_d03_time_zone_info_id: &str,
    ) -> String {
        let connection = establish_connection();

        let uuid = Uuid::new_v4().to_hyphenated().to_string();

        let new_d04 = InsertD04 {
            id: &uuid,
            d02_time_zone_utc_id: i_d02_time_zone_utc_id,
            d03_time_zone_info_id: i_d03_time_zone_info_id,
        };

        diesel::insert_into(d04_link_d02_d03::table)
            .values(&new_d04)
            .execute(&connection)
            .expect("Error saving record d04_link_d02_d03");

        uuid
    }
}

pub struct RepoD05 {}

pub trait TraitRepoD05 {
    fn insert(&self, d01_citys_id: &str, d02_time_zone_utc_id: &str) -> String;
}

impl TraitRepoD05 for RepoD05 {
    fn insert(
        &self,
        i_d01_citys_id: &str,
        i_d02_time_zone_utc_id: &str,
    ) -> String {
        let connection = establish_connection();

        let uuid = Uuid::new_v4().to_hyphenated().to_string();

        let new_d05 = InsertD05 {
            id: &uuid,
            d01_citys_id: i_d01_citys_id,
            d02_time_zone_utc_id: i_d02_time_zone_utc_id,
        };

        diesel::insert_into(d05_link_d01_d02::table)
            .values(&new_d05)
            .execute(&connection)
            .expect("Error saving record d05_link_d01_d02");

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
