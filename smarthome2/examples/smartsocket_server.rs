use std::fs;

use smarthome2::{control::server::SmartSocketServer, device::socket::SmartSocket};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut socket = SmartSocket::new("Удаленная розетка");
    socket.plug(3000.0);

    let addr =
        fs::read_to_string("settings/addr").unwrap_or_else(|_| String::from("127.0.0.1:55333"));
    let server = SmartSocketServer::bind(addr, socket)?;
    server.run();

    Ok(())
}
