use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::{config::Config, error::Error};

///
/// Структура, описывающая умный дом, будет также определять состояние приложения.
///
#[derive(Debug, Clone)]
pub struct SmartHouse {
    id: Uuid,
    pool: SqlitePool,
}

impl SmartHouse {
    ///
    /// Создать умный дом с заданной конфигурацией.
    ///
    pub async fn with_config(config: Config) -> Result<Self, Error> {
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

        Ok(Self {
            id: config.house_id(),
            pool,
        })
    }

    ///
    /// Создать новую комнату умного дома с уникальным именем.
    ///
    pub async fn create_room<S: AsRef<str>>(&self, room_name: S) -> Result<Room, Error> {
        let room_name = room_name.as_ref().to_string();

        let rooms = sqlx::query_as::<_, RoomRow>(
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
            pool: self.pool.clone(),
        })
    }

    ///
    /// Получить список всех комнат умного дома.
    ///
    pub async fn all_rooms(&self) -> Result<Vec<Room>, Error> {
        let mut rooms: Vec<_> = sqlx::query_as::<_, RoomRow>(
            "
            SELECT * FROM rooms WHERE house_id = $1;
            ",
        )
        .bind(self.id)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|rr| Room {
            id: rr.id,
            name: rr.name,
            house_id: self.id,
            pool: self.pool.clone(),
        })
        .collect();

        rooms.shrink_to_fit();
        Ok(rooms)
    }

    ///
    /// Получить комнату умного дома по идентификатору.
    ///
    pub async fn get_room(&self, room_id: Uuid) -> Result<Room, Error> {
        let room = sqlx::query_as::<_, RoomRow>(
            "
            SELECT * FROM rooms WHERE id = $1;
            ",
        )
        .bind(room_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Room {
            id: room.id,
            name: room.name,
            house_id: self.id,
            pool: self.pool.clone(),
        })
    }

    ///
    /// Удалить комнату умного дома по идентификатору.
    ///
    pub async fn delete_room(&self, room_id: Uuid) -> Result<(), Error> {
        sqlx::query(
            "
            DELETE FROM rooms WHERE id = $1;
            ",
        )
        .bind(room_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    ///
    /// Установить имя комнаты умного дома по идентификатору.
    ///
    pub async fn update_room<S: AsRef<str>>(
        &self,
        room_id: Uuid,
        room_name: S,
    ) -> Result<Room, Error> {
        let room_name = room_name.as_ref().to_string();

        sqlx::query(
            "
            UPDATE rooms SET name = $1 WHERE id = $2;
            ",
        )
        .bind(room_name.as_str())
        .bind(room_id)
        .execute(&self.pool)
        .await?;

        Ok(Room {
            id: room_id,
            name: room_name,
            house_id: self.id,
            pool: self.pool.clone(),
        })
    }
}

///
/// Структура с данными о новой комнате.
///
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NewRoom {
    ///
    /// Имя новой комнаты.
    ///
    name: String,
}

impl NewRoom {
    ///
    /// Получить имя новой комнаты.
    ///
    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

///
/// Структура с информацией о комнате в базе данных.
///
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, FromRow)]
struct RoomRow {
    id: Uuid,
    name: String,
    house_id: Uuid,
}

#[derive(Clone, Debug)]
pub struct Room {
    id: Uuid,
    name: String,
    house_id: Uuid,
    pool: SqlitePool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct RoomInfo {
    id: Uuid,
    name: String,
    house_id: Uuid,
}

impl From<Room> for RoomInfo {
    #[inline]
    fn from(room: Room) -> Self {
        Self {
            id: room.id,
            name: room.name,
            house_id: room.house_id,
        }
    }
}
