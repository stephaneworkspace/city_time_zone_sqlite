use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use super::dto::DtoCitys;
use super::errors::{AppError, ErrorType};
use super::models::*;
use super::schema::d01_citys;
use super::schema::d01_citys::dsl::*;
use super::schema::d02_time_zone_utc;
use super::schema::d02_time_zone_utc::dsl::*;
use super::schema::d03_time_zone_info;
use super::schema::d03_time_zone_info::dsl::*;
use super::schema::d04_link_d02_d03;
use super::schema::d04_link_d02_d03::dsl::*;
use super::schema::d05_link_d01_d02;
use super::schema::d05_link_d01_d02::dsl::*;
use uuid::Uuid;

const MAX_SQL_INSERT_UNIQUE: usize = 15;

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
    ) -> Result<String, AppError>;
    fn d01_search(&self, search: &str) -> Vec<DtoCitys>;
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
        d04_d02_time_zone_utc_id: &str,
        d04_d03_time_zone_info_id: &str,
    ) -> Result<(), AppError>;
}

pub trait TraitRepoD05 {
    fn d05_insert(
        &self,
        d05_d01_citys_id: &str,
        d05_d02_time_zone_utc_id: &str,
    ) -> Result<(), AppError>;
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
    ) -> Result<String, AppError> {
        let mut i: usize = 0;
        loop {
            let uuid = Uuid::new_v4().to_hyphenated().to_string();

            let new_d01 = InsertD01 {
                d01_id: &uuid,
                d01_country: i_country,
                d01_name: i_name,
                d01_lat: i_lat,
                d01_lng: i_lng,
            };

            let insert = diesel::insert_into(d01_citys::table)
                .values(&new_d01)
                .execute(&self.connection)
                .map_err(|err| {
                    AppError::from_diesel_err(err, "while insert d01_citys")
                });

            let res = unique_violation_security(insert, uuid.to_string(), i);
            if !res.0 {
                i += 1
            } else {
                return res.1;
            }
        }
    }

    /// Search
    ///
    /// let d05_recs special because :
    ///
    /// https://docs.diesel.rs/diesel/associations/index.html
    ///
    /// Associations in Diesel are always child-to-parent.
    /// You can declare an association between two records with #[belongs_to].
    /// Unlike other ORMs, Diesel has no concept of #[has_many]
    ///                                               --------
    /// Other technics for join belonging_to resut
    ///
    /// let d01_rec = d01_citys
    ///     .limit(5)
    ///     .load::<D01Citys>(&self.connection)
    ///     .expect("Error query d01_city");
    /// let d05_rec = D05LinkD01D02::belonging_to(&d01_rec)
    ///     .load::<D05LinkD01D02>(&self.connection)
    ///     .expect("Error query d01_city -> d05_link_d01_d02")
    ///     .grouped_by(&d01_rec);
    /// let res = d01_rec.clone().into_iter().zip(d05_rec).collect::<Vec<_>>();
    fn d01_search(&self, search: &str) -> Vec<DtoCitys> {
        let d05_recs = if search == "" {
            Vec::new()
        } else {
            d01_citys
                .inner_join(d05_link_d01_d02)
                // .inner_join(d04_link_d02_d03) // don't work now in this version
                //                               // of diesel
                .filter(d01_name.eq(search))
                //.limit(5)
                .select((d05_d01_citys_id, d05_d02_time_zone_utc_id))
                .load::<(String, String)>(&self.connection)
                .expect("Error query d01_city")
        };
        let mut dto_recs: Vec<DtoCitys> = Vec::new();
        for rec in &d05_recs {
            let d01_rec = d01_citys
                .find(&rec.0)
                .first::<D01Citys>(&self.connection)
                .expect("Error query find d01_city");
            let d02_rec = d02_time_zone_utc
                .find(&rec.1)
                .first::<D02TimeZoneUtc>(&self.connection)
                .expect("Error query find d02_time_zone_utc");
            let d04_recs = d04_link_d02_d03
                .filter(d04_d02_time_zone_utc_id.eq(&d02_rec.d02_id))
                .select(d04_d03_time_zone_info_id)
                .load::<String>(&self.connection)
                .expect("Error query filter d03_time_zone_info");
            let mut d03_recs: Vec<D03TimeZoneInfo> = Vec::new();
            for d04_rec in d04_recs {
                let d03_rec = d03_time_zone_info
                    .find(&d04_rec)
                    .first::<D03TimeZoneInfo>(&self.connection)
                    .expect("Error query find d03_time_zone_info");
                d03_recs.push(d03_rec);
            }
            let dto_rec = DtoCitys {
                d01_rec: d01_rec,
                d02_rec: d02_rec,
                d03_recs: d03_recs,
            };
            dto_recs.push(dto_rec);
        }
        dto_recs
    }
}

impl TraitRepoD02 for Repo {
    fn d02_insert(&self, i_name: &str) -> Result<String, AppError> {
        let uuid = Uuid::new_v4().to_hyphenated().to_string();

        let new_d02 = InsertD02 {
            d02_id: &uuid,
            d02_name: i_name,
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
            d03_id: &uuid,
            d03_offset: i_offset,
            d03_text: i_text,
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
    ) -> Result<(), AppError> {
        let new_d04 = InsertD04 {
            d04_d02_time_zone_utc_id: i_d02_time_zone_utc_id,
            d04_d03_time_zone_info_id: i_d03_time_zone_info_id,
        };

        let insert = diesel::insert_into(d04_link_d02_d03::table)
            .values(&new_d04)
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while insert d04_link_d02_d03")
            });

        match insert {
            Err(err) => Err(err),
            _ => Ok(()),
        }
    }
}

impl TraitRepoD05 for Repo {
    fn d05_insert(
        &self,
        i_d01_citys_id: &str,
        i_d02_time_zone_utc_id: &str,
    ) -> Result<(), AppError> {
        let new_d05 = InsertD05 {
            d05_d01_citys_id: i_d01_citys_id,
            d05_d02_time_zone_utc_id: i_d02_time_zone_utc_id,
        };

        let insert = diesel::insert_into(d05_link_d01_d02::table)
            .values(&new_d05)
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while insert d05_link_d01_d02")
            });

        match insert {
            Err(err) => Err(err),
            _ => Ok(()),
        }
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

/// Security for UUID don't garrented without collision (PRIMARY KEY in Sqlite)
fn unique_violation_security(
    res: Result<usize, AppError>,
    uuid: String,
    i: usize,
) -> (bool, Result<String, AppError>) {
    match res {
        Err(AppError { err_type, message }) => {
            if err_type == ErrorType::UniqueViolation {
                if i >= MAX_SQL_INSERT_UNIQUE {
                    return (true, Result::Err(AppError { err_type, message }));
                } else {
                    return (
                        false,
                        Result::Err(AppError { err_type, message }),
                    );
                }
            } else {
                return (true, Result::Err(AppError { err_type, message }));
            }
        }
        _ => return (true, Result::Ok(uuid)),
    }
}
