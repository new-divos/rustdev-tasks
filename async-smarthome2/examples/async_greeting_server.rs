use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use anyhow::{Context, Result};
use tokio::signal;

use async_smarthome2::control::{
    message::TextMessage,
    protocol::server::{Connection, Server},
};

#[tokio::main]
async fn main() -> Result<()> {
    let server = Server::bind("127.0.0.1:55332")
        .await
        .context("Failed to bind a socket")?;

    let working = Arc::new(AtomicBool::new(true));
    let control = Arc::downgrade(&working);

    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();

        if let Some(w) = control.upgrade() {
            (*w).store(false, Ordering::Relaxed);
        }
    });

    while (*working).load(Ordering::Relaxed) {
        let connection = server
            .accept()
            .await
            .context("Failed to connect to the server")?;

        process(connection).await?;
    }

    Ok(())
}

async fn process(conn: Connection) -> Result<()> {
    let req = conn
        .recv::<TextMessage>()
        .await
        .context("Failed to receive a request")?;

    println!("Message from client: {}", *req);

    conn.send(TextMessage::new("Hello from server"))
        .await
        .context("Failed to send a response")?;

    Ok(())
}
