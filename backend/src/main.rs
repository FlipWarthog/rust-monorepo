mod api;
mod db;

use actix_web::{middleware, web, App, HttpServer};
use actix_web_lab::web::spa;

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

    // Create server
    log::info!("Starting web server...");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(list_cars)
            .service(get_manufacturers)
            .service(get_car)
            .service(add_car)
            .service(
                spa()
                    .index_file("../frontend/dist/index.html")
                    .static_resources_mount("/static")
                    .static_resources_location("../frontend/dist/static/")
                    .finish(),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
