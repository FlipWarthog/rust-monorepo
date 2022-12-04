// @generated automatically by Diesel CLI.

diesel::table! {
    car (id) {
        id -> Int4,
        vin -> Text,
        make -> Text,
        model -> Text,
        year -> Int4,
        color -> Text,
        price -> Numeric,
        updated_at -> Timestamp,
    }
}
