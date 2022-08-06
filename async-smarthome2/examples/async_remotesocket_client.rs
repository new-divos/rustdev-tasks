use anyhow::{Context, Result};
use tokio::fs;

use async_smarthome2::device::{
    socket::{RemoteSmartSocket, SwitchOffEvent, SwitchOnEvent},
    AsyncDevice,
};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = fs::read_to_string("settings/addr")
        .await
        .unwrap_or_else(|_| String::from("127.0.0.1:55333"));

    let mut remote_socket = RemoteSmartSocket::connect(addr)
        .await
        .context("Failed connect to the server")?;

    println!("Удаленная розетка: {}", remote_socket);

    let _ = remote_socket
        .async_notify(Box::pin(SwitchOnEvent::new()))
        .await
        .context("Failed to process notification")?;
    println!("Удаленная розетка после включения: {}", remote_socket);

    let _ = remote_socket
        .async_notify(Box::pin(SwitchOffEvent::new()))
        .await
        .context("Failed to process notification")?;
    println!("Удаленная розетка после выключения: {}", remote_socket);

    Ok(())
}
