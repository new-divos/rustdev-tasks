use std::{
    env, fs,
    io::{stdin, stdout, Read, Write},
    path::PathBuf,
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::Error;

///
/// Конфигурация умного дома.
///
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct HouseConfig {
    // Идентификатор дома.
    #[serde(rename = "ID")]
    id: Uuid,

    // Имя дома.
    #[serde(rename = "Name")]
    name: String,
}

///
/// Конфигурация базы данных.
///
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DatabaseConfig {
    // URL для подключения к базе данных.
    #[serde(rename = "URL")]
    url: String,
}

///
/// Конфигурация программы.
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Настройки дома.
    #[serde(rename = "House")]
    house_config: HouseConfig,

    // Настройки базы данных.
    #[serde(rename = "Database")]
    database_config: DatabaseConfig,
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
    pub const APP_NAME: &'static str = "web-smarthome2";

    ///
    /// Создать конфигурацию приложения.
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
            let house_id = if let Ok(value) = env::var("HOUSE_ID") {
                value
            } else {
                let mut buffer = String::new();
                print!("Введите идентификатор умного дома: ");
                stdout().flush()?;
                let _ = stdin().read_line(&mut buffer)?;

                buffer
            };
            let house_id = Uuid::parse_str(house_id.trim())?;

            let house_name = if let Ok(value) = env::var("HOUSE_NAME") {
                value
            } else {
                let mut buffer = String::new();
                print!("Введите имя умного дома: ");
                stdout().flush()?;
                let _ = stdin().read_line(&mut buffer)?;

                buffer
            };
            let house_name = house_name.trim().to_string();

            let database_url = if let Ok(value) = env::var("DATABASE_URL") {
                value
            } else {
                let database_path = project_dirs.data_local_dir();
                if !database_path.exists() {
                    fs::create_dir_all(database_path)?;
                }
                let database_path = database_path.join(format!("{}.db", Self::APP_NAME));

                let file_url = url::Url::from_file_path(database_path.as_path())
                    .map_err(|_| Error::AppInitError)?;
                file_url.as_str().replace("file://", "sqlite://")
            };

            let config = Self {
                house_config: HouseConfig {
                    id: house_id,
                    name: house_name,
                },

                database_config: DatabaseConfig { url: database_url },
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
    /// Получить идентификатор дома.
    ///
    #[inline]
    pub fn house_id(&self) -> Uuid {
        self.house_config.id
    }

    ///
    /// Получить имя дома.
    ///
    #[inline]
    pub fn house_name(&self) -> &str {
        self.house_config.name.as_str()
    }

    ///
    /// Получить URL для подключения к базе данных.
    ///
    #[inline]
    pub fn database_url(&self) -> &str {
        self.database_config.url.as_str()
    }
}
