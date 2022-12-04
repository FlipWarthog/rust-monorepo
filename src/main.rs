#![allow(dead_code, unused)]

mod car;
mod schema;
use std::{env, error::Error};

use bigdecimal::{BigDecimal, FromPrimitive};
use car::CarResource;
use diesel::{prelude::*, pg::Pg};
use dotenvy::dotenv;

use crate::car::NewCar;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");


fn main() {
    let res = work();
    match res {
        Ok(_) => println!("Completed successfully"),
        Err(_) => println!("Errored successfully"),
    };
}

fn work() -> Result<(), diesel::result::Error> {
    let mut conn = establish_connection();

    run_migrations(&mut conn);

    let mut car_resource = CarResource::with(&mut conn);

    let c = car_resource.create(NewCar {
        vin: "new val",
        make: "new val",
        model: "new val",
        year: 2022,
        color: "new val",
        price: BigDecimal::from_f32(2000.00).unwrap(),
    })?;

    println!("Car added! {:?}", c);

    let mut update_car = c;

    update_car.make.push_str(", update");

    let u = car_resource.update(update_car)?;

    println!("Update car is {:?}", u);

    let n = car_resource.delete(u.id)?;
    println!("Deleted car. Response is {n}");
    Ok(())
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn run_migrations(connection: &mut impl MigrationHarness<Pg>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
