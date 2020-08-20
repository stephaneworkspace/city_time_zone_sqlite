/*#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
*/
// use serde::Deserialize;
// use serde::Serialize;
use city_time_zone_sqlite::{/*AppError,*/ Repo, TraitRepoD01};

fn main() {
    let repo = Repo::new();
    let _res = repo.d01_select_all();
    /*for r in res {
        println!("{:?}", r);
    }*/
}
