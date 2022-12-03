pub mod models;
pub mod schema;

use crate::schema::car::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_vins() -> Vec<String> {
    let connection = &mut establish_connection();

    car.select(vin)
        .load::<String>(connection)
        .expect("Error loading vins")
}
