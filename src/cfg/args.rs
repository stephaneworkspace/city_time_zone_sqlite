use clap::{App, Arg};
use std::env;

#[derive(Debug)]
pub struct CityConfig {
    pub city: String,
}

const AUTHOR: &str = "Stéphane Bressani <stephane@stephane-bressani.ch)";

const CITY: &str = "city";

/// Parse args for city search
pub fn parse_args() -> CityConfig {
    let matches = App::new("City time zone sqlite")
        .version(env!("CARGO_PKG_VERSION"))
        .author(AUTHOR)
        .about("Search a city worldwide and get time zone info")
        .arg(
            Arg::with_name(CITY)
                .value_name("CITY_SEARCH_QUERY")
                .multiple(false)
                .help("Name or partial name of the city (case and accent free)")
                .required(true),
        )
        .get_matches();
    CityConfig {
        city: matches
            .values_of(CITY)
            .unwrap()
            .next()
            .as_deref()
            .unwrap()
            .to_string(),
    }
}
