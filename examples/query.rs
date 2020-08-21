/*#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
*/
// use serde::Deserialize;
// use serde::Serialize;
use city_time_zone_sqlite::{AppError, Repo, TraitRepoD01, TraitRepoUtils};
use std::panic;

fn main() {
    let status = Repo::connect();
    let repo = match status {
        Ok(res) => res,
        Err(AppError { err_type, message }) => {
            panic!("{:?} {}", err_type, message)
        }
    };
    let status = repo.d01_search("Geneve");
    let recs = match status {
        Ok(res) => res,
        Err(AppError { err_type, message }) => {
            panic!("{:?} {}", err_type, message)
        }
    };
    for r in &recs {
        println!("{:?}\n", r);
    }
    println!("Query with {} result(s)", recs.len());
}
