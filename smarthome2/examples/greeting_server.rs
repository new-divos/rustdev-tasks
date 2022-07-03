use std::error::Error;

use smarthome2::control::{
    message::TextMessage,
    protocol::server::{Connection, Server},
};

fn main() -> Result<(), Box<dyn Error>> {
    let server = Server::bind("127.0.0.1:55332")?;
    for connection in server.incoming() {
        process(connection?)?;
    }

    Ok(())
}

fn process(mut conn: Connection) -> Result<(), Box<dyn Error>> {
    let req = conn.recv::<TextMessage>()?;
    println!("Message from client: {}", *req);
    conn.send(TextMessage::new("Hello from server"))?;

    Ok(())
}
