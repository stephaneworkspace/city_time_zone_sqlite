# Prerequisites

On OSX:
````
brew install sqlite3
cargo install diesel_cli --no-default-features --features "sqlite"
````

On Debian
````
sudo apt-get install sqlite3
sudo apt-get install libsqlite3-dev
cargo install diesel_cli --no-default-features --features "sqlite"
````

# Seed the database

For create the db (this take 7 minutes ~ on my mac book air end 2019 and 33
minutes ~ on vps single core from ovh)

Warning : this delete the db in .env (city_time_zone.db)
````
./seed.sh
````

# Query cli

````
cargo run --example query Geneve
cargo run --example query -- --help

City time zone sqlite 0.1.0
Stéphane Bressani <stephane@stephane-bressani.ch)
Search a city worldwide and get time zone info

USAGE:
    query <CITY_SEARCH_QUERY>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <CITY_SEARCH_QUERY>    Name or partial name of the city (case and accent free)
````
