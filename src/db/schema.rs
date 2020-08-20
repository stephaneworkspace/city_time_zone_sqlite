table! {
    d01_citys (d01_id) {
        d01_id -> Text,
        d01_country -> Text,
        d01_name -> Text,
        d01_lat -> Float,
        d01_lng -> Float,
    }
}

table! {
    d02_time_zone_utc (d02_id) {
        d02_id -> Text,
        d02_name -> Text,
    }
}

table! {
    d03_time_zone_info (d03_id) {
        d03_id -> Text,
        d03_offset -> Float,
        d03_text -> Text,
    }
}

table! {
    d04_link_d02_d03 (d04_d02_time_zone_utc_id, d04_d03_time_zone_info_id) {
        d04_d02_time_zone_utc_id -> Text,
        d04_d03_time_zone_info_id -> Text,
    }
}

table! {
    d05_link_d01_d02 (d05_d01_citys_id, d05_d02_time_zone_utc_id) {
        d05_d01_citys_id -> Text,
        d05_d02_time_zone_utc_id -> Text,
    }
}

joinable!(d04_link_d02_d03 -> d02_time_zone_utc (d04_d02_time_zone_utc_id));
joinable!(d04_link_d02_d03 -> d03_time_zone_info (d04_d03_time_zone_info_id));
joinable!(d05_link_d01_d02 -> d01_citys (d05_d01_citys_id));
joinable!(d05_link_d01_d02 -> d02_time_zone_utc (d05_d02_time_zone_utc_id));

allow_tables_to_appear_in_same_query!(
    d01_citys,
    d02_time_zone_utc,
    d03_time_zone_info,
    d04_link_d02_d03,
    d05_link_d01_d02,
);
