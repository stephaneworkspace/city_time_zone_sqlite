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
    let res = repo.d01_search("Genève");
    println!("OK");
    for r in res {
        println!("{:?}\n", r);
    }
}
