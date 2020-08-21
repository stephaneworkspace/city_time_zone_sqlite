use city_time_zone_sqlite::seed_db;
//use std::panic;

const PATH: &str = "assets/citys.json";
const PATH_TZ: &str = "assets/tz_utc.json";

fn main() {
    seed_db(PATH, PATH_TZ);
}
