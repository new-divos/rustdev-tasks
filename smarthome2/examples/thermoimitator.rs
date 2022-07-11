use std::fs;
use std::sync::atomic::Ordering;

use smarthome2::{
    device::thermometer::{AutonomousThermometer, SmartThermometer},
    error::DeviceError,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let thermometer = SmartThermometer::new("Автономный термометер", 20.0);
    let thermometer = AutonomousThermometer::builder()
        .bind(
            fs::read_to_string("settings/auto_addr")
                .unwrap_or_else(|_| String::from("127.0.0.1:55334")),
        )
        .connect(
            fs::read_to_string("settings/remote_addr")
                .unwrap_or_else(|_| String::from("127.0.0.1:55335")),
        )
        .with_noise()
        .build(thermometer)?;

    let (handle, control) = thermometer.run()?;
    ctrlc::set_handler(move || {
        if let Some(w) = control.upgrade() {
            log::info!("Terminating process ...");
            (*w).store(false, Ordering::Relaxed);
        }
    })?;
    if handle.join().is_err() {
        return Err(Box::new(DeviceError::UnexpectedMessage));
    }

    Ok(())
}
