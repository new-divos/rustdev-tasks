use std::sync::atomic::Ordering;

use anyhow::{Context, Result};
use tokio::{fs, signal};

use async_smarthome2::device::thermometer::{AutonomousThermometer, SmartThermometer};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let thermometer = SmartThermometer::new("Автономный термометер", 20.0);
    let (thermometer, control) = AutonomousThermometer::builder()
        .bind(
            fs::read_to_string("settings/auto_addr")
                .await
                .unwrap_or_else(|_| String::from("127.0.0.1:55334")),
        )
        .connect(
            fs::read_to_string("settings/remote_addr")
                .await
                .unwrap_or_else(|_| String::from("127.0.0.1:55335")),
        )
        .with_noise()
        .build(thermometer)
        .await
        .context("Failed to initialize the autonomous thermometer")?;

    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();

        if let Some(w) = control.upgrade() {
            log::info!("Terminating process ...");
            (*w).store(false, Ordering::Relaxed);
        }
    });

    thermometer
        .run()
        .await
        .context("Failed to run the autonomous thermometer")?;
    Ok(())
}
