pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::*;
use schema::d01_citys;
use schema::d01_citys::dsl::*;
use uuid::Uuid;

pub fn insert_d01_citys(
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

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_d01_citys() -> Vec<D01Citys> {
    let connection = establish_connection();
    d01_citys
        //.filter(country.eq("Switzerland"))
        //.limit(5)
        .load::<D01Citys>(&connection)
        .expect("Error query d01_city")
}
