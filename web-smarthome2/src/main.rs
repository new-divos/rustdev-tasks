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
                    .route("", web::post().to(routes::room::new_room))
                    .route("", web::get().to(routes::room::all_rooms))
                    .route("", web::delete().to(routes::room::delete_rooms))
                    .service(
                        web::scope("/{room_id}")
                            .route("", web::get().to(routes::room::get_room))
                            .route("", web::delete().to(routes::room::delete_room))
                            .route("", web::put().to(routes::room::update_room))
                            .service(
                                web::scope("/thermometers")
                                    .route("", web::post().to(routes::thermometer::new_thermometer))
                                    .route("", web::get().to(routes::thermometer::all_thermometers))
                                    .service(
                                        web::scope("/{thermometer_id}")
                                            .route(
                                                "",
                                                web::get().to(routes::thermometer::get_thermometer),
                                            )
                                            .route(
                                                "",
                                                web::delete()
                                                    .to(routes::thermometer::delete_thermometer),
                                            )
                                            .route(
                                                "",
                                                web::put()
                                                    .to(routes::thermometer::update_thermometer),
                                            ),
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
