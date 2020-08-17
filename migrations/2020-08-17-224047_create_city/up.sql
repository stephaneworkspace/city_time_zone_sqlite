CREATE TABLE d01_citys (
  id VARCHAR NOT NULL PRIMARY KEY,
  country VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  lat FLOAT NOT NULL,
  lng FLOAT NOT NULL
);

CREATE TABLE d02_time_zone_utc (
  id VARCHAR NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE d03_time_zone_info (
  id VARCHAR NOT NULL PRIMARY KEY,
  offset FLOAT NOT NULL,
  text VARCHAR NOT NULL
);

CREATE TABLE d04_d02_d03 (
  id VARCHAR NOT NULL PRIMARY KEY,
  d02_time_zone_utc_id VARCHAR NOT NULL,
  d03_time_zone_info_id VARCHAR NOT NULL,
  FOREIGN KEY(d02_time_zone_utc_id) REFERENCES d02_citys(id),
  FOREIGN KEY(d03_time_zone_info_id) REFERENCES d03_time_zone_info(id)
);

CREATE TABLE d05_d01_d02 (
  id VARCHAR NOT NULL PRIMARY KEY,
  d01_citys_id VARCHAR NOT NULL,
  d02_time_zone_utc_id VARCHAR NOT NULL,
  FOREIGN KEY(d01_citys_id) REFERENCES d01_citys(id),
  FOREIGN KEY(d02_time_zone_utc_id) REFERENCES d02_citys(id)
);
