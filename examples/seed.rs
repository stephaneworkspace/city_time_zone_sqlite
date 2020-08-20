#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

// use serde::Deserialize;
// use serde::Serialize;
use city_time_zone_sqlite::{
    AppError, Repo, TraitRepoD01, TraitRepoD02, TraitRepoD03, TraitRepoD04,
    TraitRepoD05,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::panic;

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

#[derive(Debug, Clone, PartialEq)]
pub struct TimeZones {
    pub time_zone: Vec<TimeZone>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TimeZone {
    pub text: String,
    pub offset: f32,
    pub utc: Vec<String>,
}
#[derive(Debug, Clone)]
pub struct TempD04D02 {
    pub id: String,
    pub name: String,
    pub d03: Vec<TempD04D03>,
}

#[derive(Debug, Clone)]
pub struct TempD04D03 {
    pub id: String,
    pub text: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TempD05 {
    pub d02: Vec<TempD05D02>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TempD05D02 {
    pub id: String, // d02_id
    pub name: String,
    pub d01: Vec<TempD05D01>,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct TempD05D01 {
    pub id: String,   // d01_id
    pub name: String, // d01 name
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct TempIndexD05D02 {
    pub id_d01: String, // d01_id
    pub name: String,   // d02 name
}

struct HashMapD05 {
    map: HashMap<TempIndexD05D02, TempD05D01>,
}

trait TraitHashMapD05 {
    fn add(&mut self, id_d02: TempIndexD05D02, rec_d01: TempD05D01);
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

impl TempD05 {
    fn filter_time_zone_d02_name(
        &self,
        time_zone_d02_name: String,
    ) -> Vec<TempD05D02> {
        self.d02
            .iter()
            .filter(|&x| x.name == time_zone_d02_name)
            .cloned()
            .collect::<Vec<TempD05D02>>()
    }
}

impl TraitHashMapD05 for HashMapD05 {
    fn add(&mut self, d02: TempIndexD05D02, rec_d01: TempD05D01) {
        self.map.insert(d02, rec_d01);
    }
}

fn main() {
    println!("Seed database");
    // If this project is bigger, i need to put this code in one controller
    // for better reading of the code
    let mut i: u32 = 0;
    let citys = Citys::new(PATH);
    let time_zones = TimeZones::new(PATH_TZ);
    let mut temp_hash: HashMapD05 = HashMapD05 {
        map: HashMap::new(),
    };
    let repo = Repo::new();
    // d01
    for c in citys.city.clone() {
        let status =
            repo.d01_insert(c.country.as_ref(), c.name.as_ref(), c.lat, c.lng);
        match status {
            Ok(id) => {
                for t in c.time_zone_name.clone() {
                    temp_hash.add(
                        TempIndexD05D02 {
                            id_d01: id.clone(),
                            name: t.clone(),
                        },
                        TempD05D01 {
                            id: id.clone(),
                            name: c.name.clone(),
                        },
                    );
                }
                i += 1;
            }
            Err(AppError { err_type, message }) => match err_type {
                _ => {
                    panic!("{:?} {:?}", err_type, message);
                }
            },
        }
    }
    println!("d01 -> {} record(s) insert", i);
    // d02
    i = 0;
    let mut temp_d04: Vec<TempD04D02> = Vec::new();
    let mut temp_d05: TempD05 = { TempD05 { d02: Vec::new() } };
    for city in citys.city.clone() {
        for t in city.time_zone_name.clone() {
            let temp =
                temp_d05.filter_time_zone_d02_name(t.clone().to_string());
            if temp.len() > 0 {
            } else {
                let status = repo.d02_insert(t.as_ref());
                match status {
                    Ok(id) => {
                        let rec_d04d02 = TempD04D02 {
                            id: id.clone(),
                            name: t.clone().to_string(),
                            d03: Vec::new(),
                        };
                        temp_d04.push(rec_d04d02);
                        i += 1;
                        let rec_d05d02 = TempD05D02 {
                            id: id.clone(),
                            name: t.clone().to_string(),
                            d01: Vec::new(),
                        };
                        temp_d05.d02.push(rec_d05d02);
                    }
                    Err(AppError { err_type, message }) => {
                        panic!("{:?} {:?}", err_type, message)
                    }
                }
            }
        }
    }
    let clone_d05: TempD05 = temp_d05.clone();
    temp_d05 = TempD05 { d02: Vec::new() };
    for c in clone_d05.d02 {
        let mut temp_d01 = Vec::new();
        let key = TempIndexD05D02 {
            id_d01: "".to_string(),
            name: c.name.clone(),
        };
        /*match temp_hash.map.get(&key) {
            Some(d01) => {
                temp_d01.push(TempD05D01 {
                    id: d01.id.to_string(),
                    name: d01.name.to_string(),
                });
            }
            _ => println!("missing"),
        }*/
        for (d02, d01) in &temp_hash.map {
            if &d02.name == &key.name {
                temp_d01.push(TempD05D01 {
                    id: d01.id.to_string(),
                    name: d01.name.to_string(),
                });
            }
        }
        temp_d05.d02.push(TempD05D02 {
            id: c.id,
            name: c.name.clone(),
            d01: temp_d01.clone(),
        });
    }
    println!("d02 -> {} record(s) insert", i);
    // d03
    i = 0;
    for t in time_zones.time_zone {
        let status = repo.d03_insert(t.offset, t.text.as_ref());
        match status {
            Ok(id) => {
                for utc in t.utc {
                    let clone_d04: Vec<TempD04D02> = temp_d04.clone();
                    temp_d04 = Vec::new();
                    for c in clone_d04 {
                        if utc == c.name {
                            let mut temp_d03 = Vec::new();
                            for c_d03 in c.d03 {
                                temp_d03.push(TempD04D03 {
                                    id: c_d03.id,
                                    text: c_d03.text,
                                });
                            }
                            // Add
                            temp_d03.push(TempD04D03 {
                                id: id.clone(),
                                text: t.text.clone(),
                            });
                            temp_d04.push(TempD04D02 {
                                id: c.id,
                                name: c.name,
                                d03: temp_d03,
                            });
                        } else {
                            let mut temp_d03 = Vec::new();
                            for c_d03 in c.d03 {
                                temp_d03.push(TempD04D03 {
                                    id: c_d03.id,
                                    text: c_d03.text,
                                });
                            }
                            temp_d04.push(TempD04D02 {
                                id: c.id,
                                name: c.name,
                                d03: temp_d03,
                            });
                        }
                    }
                }
                i += 1;
            }
            Err(AppError { err_type, message }) => {
                println!("{:?}: {}", err_type, message);
                panic!(t.text)
            }
        }
    }
    println!("d03 -> {} record(s) insert", i);
    // d04
    i = 0;
    for t_d04 in temp_d04 {
        //t_d04 = d02
        for t_d03 in t_d04.d03 {
            let status = repo.d04_insert(t_d04.id.as_ref(), t_d03.id.as_ref());
            match status {
                Ok(()) => {
                    i += 1;
                }
                Err(AppError { err_type, message }) => match err_type {
                    _ => {
                        panic!("{:?} {:?}", err_type, message);
                    }
                },
            }
        }
    }
    println!("d04 -> {} record(s) insert", i);
    // d05
    i = 0;
    for t_d02 in temp_d05.d02 {
        //t_d05 = d02
        for t_d01 in t_d02.d01 {
            let status = repo.d05_insert(t_d01.id.as_ref(), t_d02.id.as_ref());
            match status {
                Ok(()) => {
                    i += 1;
                }
                Err(AppError { err_type, message }) => match err_type {
                    _ => {
                        panic!("{:?} {:?}", err_type, message);
                    }
                },
            }
        }
    }
    println!("d05 -> {} record(s) insert", i);
}
