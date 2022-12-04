mod schema;
use std::env;

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use dotenvy::dotenv;
use schema::car::dsl::*;

#[derive(Debug, PartialEq, Eq, Queryable)]
pub struct Car {
    id: i32,
    vin: String,
    make: String,
    model: String,
    year: i32,
    color: String,
    price: BigDecimal,
    updated_at: NaiveDateTime,
}

fn main() {
    let mut conn = establish_connection();

    let cars = car.load::<Car>(&mut conn).expect("Error connecting?");

    for c in cars {
        println!("{:?}", c);
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
