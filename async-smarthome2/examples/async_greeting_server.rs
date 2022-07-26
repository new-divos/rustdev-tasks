use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tokio::signal;

use async_smarthome2::control::{
    message::TextMessage,
    protocol::server::{Connection, Server},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::bind("127.0.0.1:55332").await?;

    let working = Arc::new(AtomicBool::new(true));
    let control = Arc::downgrade(&working);

    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();

        if let Some(w) = control.upgrade() {
            (*w).store(false, Ordering::Relaxed);
        }
    });

    while (*working).load(Ordering::Relaxed) {
        let connection = server.accept().await?;
        process(connection).await?;
    }

    Ok(())
}

async fn process(conn: Connection) -> Result<(), Box<dyn std::error::Error>> {
    let req = conn.recv::<TextMessage>().await?;
    println!("Message from client: {}", *req);
    conn.send(TextMessage::new("Hello from server")).await?;

    Ok(())
}
