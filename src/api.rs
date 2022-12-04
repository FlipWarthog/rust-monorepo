use actix_cors::Cors;
use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use bigdecimal::{BigDecimal, FromPrimitive};
use crate::{db, car::{CarResource, Car}};
use diesel::PgConnection;
use dotenvy::dotenv;
use serde::Serialize;

/// Finds car by UID.
#[get("/entity/cars/{car_id}")]
async fn get_car(
    pool: web::Data<db::DbPool>,
    car_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let car_id = car_id.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let car = web::block(move || {
        let mut conn = pool.get().unwrap();
        let mut res = CarResource::with(&mut conn);

        res.get_single(car_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(car))
}

/// Lists manufacturers.
#[get("/entity/cars/manufacturers")]
async fn get_manufacturers(pool: web::Data<db::DbPool>) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let manufacturers = web::block(move || {
        let mut conn = pool.get().unwrap();
        let mut res = CarResource::with(&mut conn);

        res.get_manufacturers()
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(manufacturers))
}

/// Lists all cars.
#[get("/entity/cars")]
async fn list_cars(pool: web::Data<db::DbPool>) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let cars = web::block(move || {
        let mut conn = pool.get().unwrap();
        let mut res = CarResource::with(&mut conn);

        res.list()
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(ListResp {
        total_records: cars.len(),
        records: cars,
    }))
}

#[derive(Serialize)]
struct ListResp {
    #[serde(rename = "totalRecords")]
    total_records: usize,
    records: Vec<Car>,
}
