table! {
    light_readings (id) {
        id -> Int4,
        user_id -> Int4,
        created_at -> Timestamptz,
        level -> Float4,
    }
}

table! {
    parameters (id) {
        id -> Int4,
        cell_size -> Int4,
    }
}

table! {
    sound_readings (id) {
        id -> Int4,
        user_id -> Int4,
        created_at -> Timestamptz,
        level -> Float4,
    }
}

table! {
    wifi_readings (id) {
        id -> Int4,
        user_id -> Int4,
        created_at -> Timestamptz,
        ssid -> Varchar,
        level -> Float4,
        frequency -> Float4,
    }
}
