use actix_web::http;
use anyhow::{Context, Result};
use url::Url;

use web_smarthome2::{error::Error, routes::rooms::NewSmartRoom};

const BASE_URL: &str = "http://127.0.0.1:8080";

#[actix_web::main]
async fn main() -> Result<()> {
    let base_url = Url::parse(BASE_URL).context("URL error")?;
    let client = reqwest::Client::new();

    let rooms_url = base_url.join("rooms").context("URL error")?;
    let response = client
        .delete(rooms_url.as_str())
        .send()
        .await
        .context("HTTP request error")?;
    if response.status() != http::StatusCode::OK {
        return Err(Error::BadRequest).context("Bad request error");
    }

    for room_name in ["Ванная", "Кухня", "Столовая"] {
        let new_room = NewSmartRoom::with_name(room_name);
        let response = client
            .post(rooms_url.as_str())
            .json(&new_room)
            .send()
            .await
            .context("HTTP request error")?;
        if response.status() == http::StatusCode::OK {
            let text = response.text().await.context("text")?;
            println!("\n{}\n", text);
        }
    }

    Ok(())
}
