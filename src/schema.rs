// @generated automatically by Diesel CLI.

diesel::table! {
    car (id) {
        id -> Int8,
        vin -> Varchar,
        make -> Varchar,
        model -> Varchar,
        year -> Int4,
        color -> Varchar,
        price -> Numeric,
        updated_at -> Timestamp,
    }
}
