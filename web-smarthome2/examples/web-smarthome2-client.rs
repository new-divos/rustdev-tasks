use std::collections::HashMap;

use actix_web::http;
use anyhow::{Context, Result};
use rand::{thread_rng, Rng};
use url::Url;
use uuid::Uuid;

use web_smarthome2::{
    db::model::{
        house::{NewRoom, RoomInfo},
        thermometer::NewThermometer,
    },
    error::Error,
};

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

    let mut rooms: HashMap<String, Uuid> = HashMap::new();
    for room_name in ["Ванная", "Кухня", "Столовая"] {
        let new_room = NewRoom::new(room_name);
        let response = client
            .post(rooms_url.as_str())
            .json(&new_room)
            .send()
            .await
            .context("HTTP request error")?;
        if response.status() == http::StatusCode::OK {
            let room = response.json::<RoomInfo>().await.context("Data error")?;
            rooms.insert(room_name.to_string(), room.room_id());
        }
    }

    let mut rng = thread_rng();
    for (room_name, room_id) in rooms.iter() {
        let thermometers_url = rooms_url
            .join(format!("/{}/thermometers", room_id).as_str())
            .context("URL error")?;
        println!("{}", thermometers_url);

        for i in 1..=1 {
            let temperature = rng.gen_range(20..=80) as f64;
            let thermometer_name = format!("{}, термометр {}", room_name, i);

            let new_thermometer = NewThermometer::new(thermometer_name, temperature);
            let response = client
                .post(thermometers_url.as_str())
                .json(&new_thermometer)
                .send()
                .await
                .context("HTTP request error")?;
            if response.status() != http::StatusCode::OK {
                println!("{:?}", response);
            }
        }
    }

    Ok(())
}
