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
use super::schema::d04_link_d02_d03;
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
    fn d01_select_by_name(&self, name: &str) -> Vec<D01Citys>;
    fn d01_select_all(&self) -> Vec<D01Citys>;
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
    ) -> Result<(), AppError>;
}

pub trait TraitRepoD05 {
    fn d05_insert(
        &self,
        d01_citys_id: &str,
        d02_time_zone_utc_id: &str,
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
                id: &uuid,
                country: i_country,
                name: i_name,
                lat: i_lat,
                lng: i_lng,
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

    fn d01_select_by_name(&self, _s_name: &str) -> Vec<D01Citys> {
        Vec::new()
        /*d01_citys
        .filter(name.eq(s_name))
        //.limit(5)
        .load::<D01Citys>(&self.connection)
        .expect("Error query d01_city")*/
    }

    /// let d05_recs special because :
    ///
    /// https://docs.diesel.rs/diesel/associations/index.html
    /// Associations in Diesel are always child-to-parent. You can declare an association between two records with #[belongs_to]. Unlike other ORMs, Diesel has no concept of #[has_many]
    fn d01_select_all(&self) -> Vec<D01Citys> {
        let d05_recs = d01_citys
            .inner_join(d05_link_d01_d02)
            .filter(country.eq("CH"))
            //.limit(5)
            .select((d01_citys_id, d02_time_zone_utc_id))
            .load::<(String, String)>(&self.connection)
            .expect("Error query d01_city");
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
            let dto_rec = DtoCitys {
                d01_rec: d01_rec,
                d02_rec: d02_rec,
            };
            dto_recs.push(dto_rec);
            //let result =
            //let res = d01_rec.clone().into_iter().zip(d05_rec).collect::<Vec<_>>();
        }
        /*
        let d01_rec = d01_citys
            .limit(5)
            .load::<D01Citys>(&self.connection)
            .expect("Error query d01_city");
        */
        /*let d05_rec = D05LinkD01D02::belonging_to(&d01_rec)
            .load::<D05LinkD01D02>(&self.connection)
            .expect("Error query d01_city -> d05_link_d01_d02")
            .grouped_by(&d01_rec);
        */

        /*let d02_key = D05LinkD01D02::belonging_to(&d01_rec)
        .select(d02_time_zone_utc_id)
        .load::<String>(&self.connection);*/
        /*
        let versions = Version::belonging_to(krate)
          .select(id)
          .order(num.desc())
          .limit(5);
        let downloads = version_downloads
          .filter(date.gt(now - 90.days()))
          .filter(version_id.eq(any(versions)))
          .order(date)
          .load::<Download>(&conn)?;
        */
        //let d02_rec = D02TimeZoneUtc::belonging_to(&da

        //let res = d01_rec.clone().into_iter().zip(d05_rec).collect::<Vec<_>>();
        println!("{:?}", dto_recs);

        Vec::new()
        //d01_rec
    } // d05_link_d01_d02:
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
    ) -> Result<(), AppError> {
        let new_d04 = InsertD04 {
            d02_time_zone_utc_id: i_d02_time_zone_utc_id,
            d03_time_zone_info_id: i_d03_time_zone_info_id,
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
            d01_citys_id: i_d01_citys_id,
            d02_time_zone_utc_id: i_d02_time_zone_utc_id,
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
