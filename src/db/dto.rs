use super::models::{D01Citys, D02TimeZoneUtc, D03TimeZoneInfo};

#[derive(Serialize, Queryable, Debug)]
pub struct DtoCitys {
    pub d01_rec: D01Citys,
    pub d02_rec: D02TimeZoneUtc,
    pub d03_recs: Vec<D03TimeZoneInfo>,
}

#[derive(Serialize, Queryable, Debug)]
pub struct DtoCitysCompact {
    pub country: String,
    pub name: String,
    pub lat: f32,
    pub lng: f32,
    pub tz_name: String, // Ex: "Europe/Zurich"
    pub tz: Vec<DtoTimeZoneCompact>,
}

#[derive(Serialize, Queryable, Debug)]
pub struct DtoTimeZoneCompact {
    pub offset: f32,
    pub text: String, // Ex: "(UTC+01:00) Amsterdam, Berlin, Bern, Rome, Stockholm, Vienna"
}
