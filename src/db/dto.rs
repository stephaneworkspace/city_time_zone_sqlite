use super::models::{D01Citys, D02TimeZoneUtc, D03TimeZoneInfo};

#[derive(Serialize, Queryable, Debug)]
pub struct DtoCitys {
    pub d01_rec: D01Citys,
    pub d02_rec: D02TimeZoneUtc,
    pub d03_recs: Vec<D03TimeZoneInfo>,
}
