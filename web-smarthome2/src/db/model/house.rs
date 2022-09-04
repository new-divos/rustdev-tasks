use std::iter::FromIterator;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::{
    config::Config,
    db::model::thermometer::{ThermometerInfo, ThermometerRow},
    error::Error,
};

///
/// Структура, описывающая умный дом, будет также определять состояние приложения.
///
#[derive(Debug, Clone)]
pub struct SmartHouse {
    id: Uuid,
    name: String,
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
            name: config.house_name().to_string(),
            pool,
        })
    }

    ///
    /// Получить идентификатор умного дома.
    ///
    #[inline]
    pub fn house_id(&self) -> Uuid {
        self.id
    }

    ///
    /// Получить наименование умного дома.
    ///
    #[inline]
    pub fn house_name(&self) -> &str {
        self.name.as_str()
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
    ///
    /// Идентификатор комнаты умного дома.
    ///
    id: Uuid,

    ///
    /// Наименование комнаты умного дома.
    ///
    name: String,

    ///
    /// Идентификатор умного дома.
    ///
    house_id: Uuid,
}

///
/// Структура, описывающая состояние комнаты умного дома.
///
#[derive(Clone, Debug)]
pub struct Room {
    ///
    /// Идентификатор комнаты умного дома.
    ///
    id: Uuid,

    ///
    /// Наименование комнаты умного дома.
    ///
    name: String,

    ///
    /// Идентификатор умного дома.
    ///
    house_id: Uuid,

    ///
    /// Пул запросов SQL.
    ///
    pool: SqlitePool,
}

impl Room {
    ///
    /// Получить идентификатор комнаты умного дома.
    ///
    #[inline]
    pub fn room_id(&self) -> Uuid {
        self.id
    }

    ///
    /// Получить наименование комнаты умного дома.
    ///
    #[inline]
    pub fn room_name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Создать новый термометр в комнате умного дома с уникальным именем
    /// и начальным значением температуры.
    ///
    pub async fn create_thermometer<S: AsRef<str>>(
        &self,
        name: S,
        temperature: f64,
    ) -> Result<ThermometerInfo, Error> {
        let thermometer_name = name.as_ref().to_string();

        let thermometers: Vec<_> = sqlx::query_as::<_, ThermometerRow>(
            "
            SELECT * FROM thermometers 
            JOIN rooms ON room.id = thermometers.room_id
            WHERE room.house_id = $1;
            ",
        )
        .bind(self.house_id)
        .fetch_all(&self.pool)
        .await?;

        if !thermometers.is_empty() {
            return Err(Error::IllegalThermometerName(thermometer_name));
        }

        let thermometer_id = Uuid::new_v4();
        sqlx::query(
            "
            INSERT INTO thermometers VALUES($1, $2, $3, $4);
            ",
        )
        .bind(thermometer_id)
        .bind(thermometer_name.as_str())
        .bind(self.id)
        .bind(temperature)
        .execute(&self.pool)
        .await?;

        Ok(ThermometerInfo::new(
            thermometer_id,
            thermometer_name,
            temperature,
        ))
    }

    ///
    /// Получить информацию о всех термометрах в комнате.
    ///
    pub async fn all_thermometers(&self) -> Result<Vec<ThermometerInfo>, Error> {
        let mut thermometers: Vec<_> = sqlx::query_as::<_, ThermometerRow>(
            "
            SELECT * FROM thermometers WHERE room_id = $1;
            ",
        )
        .bind(self.id)
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(ThermometerInfo::from)
        .collect();

        thermometers.shrink_to_fit();
        Ok(thermometers)
    }
}

///
/// Информация о комнате умного дома.
///
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RoomInfo {
    ///
    /// Идентификатор комнаты умного дома.
    ///
    id: Uuid,

    ///
    /// Наименование комнаты умного дома.
    ///
    name: String,

    ///
    /// Информация о термометрах в комнате.
    ///
    thermometers: Vec<ThermometerInfo>,
}

impl RoomInfo {
    ///
    /// Получить информацию о комнате умного дома.
    ///
    pub async fn with_room(room: Room) -> Result<Self, Error> {
        let thermometers = room.all_thermometers().await?;

        Ok(Self {
            id: room.id,
            name: room.name,
            thermometers,
        })
    }
}

///
/// Информация об умном доме.
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HouseInfo {
    ///
    /// Идентификатор умного дома.
    ///
    id: Uuid,

    ///
    /// Наименование умного дома.
    ///
    name: String,

    ///
    /// Список комнат умного дома.
    ///
    rooms: Vec<RoomInfo>,
}

impl HouseInfo {
    ///
    /// Сформировать информацию об умном доме.
    ///
    #[inline]
    pub fn new<S: AsRef<str>>(id: Uuid, name: S, rooms: impl Iterator<Item = RoomInfo>) -> Self {
        Self {
            id,
            name: name.as_ref().to_string(),
            rooms: Vec::<RoomInfo>::from_iter(rooms),
        }
    }
}
