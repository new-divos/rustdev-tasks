use std::sync::Arc;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::{config::Config, error::Error};

///
/// Структура, описывающая умный дом, будет также определять состояние приложения.
///
#[derive(Debug)]
pub struct SmartHouse {
    id: Uuid,
    pool: SqlitePool,
}

impl SmartHouse {
    ///
    /// Создать умный дом с заданной конфигурацией.
    ///
    pub async fn with_config(config: Config) -> Result<Arc<Self>, Error> {
        let pool = SqlitePool::connect(config.database_url()).await?;

        sqlx::query(
            "
            INSERT OR IGNORE INTO houses VALUES($1, $2);
        ",
        )
        .bind(config.house_id())
        .bind(config.house_name())
        .execute(&pool)
        .await?;

        Ok(Arc::new(Self {
            id: config.house_id(),
            pool,
        }))
    }

    ///
    /// Создать новую комнату умного дома с уникальным именем.
    ///
    pub async fn create_room<S: AsRef<str>>(&self, name: S) -> Result<Room, Error> {
        let room_name = name.as_ref().to_string();

        let rooms = sqlx::query_as::<_, Room>(
            "
            SELECT * FROM rooms WHERE house_id = $1 AND name = $2;
            ",
        )
        .bind(self.id)
        .bind(room_name.as_str())
        .fetch_all(&self.pool)
        .await?;

        if !rooms.is_empty() {
            return Err(Error::IllegalRoomName(room_name));
        }

        let room_id = Uuid::new_v4();
        sqlx::query(
            "
            INSERT INTO rooms VALUES($1, $2, $3);
            ",
        )
        .bind(room_id)
        .bind(room_name.as_str())
        .bind(self.id)
        .execute(&self.pool)
        .await?;

        Ok(Room {
            id: room_id,
            name: room_name,
            house_id: self.id,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NewRoom {
    name: String,
}

impl NewRoom {
    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, FromRow)]
pub struct Room {
    id: Uuid,
    name: String,
    house_id: Uuid,
}
