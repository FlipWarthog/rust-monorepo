pub mod model;

use actix_web::{get, web, Error, HttpResponse};

use crate::db::{self, car::CarResource};

use model::CarJson;

/// Finds car by UID.
#[get("/entity/cars/{car_id}")]
async fn get_car(
    pool: web::Data<db::DbPool>,
    car_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let car_id = car_id.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let c = web::block(move || {
        let mut conn = pool.get().unwrap();
        let mut res = CarResource::with(&mut conn);

        res.get_single(car_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(CarJson {
        id: c.id,
        vin: c.vin.clone(),
        make: c.make.clone(),
        model: c.model.clone(),
        year: c.year,
        color: c.color.clone(),
        price: c.price.clone(),
        updated_at: c.updated_at,
    }))
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

        res.list().map(|v| {
            v.iter()
                .map(|c| CarJson {
                    id: c.id,
                    vin: c.vin.clone(),
                    make: c.make.clone(),
                    model: c.model.clone(),
                    year: c.year,
                    color: c.color.clone(),
                    price: c.price.clone(),
                    updated_at: c.updated_at,
                })
                .collect::<Vec<CarJson>>()
        })
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(model::ListResp {
        total_records: cars.len(),
        records: cars,
    }))
}