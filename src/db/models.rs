use super::schema::{
    d01_citys, d02_time_zone_utc, d03_time_zone_info, d04_link_d02_d03,
    d05_link_d01_d02,
};

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[table_name = "d01_citys"]
#[primary_key(d01_id)]
pub struct D01Citys {
    pub d01_id: String,
    pub d01_country: String,
    pub d01_name: String,
    pub d01_lat: f32,
    pub d01_lng: f32,
}

#[derive(Insertable)]
#[table_name = "d01_citys"]
pub struct InsertD01<'a> {
    pub d01_id: &'a str,
    pub d01_country: &'a str,
    pub d01_name: &'a str,
    pub d01_lat: f32,
    pub d01_lng: f32,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[table_name = "d02_time_zone_utc"]
#[primary_key(d02_id)]
pub struct D02TimeZoneUtc {
    pub d02_id: String,
    pub d02_name: String, // Ex: "Europe/Zurich"
}

#[derive(Insertable)]
#[table_name = "d02_time_zone_utc"]
pub struct InsertD02<'a> {
    pub d02_id: &'a str,
    pub d02_name: &'a str,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[table_name = "d03_time_zone_info"]
#[primary_key(d03_id)]
pub struct D03TimeZoneInfo {
    pub d03_id: String,
    pub d03_offset: f32,
    pub d03_text: String, // Ex: "(UTC+01:00) Amsterdam, Berlin, Bern, Rome, Stockholm, Vienna"
}

#[derive(Insertable)]
#[table_name = "d03_time_zone_info"]
pub struct InsertD03<'a> {
    pub d03_id: &'a str,
    pub d03_offset: f32,
    pub d03_text: &'a str,
}

#[derive(Serialize, Queryable, Identifiable, Associations, Debug)]
#[belongs_to(D02TimeZoneUtc, foreign_key = "d04_d02_time_zone_utc_id")]
#[belongs_to(D03TimeZoneInfo, foreign_key = "d04_d03_time_zone_info_id")]
#[table_name = "d04_link_d02_d03"]
#[primary_key(d04_d02_time_zone_utc_id, d04_d03_time_zone_info_id)]
pub struct D04LinkD02D03 {
    pub d04_d02_time_zone_utc_id: String,
    pub d04_d03_time_zone_info_id: String,
}

#[derive(Insertable)]
#[table_name = "d04_link_d02_d03"]
pub struct InsertD04<'a> {
    pub d04_d02_time_zone_utc_id: &'a str,
    pub d04_d03_time_zone_info_id: &'a str,
}

#[derive(Serialize, Queryable, Identifiable, Associations, Debug)]
#[belongs_to(D01Citys, foreign_key = "d05_d01_citys_id")]
#[belongs_to(D02TimeZoneUtc, foreign_key = "d05_d02_time_zone_utc_id")]
#[table_name = "d05_link_d01_d02"]
#[primary_key(d05_d01_citys_id, d05_d02_time_zone_utc_id)]
pub struct D05LinkD01D02 {
    pub d05_d01_citys_id: String,
    pub d05_d02_time_zone_utc_id: String,
}

#[derive(Insertable)]
#[table_name = "d05_link_d01_d02"]
pub struct InsertD05<'a> {
    pub d05_d01_citys_id: &'a str,
    pub d05_d02_time_zone_utc_id: &'a str,
}
