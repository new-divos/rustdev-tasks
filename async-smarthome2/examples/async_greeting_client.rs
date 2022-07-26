use std::error::Error;

use async_smarthome2::control::{message::TextMessage, protocol::client::Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::connect("127.0.0.1:55332").await?;
    let response: Box<TextMessage> = client
        .request(TextMessage::new("Hello from client"))
        .await?;
    println!("Message from server: {}", *response);

    Ok(())
}
