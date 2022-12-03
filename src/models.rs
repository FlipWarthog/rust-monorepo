use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Car {
    pub id: i32,
    pub vin: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub color: String,
    pub price: f64,
    pub updated_at: NaiveDateTime,
}
