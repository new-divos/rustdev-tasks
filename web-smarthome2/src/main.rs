use actix_web::{web, App, HttpServer};
use anyhow::{Context, Result};

use web_smarthome2::{
    config::Config,
    db::{create_database, model::house::SmartHouse},
    routes,
};

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::new().context("create configuration")?;
    create_database(config.database_url())
        .await
        .context("create database")?;

    let house = SmartHouse::with_config(config).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(house.clone()))
            .default_service(web::route().to(routes::not_found))
            .service(
                web::scope("/rooms")
                    .route("", web::post().to(routes::rooms::new))
                    .route("", web::get().to(routes::rooms::all))
                    .route("", web::delete().to(routes::rooms::delete_all))
                    .route("/info", web::get().to(routes::rooms::report))
                    .service(
                        web::scope("/{room_id}")
                            .route("", web::get().to(routes::rooms::get))
                            .route("", web::delete().to(routes::rooms::delete))
                            .route("", web::put().to(routes::rooms::update))
                            .route("/info", web::get().to(routes::rooms::info))
                            .service(
                                web::scope("/devices")
                                    .route("/socket", web::post().to(routes::devices::new_socket))
                                    .route(
                                        "/thermometer",
                                        web::post().to(routes::devices::new_thermometer),
                                    )
                                    .route("", web::get().to(routes::devices::all))
                                    .service(
                                        web::scope("/{device_id}")
                                            .route("", web::get().to(routes::devices::get))
                                            .route("", web::delete().to(routes::devices::delete))
                                            .route("", web::put().to(routes::devices::update))
                                            .route("/info", web::get().to(routes::devices::info)),
                                    ),
                            ),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))
    .context("HTTP server binding")?
    .run()
    .await
    .context("HTTP server running")
}
