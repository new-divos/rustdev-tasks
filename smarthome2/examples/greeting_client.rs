use std::error::Error;

use smarthome2::control::{message::TextMessage, protocol::client::Client};

fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::connect("127.0.0.1:55332")?;
    let response: Box<TextMessage> = client.request(TextMessage::new("Hello from client"))?;
    println!("Message from server: {}", *response);

    Ok(())
}
