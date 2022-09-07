use std::iter::repeat_with;

use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::{
    config::Config,
    db::model::room::{SmartRoom, SmartRoomData, SmartRoomRow},
    error::Error,
};

///
/// Структура, описывающая умный дом, будет также определять состояние приложения.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartHouse {
    ///
    /// Идентификатор умного дома.
    ///
    house_id: Uuid,

    ///
    /// Наименование умного дома.
    ///
    house_name: String,

    ///
    /// Список комнат умного дома.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    rooms: Option<SmartRoom>,

    ///
    /// Пул запросов.
    ///
    #[serde(skip)]
    pool: Option<SqlitePool>,
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
            house_id: config.house_id(),
            house_name: config.house_name().to_string(),
            rooms: None,
            pool: Some(pool),
        })
    }

    ///
    /// Получить идентификатор умного дома.
    ///
    #[inline]
    pub fn house_id(&self) -> Uuid {
        self.house_id
    }

    ///
    /// Получить наименование умного дома.
    ///
    #[inline]
    pub fn house_name(&self) -> &str {
        self.house_name.as_str()
    }

    ///
    /// Создать комнату умного дома.
    ///
    pub async fn create_room<S: AsRef<str>>(&self, name: S) -> Result<SmartRoom, Error> {
        if let Some(ref pool) = self.pool {
            let mut tx = pool.begin().await?;
            let room_name = name.as_ref().to_string();

            let rooms = sqlx::query_as::<_, SmartRoomRow>(
                "
                SELECT * FROM rooms WHERE name = $1 AND house_id = $2;
                ",
            )
            .bind(room_name.as_str())
            .bind(self.house_id)
            .fetch_all(&mut tx)
            .await?;

            if !rooms.is_empty() {
                tx.rollback().await?;
                return Err(Error::IllegalRoomName(room_name));
            }

            let room_id = Uuid::new_v4();
            sqlx::query(
                "
                INSERT INTO rooms VALUES ($1, $2, $3);
                ",
            )
            .bind(room_id)
            .bind(room_name.as_str())
            .bind(self.house_id)
            .execute(&mut tx)
            .await?;

            tx.commit().await?;

            Ok(SmartRoom::with_name(
                room_id,
                self.house_id,
                room_name,
                pool.clone(),
            ))
        } else {
            Err(Error::DataIntegrityError)
        }
    }

    ///
    /// Получить комнату умного дома по идентификатору.
    ///
    #[inline]
    pub fn get(&self, room_id: Uuid) -> Result<SmartRoom, Error> {
        if let Some(ref pool) = self.pool {
            Ok(SmartRoom::new(room_id, self.house_id, pool.clone()))
        } else {
            Err(Error::DataIntegrityError)
        }
    }

    ///
    /// Получить все комнаты умного дома.
    ///
    pub async fn all(&self) -> Result<Vec<SmartRoom>, Error> {
        if let Some(ref pool) = self.pool {
            let mut rooms = stream::iter(
                sqlx::query_as::<_, SmartRoomRow>(
                    "
                SELECT * FROM rooms WHERE house_id = $1;
                ",
                )
                .bind(self.house_id)
                .fetch_all(pool)
                .await?
                .into_iter(),
            )
            .zip(stream::iter(repeat_with(|| pool.clone())))
            .then(|(r, pool)| async move {
                let mut room = SmartRoom::new(r.id, r.house_id, pool);
                let devices = room.all().await?;

                room.data = Some(SmartRoomData {
                    name: r.name,
                    devices,
                });
                Ok(room) as Result<SmartRoom, Error>
            })
            .filter_map(|e| async move { e.ok() })
            .collect::<Vec<_>>()
            .await;

            rooms.shrink_to_fit();
            Ok(rooms)
        } else {
            Err(Error::DataIntegrityError)
        }
    }

    ///
    /// Удалить все комнаты умного дома.
    ///
    pub async fn delete(&mut self) -> Result<(), Error> {
        if let Some(ref pool) = self.pool {
            let mut tx = pool.begin().await?;

            sqlx::query(
                "
                DELETE FROM rooms WHERE house_id = $1;
                ",
            )
            .bind(self.house_id)
            .execute(&mut tx)
            .await?;

            tx.commit().await?;
            self.rooms = None;

            Ok(())
        } else {
            Err(Error::DataIntegrityError)
        }
    }
}
