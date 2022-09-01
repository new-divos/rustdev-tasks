use anyhow::{Context, Result};

use web_smarthome2::config::Config;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::new().context("create configuration")?;

    println!("{}", config.database_url());

    Ok(())
}
