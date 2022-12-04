#![allow(dead_code, unused)]

mod car;
mod schema;
use std::env;

use bigdecimal::{BigDecimal, FromPrimitive};
use car::CarResource;
use diesel::prelude::*;
use dotenvy::dotenv;

use crate::car::NewCar;

fn main() {
    let mut conn = establish_connection();
    let mut car_resource = CarResource::with(&mut conn);

    let r = car_resource.create(NewCar {
        vin: "new val",
        make: "new val",
        model: "new val",
        year: 2022,
        color: "new val",
        price: BigDecimal::from_f32(2000.00).unwrap(),
    });
    if let Ok(c) = r {
        println!("Car added! {:?}", c);

        let mut update_car = c;

        update_car.make.push_str(", update");

        let r = car_resource.update(update_car);

        if let Ok(u) = r {
            println!("Update car is {:?}", u);

            let d = car_resource.delete(u.id);
            if let Ok(n) = d {
                println!("Deleted car. Response is {n}");
            }
        }
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
