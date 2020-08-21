CREATE TABLE d01_citys (
  d01_id VARCHAR NOT NULL PRIMARY KEY,
  d01_country VARCHAR NOT NULL,
  d01_name VARCHAR NOT NULL,
  d01_name_search VARCHAR NOT NULL COLLATE NOCASE,
  d01_lat FLOAT NOT NULL,
  d01_lng FLOAT NOT NULL
);

CREATE TABLE d02_time_zone_utc (
  d02_id VARCHAR NOT NULL PRIMARY KEY,
  d02_name VARCHAR NOT NULL,
  UNIQUE (d02_name)
);

CREATE TABLE d03_time_zone_info (
  d03_id VARCHAR NOT NULL PRIMARY KEY,
  d03_offset FLOAT NOT NULL,
  d03_text VARCHAR NOT NULL,
  UNIQUE (d03_text)
);

CREATE TABLE d04_link_d02_d03 (
  d04_d02_time_zone_utc_id VARCHAR NOT NULL,
  d04_d03_time_zone_info_id VARCHAR NOT NULL,
  PRIMARY KEY(d04_d02_time_zone_utc_id, d04_d03_time_zone_info_id),
  FOREIGN KEY(d04_d02_time_zone_utc_id) REFERENCES d02_time_zone_utc(d02_id),
  FOREIGN KEY(d04_d03_time_zone_info_id) REFERENCES d03_time_zone_info(d03_id)
);

CREATE TABLE d05_link_d01_d02 (
  d05_d01_citys_id VARCHAR NOT NULL,
  d05_d02_time_zone_utc_id VARCHAR NOT NULL,
  PRIMARY KEY (d05_d01_citys_id, d05_d02_time_zone_utc_id),
  FOREIGN KEY(d05_d01_citys_id) REFERENCES d01_citys(d01_id),
  FOREIGN KEY(d05_d02_time_zone_utc_id) REFERENCES d02_time_zone_utc(d02_id)
);
