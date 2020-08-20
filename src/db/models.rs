use super::schema::{
    d01_citys, d02_time_zone_utc, d03_time_zone_info, d04_link_d02_d03,
    d05_link_d01_d02,
};

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[table_name = "d01_citys"]
#[primary_key(id)]
pub struct D01Citys {
    pub id: String,
    pub country: String,
    pub name: String,
    pub lat: f32,
    pub lng: f32,
}

#[derive(Insertable)]
#[table_name = "d01_citys"]
pub struct InsertD01<'a> {
    pub id: &'a str,
    pub country: &'a str,
    pub name: &'a str,
    pub lat: f32,
    pub lng: f32,
}

#[derive(Serialize, Queryable, Identifiable, Debug)]
#[table_name = "d02_time_zone_utc"]
#[primary_key(id)]
pub struct D02TimeZoneUtc {
    pub id: String,
    pub name: String, // Ex: "Europe/Zurich"
}

#[derive(Insertable)]
#[table_name = "d02_time_zone_utc"]
pub struct InsertD02<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

#[derive(Serialize, Queryable, Debug)]
pub struct D03TimeZoneInfo {
    pub id: String,
    pub offset: f32,
    pub text: String, // Ex: "(UTC+01:00) Amsterdam, Berlin, Bern, Rome, Stockholm, Vienna"
}

#[derive(Insertable)]
#[table_name = "d03_time_zone_info"]
pub struct InsertD03<'a> {
    pub id: &'a str,
    pub offset: f32,
    pub text: &'a str,
}

#[derive(Serialize, Queryable, Debug)]
pub struct D04LinkD02D03 {
    pub d02_time_zone_utc_id: String,
    pub d03_time_zone_info_id: String,
}

#[derive(Insertable)]
#[table_name = "d04_link_d02_d03"]
pub struct InsertD04<'a> {
    pub d02_time_zone_utc_id: &'a str,
    pub d03_time_zone_info_id: &'a str,
}

#[derive(Serialize, Queryable, Identifiable, Associations, Debug)]
#[belongs_to(D01Citys, foreign_key = "d01_citys_id")]
#[belongs_to(D02TimeZoneUtc, foreign_key = "d02_time_zone_utc_id")]
#[table_name = "d05_link_d01_d02"]
#[primary_key(d01_citys_id, d02_time_zone_utc_id)]
pub struct D05LinkD01D02 {
    pub d01_citys_id: String,
    pub d02_time_zone_utc_id: String,
}

#[derive(Insertable)]
#[table_name = "d05_link_d01_d02"]
pub struct InsertD05<'a> {
    pub d01_citys_id: &'a str,
    pub d02_time_zone_utc_id: &'a str,
}
