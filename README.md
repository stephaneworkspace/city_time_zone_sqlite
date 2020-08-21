# Prerequisites

On OSX:
````
brew install sqlite3
cargo install diesel_cli
````

For create the db (this take 7 minutes ~)
(this delete the db in .env)
````
./seed.sh
````

For query
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
