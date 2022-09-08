use std::collections::HashMap;

use actix_web::http;
use anyhow::{Context, Result};
use serde_json::Value;
use url::Url;

use web_smarthome2::{
    error::Error,
    routes::{
        devices::{NewSmartDevice, SmartDevicePatch},
        rooms::NewSmartRoom,
    },
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

    let mut rooms: HashMap<String, String> = HashMap::new();
    for room_name in ["Ванная", "Кухня", "Столовая"] {
        let new_room = NewSmartRoom::with_name(room_name);
        let response = client
            .post(rooms_url.as_str())
            .json(&new_room)
            .send()
            .await
            .context("HTTP request error")?;
        if response.status() == http::StatusCode::OK {
            let response = response.json::<Value>().await.context("JSON error")?;
            if let Some(room_id) = response.get("room_id") {
                if let Some(room_id) = room_id.as_str() {
                    rooms.insert(room_name.to_string(), room_id.to_string());
                }
            }
        }
    }

    let mut sockets: HashMap<String, String> = HashMap::new();
    for (room_name, room_id) in rooms.iter() {
        let devices_url = Url::parse(format!("{}/{}/devices", rooms_url, room_id).as_str())
            .context("URL error")?;

        let socket_url =
            Url::parse(format!("{}/socket", devices_url).as_str()).context("URL error")?;
        for i in 1..=5 {
            let socket_name = format!("{room_name} розетка {i}");

            let new_device = NewSmartDevice::with_name(socket_name.as_str());
            let response = client
                .post(socket_url.as_str())
                .json(&new_device)
                .send()
                .await
                .context("HTTP request error")?;
            if response.status() == http::StatusCode::OK {
                let response = response.json::<Value>().await.context("JSON error")?;
                if let Some(device_id) = response.get("device_id") {
                    if let Some(device_id) = device_id.as_str() {
                        sockets.insert(device_id.to_string(), room_id.clone());
                    }
                }
            }
        }

        let thermometer_url = socket_url.join("thermometer").context("URL error")?;
        for i in 1..=3 {
            let thermometer_name = format!("{room_name} термометр {i}");

            let new_device = NewSmartDevice::with_name(thermometer_name.as_str());
            let _ = client
                .post(thermometer_url.as_str())
                .json(&new_device)
                .send()
                .await
                .context("HTTP request error")?;
        }
    }

    let info_url = Url::parse(format!("{}/info", rooms_url).as_str())?;
    let response = client
        .get(info_url.as_str())
        .send()
        .await
        .context("HTTP request error")?;
    if response.status() == http::StatusCode::OK {
        let response = response.json::<Value>().await.context("JSON error")?;
        if let Some(message) = response.get("message") {
            if let Some(message) = message.as_str() {
                println!("\nСостояние умного дома после создания\n\n{message}\n");
            }
        }
    }

    // Включить все розетки.
    let on_patch = SmartDevicePatch::new().with_state(true);

    for (device_id, room_id) in sockets.iter() {
        let device_url =
            Url::parse(format!("{}/{}/devices/{}", rooms_url, room_id, device_id).as_str())
                .context("URL error")?;

        let _ = client
            .put(device_url.as_str())
            .json(&on_patch)
            .send()
            .await
            .context("HTTP request error")?;
    }

    let response = client
        .get(info_url.as_str())
        .send()
        .await
        .context("HTTP request error")?;
    if response.status() == http::StatusCode::OK {
        let response = response.json::<Value>().await.context("JSON error")?;
        if let Some(message) = response.get("message") {
            if let Some(message) = message.as_str() {
                println!("\nСостояние умного дома после включения розеток\n\n{message}\n");
            }
        }
    }

    // Выключить все розетки.
    let off_patch = SmartDevicePatch::new().with_state(false);

    for (device_id, room_id) in sockets.iter() {
        let device_url =
            Url::parse(format!("{}/{}/devices/{}", rooms_url, room_id, device_id).as_str())
                .context("URL error")?;

        let _ = client
            .put(device_url.as_str())
            .json(&off_patch)
            .send()
            .await
            .context("HTTP request error")?;
    }

    let response = client
        .get(info_url.as_str())
        .send()
        .await
        .context("HTTP request error")?;
    if response.status() == http::StatusCode::OK {
        let response = response.json::<Value>().await.context("JSON error")?;
        if let Some(message) = response.get("message") {
            if let Some(message) = message.as_str() {
                println!("\nСостояние умного дома после выключения розеток\n\n{message}\n");
            }
        }
    }

    Ok(())
}
