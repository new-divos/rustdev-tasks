use anyhow::{Context, Result};

use async_smarthome2::control::{message::TextMessage, protocol::client::Client};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::connect("127.0.0.1:55332")
        .await
        .context("Failed to connect to the server")?;

    let response: Box<TextMessage> = client
        .request(TextMessage::new("Hello from client"))
        .await
        .context("Failed to process a request")?;
    println!("Message from server: {}", *response);

    Ok(())
}
