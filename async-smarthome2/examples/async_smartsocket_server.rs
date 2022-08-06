use std::sync::atomic::Ordering;

use anyhow::{Context, Result};
use tokio::{fs, signal};

use async_smarthome2::{control::server::SmartSocketServer, device::socket::SmartSocket};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mut socket = SmartSocket::new("Удаленная розетка");
    socket.plug(3000.0);

    let addr = fs::read_to_string("settings/addr")
        .await
        .unwrap_or_else(|_| String::from("127.0.0.1:55333"));
    let (server, control) = SmartSocketServer::bind(addr, socket)
        .await
        .context("Failed to bind a socket")?;

    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();

        if let Some(w) = control.upgrade() {
            (*w).store(false, Ordering::Relaxed);
        }
    });
    server.run().await;

    Ok(())
}
