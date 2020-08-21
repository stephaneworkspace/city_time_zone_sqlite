use city_time_zone_sqlite::{seed_db, AppError};
use std::panic;

const PATH: &str = "assets/citys.json";
const PATH_TZ: &str = "assets/tz_utc.json";

/// If the files are missing, you can download here :
///
/// https://raw.githubusercontent.com/stephaneworkspace/city_time_zone_sqlite/master/assets/citys.json
/// https://raw.githubusercontent.com/stephaneworkspace/city_time_zone_sqlite/master/assets/tz_utc.json
fn main() {
    match seed_db(PATH, PATH_TZ) {
        Ok(()) => {}
        Err(AppError { err_type, message }) => {
            panic!("{:?} {}", err_type, message)
        }
    }
}
