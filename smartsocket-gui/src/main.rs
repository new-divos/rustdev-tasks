use anyhow::{Context, Result};
use iced::{Application, Settings};

use smartsocket_gui::{application::SmartSocketClient, config::Config};

fn main() -> Result<()> {
    let config = Config::new().context("Configuration error")?;

    SmartSocketClient::run(Settings {
        flags: config,
        default_font: Some(include_bytes!("../fonts/a_Assuan Medium.ttf")),
        window: iced::window::Settings {
            size: (550, 300),
            ..Default::default()
        },
        ..Default::default()
    })
    .context("Application error")
}
