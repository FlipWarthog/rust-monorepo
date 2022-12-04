mod api;
mod db;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};

use dotenvy::dotenv;

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
        if mig_res.is_err() {
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
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
