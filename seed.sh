#!/bin/sh
DB=$(grep DATABASE_URL .env | cut -d '=' -f 2-)
rm $DB
diesel migration run
cargo run --example seed
