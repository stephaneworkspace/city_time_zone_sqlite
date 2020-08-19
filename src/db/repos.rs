use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use super::errors::AppError;
use super::models::*;
use super::schema::d01_citys;
use super::schema::d01_citys::dsl::*;
use super::schema::d02_time_zone_utc;
use super::schema::d03_time_zone_info;
use super::schema::d04_link_d02_d03;
use super::schema::d05_link_d01_d02;
use uuid::Uuid;

pub struct Repo {
    connection: SqliteConnection,
}

pub trait TraitRepoD01 {
    fn d01_insert(
        &self,
        country: &str,
        name: &str,
        lat: f32,
        lng: f32,
    ) -> String;
    fn d01_read_all(&self) -> Vec<D01Citys>;
}

pub trait TraitRepoD02 {
    fn d02_insert(&self, name: &str) -> Result<String, AppError>;
}

pub trait TraitRepoD03 {
    fn d03_insert(&self, offset: f32, text: &str) -> Result<String, AppError>;
}

pub trait TraitRepoD04 {
    fn d04_insert(
        &self,
        d02_time_zone_utc_id: &str,
        d03_time_zone_info_id: &str,
    ) -> String;
}

pub trait TraitRepoD05 {
    fn d05_insert(
        &self,
        d01_citys_id: &str,
        d02_time_zone_utc_id: &str,
    ) -> String;
}

impl Repo {
    pub fn new() -> Repo {
        Repo {
            connection: establish_connection(),
        }
    }
}

impl TraitRepoD01 for Repo {
    fn d01_insert(
        &self,
        i_country: &str,
        i_name: &str,
        i_lat: f32,
        i_lng: f32,
    ) -> String {
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
            .execute(&self.connection)
            .expect("Error saving record d01_citys");

        uuid
    }

    fn d01_read_all(&self) -> Vec<D01Citys> {
        let connection = establish_connection();
        d01_citys
            //.filter(country.eq("Switzerland"))
            //.limit(5)
            .load::<D01Citys>(&connection)
            .expect("Error query d01_city")
    }
}

impl TraitRepoD02 for Repo {
    fn d02_insert(&self, i_name: &str) -> Result<String, AppError> {
        let uuid = Uuid::new_v4().to_hyphenated().to_string();

        let new_d02 = InsertD02 {
            id: &uuid,
            name: i_name,
        };

        let insert = diesel::insert_into(d02_time_zone_utc::table)
            .values(&new_d02)
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while insert d02_time_zone_utc")
            });

        match insert {
            Err(err) => Err(err),
            _ => Ok(uuid),
        }
    }
}

impl TraitRepoD03 for Repo {
    fn d03_insert(
        &self,
        i_offset: f32,
        i_text: &str,
    ) -> Result<String, AppError> {
        let uuid = Uuid::new_v4().to_hyphenated().to_string();

        let new_d03 = InsertD03 {
            id: &uuid,
            offset: i_offset,
            text: i_text,
        };

        let insert = diesel::insert_into(d03_time_zone_info::table)
            .values(&new_d03)
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(
                    err,
                    "while insert d03_time_zone_info",
                )
            });

        match insert {
            Err(err) => Err(err),
            _ => Ok(uuid),
        }
    }
}

impl TraitRepoD04 for Repo {
    fn d04_insert(
        &self,
        i_d02_time_zone_utc_id: &str,
        i_d03_time_zone_info_id: &str,
    ) -> String {
        let uuid = Uuid::new_v4().to_hyphenated().to_string();

        let new_d04 = InsertD04 {
            id: &uuid,
            d02_time_zone_utc_id: i_d02_time_zone_utc_id,
            d03_time_zone_info_id: i_d03_time_zone_info_id,
        };

        diesel::insert_into(d04_link_d02_d03::table)
            .values(&new_d04)
            .execute(&self.connection)
            .expect("Error saving record d04_link_d02_d03");

        uuid
    }
}

impl TraitRepoD05 for Repo {
    fn d05_insert(
        &self,
        i_d01_citys_id: &str,
        i_d02_time_zone_utc_id: &str,
    ) -> String {
        let uuid = Uuid::new_v4().to_hyphenated().to_string();

        let new_d05 = InsertD05 {
            id: &uuid,
            d01_citys_id: i_d01_citys_id,
            d02_time_zone_utc_id: i_d02_time_zone_utc_id,
        };

        diesel::insert_into(d05_link_d01_d02::table)
            .values(&new_d05)
            .execute(&self.connection)
            .expect("Error saving record d05_link_d01_d02");

        uuid
    }
}

/// Connection to Sqlite
fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
