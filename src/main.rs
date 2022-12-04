mod api;
mod car;
mod db;
mod schema;

use actix_cors::Cors;
use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use bigdecimal::{BigDecimal, FromPrimitive};
use car::{CarResource, Car};
use diesel::PgConnection;
use dotenvy::dotenv;
use serde::Serialize;

use crate::api::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    log::info!("Starting db connection...");
    {
        let conn = &mut db::establish_connection();

        log::info!("Running migrations...");
        let mig_res = db::run_migrations(conn);
        if let Err(_) = mig_res {
            log::error!("Received error migrating database");
            panic!("DB error");
        }
    }

    // Create connection pool
    let pool = db::connection_pool();
    log::info!("Starting web server...");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(list_cars)
            .service(get_manufacturers)
            .service(get_car)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}