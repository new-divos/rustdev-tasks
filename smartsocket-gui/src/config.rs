use serde::{Deserialize, Serialize};

///
/// Конфигурация сервера умной розетки.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
}
