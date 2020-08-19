#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// use serde::Deserialize;
// use serde::Serialize;
use std::fs::File;
use std::io::Read;
use std::panic;

use city_time_zone_sqlite::{
    AppError, ErrorType, Repo, TraitRepoD01, TraitRepoD02, TraitRepoD03,
    TraitRepoD04, TraitRepoD05,
};

const PATH: &str = "assets/citys.json";
const PATH_TZ: &str = "assets/tz_utc.json";

#[derive(Debug, Clone)]
pub struct Citys {
    pub city: Vec<City>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct City {
    pub country: String,
    pub name: String,
    pub lat: f32,
    pub lng: f32,
    pub time_zone_name: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TimeZones {
    pub time_zone: Vec<TimeZone>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeZone {
    pub text: String,
    pub offset: f32,
    pub utc: Vec<String>,
}

impl Citys {
    fn new(path: &str) -> Citys {
        let mut s = String::new();
        let mut file_path: std::path::PathBuf = std::path::PathBuf::new();
        file_path.push(std::env::current_dir().unwrap().as_path());
        file_path.push(path);
        File::open(file_path.as_path())
            .unwrap()
            .read_to_string(&mut s)
            .unwrap();
        Citys {
            city: serde_json::from_str(&s).unwrap(),
        }
    }
}

impl TimeZones {
    fn new(path: &str) -> TimeZones {
        let mut s = String::new();
        let mut file_path: std::path::PathBuf = std::path::PathBuf::new();
        file_path.push(std::env::current_dir().unwrap().as_path());
        file_path.push(path);
        File::open(file_path.as_path())
            .unwrap()
            .read_to_string(&mut s)
            .unwrap();
        TimeZones {
            time_zone: serde_json::from_str(&s).unwrap(),
        }
    }
}

fn main() {
    println!("Seed database");
    // If this project is bigger, i need to put this code in one controller
    // for better reading of the code
    let mut i: u32 = 0;
    let citys = Citys::new(PATH);
    let time_zones = TimeZones::new(PATH_TZ);
    let repo = Repo::new();
    for c in &citys.city {
        let res =
            repo.d01_insert(c.country.as_ref(), c.name.as_ref(), c.lat, c.lng);
        match res {
            Ok(_id) => i += 1,
            Err(AppError { err_type, message }) => match err_type {
                _ => {
                    panic!("{:?} {:?}", err_type, message);
                }
            },
        }
    }
    println!("d01 -> {} record(s) insert", i);
    i = 0;
    for c in citys.city {
        for t in c.time_zone_name {
            let res = repo.d02_insert(t.as_ref());
            match res {
                Ok(_id) => i += 1,
                Err(AppError { err_type, message }) => match err_type {
                    ErrorType::UniqueViolation => {}
                    _ => {
                        panic!("{:?} {:?}", err_type, message);
                    }
                },
            }
        }
    }
    println!("d02 -> {} record(s) insert", i);
    i = 0;
    for t in time_zones.time_zone {
        let res = repo.d03_insert(t.offset, t.text.as_ref());
        match res {
            Ok(_id) => i += 1,
            Err(AppError { err_type, message }) => {
                println!("{:?}: {}", err_type, message);
                panic!(t.text)
            }
        }
    }
    println!("d03 -> {} record(s) insert", i);
}
/*
 * pub enum ErrorType {
    Internal,
    NotFound,
    UniqueViolation,
}

*/
