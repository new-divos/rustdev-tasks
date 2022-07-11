use std::{
    fs,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread, time,
};

use smarthome2::device::thermometer::RemoteThermometer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let thermometer = RemoteThermometer::builder()
        .with_name("Удаленный термометр")
        .bind(
            fs::read_to_string("settings/remote_addr")
                .unwrap_or_else(|_| String::from("127.0.0.1:55335")),
        )
        .connect(
            fs::read_to_string("settings/auto_addr")
                .unwrap_or_else(|_| String::from("127.0.0.1:55334")),
        )
        .build();

    let duration = time::Duration::from_secs(1);

    let working = Arc::new(AtomicBool::new(true));
    let control = Arc::downgrade(&working);
    ctrlc::set_handler(move || {
        if let Some(w) = control.upgrade() {
            (*w).store(false, Ordering::Relaxed);
        }
    })?;

    while (*working).load(Ordering::Relaxed) {
        println!("Состояние термометра: {}", thermometer);
        thread::sleep(duration);
    }

    Ok(())
}
