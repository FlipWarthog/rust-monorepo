pub mod model;

use actix_web::{get, post, web::{self}, Error, HttpResponse};

use crate::db::{
    self,
    car::{CarResource, NewCar},
};

use model::{ QueryRequestHolder, CarJson };

use self::model::QueryRequest;

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
        id: Some(c.id),
        vin: c.vin.clone(),
        make: c.make.clone(),
        model: c.model.clone(),
        year: c.year,
        color: c.color.clone(),
        price: c.price.clone(),
        updated_at: Some(c.updated_at),
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
async fn list_cars(pool: web::Data<db::DbPool>, request: web::Query<QueryRequestHolder>) -> Result<HttpResponse, Error> {
    let query = serde_json::from_str::<QueryRequest>(&request.request).unwrap_or_default();
    log::info!("query is {:?}", query);
    // use web::block to offload blocking Diesel code without blocking server thread
    let cars = web::block(move || {
        let mut conn = pool.get().unwrap();
        let mut res = CarResource::with(&mut conn);

        res.list(query).map(|(v, i)| {
            (v.iter()
                .map(|c| CarJson {
                    id: Some(c.id),
                    vin: c.vin.clone(),
                    make: c.make.clone(),
                    model: c.model.clone(),
                    year: c.year,
                    color: c.color.clone(),
                    price: c.price.clone(),
                    updated_at: Some(c.updated_at),
                })
                .collect::<Vec<CarJson>>(), i)
        })
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(model::ListResp {
        total_records: cars.1,
        records: cars.0,
    }))
}

#[post("/entity/cars")]
async fn add_car(
    pool: web::Data<db::DbPool>,
    data: web::Json<CarJson>,
) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let car = web::block(move || {
        let mut conn = pool.get().unwrap();
        let mut res = CarResource::with(&mut conn);

        res.create(NewCar {
            vin: &data.vin,
            make: &data.make,
            model: &data.model,
            year: data.year,
            color: &data.color,
            price: data.price.clone(),
        })
        .map(|c| CarJson {
            id: Some(c.id),
            vin: c.vin.clone(),
            make: c.make.clone(),
            model: c.model.clone(),
            year: c.year,
            color: c.color.clone(),
            price: c.price.clone(),
            updated_at: Some(c.updated_at),
        })
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(car))
}
