#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// use serde::Deserialize;
// use serde::Serialize;
use std::fs::File;
use std::io::Read;

use city_time_zone_sqlite::{RepoD01, TraitRepoD01};

const PATH: &str = "assets/citys.json";

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

fn main() {
    println!("Seed database");
    // If this project is bigger, i need to put this code in one controller
    // for better reading of the code
    let mut i: u32 = 0;
    let citys = Citys::new(PATH);
    let repo_d01 = RepoD01::new(); // TODO main Repo Struct with Trait
    for c in citys.city {
        let _record_d01_id =
            repo_d01.insert(c.country.as_ref(), c.name.as_ref(), c.lat, c.lng);
        i += 1;
    }
    println!("{} records insert", i);
}
