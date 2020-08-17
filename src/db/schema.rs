table! {
    d01_citys (id) {
        id -> Text,
        country -> Text,
        name -> Text,
        lat -> Float,
        lng -> Float,
    }
}

table! {
    d02_time_zone_utc (id) {
        id -> Text,
        name -> Text,
    }
}

table! {
    d03_time_zone_info (id) {
        id -> Text,
        offset -> Float,
        text -> Text,
    }
}

table! {
    d04_link_d02_d03 (id) {
        id -> Text,
        d02_time_zone_utc_id -> Text,
        d03_time_zone_info_id -> Text,
    }
}

table! {
    d05_link_d01_d02 (id) {
        id -> Text,
        d01_citys_id -> Text,
        d02_time_zone_utc_id -> Text,
    }
}

joinable!(d04_link_d02_d03 -> d03_time_zone_info (d03_time_zone_info_id));
joinable!(d05_link_d01_d02 -> d01_citys (d01_citys_id));

allow_tables_to_appear_in_same_query!(
    d01_citys,
    d02_time_zone_utc,
    d03_time_zone_info,
    d04_link_d02_d03,
    d05_link_d01_d02,
);
