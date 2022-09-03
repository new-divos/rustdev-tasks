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
            .service(web::scope("/rooms").route("", web::post().to(routes::room::new_room)))
    })
    .bind(("127.0.0.1", 8080))
    .context("HTTP server binding")?
    .run()
    .await
    .context("HTTP server running")
}
