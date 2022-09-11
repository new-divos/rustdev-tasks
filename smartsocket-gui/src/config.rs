use std::{
    env, fs,
    io::{Read, Write},
    path::PathBuf,
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::error::Error;

///
/// Конфигурация сервера умной розетки.
///
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct SmartSocketServerConfig {
    ///
    /// IP адрес сервера умной розетки.
    ///
    #[serde(rename = "IP")]
    addr: String,

    ///
    /// Прослушиваемый сервером умной розетки порт.
    ///
    #[serde(rename = "Port")]
    port: i32,
}

///
/// Конфигурация программы.
///
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    ///
    /// Конфигурация сервера умной розетки.
    ///
    #[serde(rename = "Smart Socket Server")]
    server_config: SmartSocketServerConfig,
}

impl Config {
    ///
    /// Квалификатор приложения.
    ///
    pub const APP_QUALIFIER: &'static str = "ru";

    ///
    /// Автор приложения.
    ///
    pub const APP_AUTHOR: &'static str = "new-divos";

    ///
    /// Наимнование приложения.
    ///
    pub const APP_NAME: &'static str = "smartsocket-gui";

    ///
    /// Получить конфигурацию программы.
    ///
    pub fn new() -> Result<Self, Error> {
        let project_dirs = ProjectDirs::from(Self::APP_QUALIFIER, Self::APP_AUTHOR, Self::APP_NAME)
            .ok_or(Error::AppInitError)?;

        let config_path = match env::var("CONFIG_PATH") {
            Ok(p) => PathBuf::from(p),
            Err(_) => project_dirs
                .config_dir()
                .join(format!("{}.toml", Self::APP_NAME)),
        };
        if let Some(parent_path) = config_path.parent() {
            if !parent_path.exists() {
                fs::create_dir_all(parent_path)?;
            }
        }

        if !config_path.exists() {
            let config = Config {
                server_config: SmartSocketServerConfig {
                    addr: "127.0.0.1".to_string(),
                    port: 55333,
                },
            };

            let content = toml::to_string(&config)?;
            {
                let mut file = fs::File::create(config_path.as_path())?;
                file.write_all(content.as_bytes())?;
            }
        }

        let mut buffer = String::new();
        {
            let mut file = fs::File::open(config_path.as_path())?;
            file.read_to_string(&mut buffer)?;
        }

        Ok(toml::from_str::<Self>(&buffer)?)
    }

    ///
    /// Получить IP адрес сервера умной розетки.
    ///
    #[inline]
    pub fn server_addr(&self) -> &str {
        self.server_config.addr.as_str()
    }

    ///
    /// Получить порт сервера умной розетки.
    ///
    #[inline]
    pub fn server_port(&self) -> i32 {
        self.server_config.port
    }

    ///
    /// Получить адрес для подключения к серверу.
    ///
    #[inline]
    pub fn server_addrs(&self) -> String {
        format!(
            "{}:{}",
            self.server_config.addr.as_str(),
            self.server_config.port
        )
    }
}
