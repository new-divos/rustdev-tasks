use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::{
    db::model::device::{SmartDevice, SmartDeviceData, SmartSocketRow, SmartThermometerRow},
    error::Error,
};

///
/// Структура с данными комнаты умного дома из базы данных.
///
#[derive(Debug, Clone, FromRow)]
pub(crate) struct SmartRoomRow {
    ///
    /// Идентификатор комнаты умного дома.
    ///
    pub(crate) id: Uuid,
    ///
    /// Наименование комнаты умного дома.
    ///
    pub(crate) name: String,
    ///
    /// Идентификатор умного дома.
    ///
    pub(crate) house_id: Uuid,
}

///
/// Структура с данными комнаты умного дома.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRoomData {
    ///
    /// Наименование комнаты умного дома.
    ///
    pub(crate) name: String,
    ///
    /// Устройства комнаты умного дома.
    ///
    pub(crate) devices: Vec<SmartDevice>,
}

///
/// Структура, описывающая комнату умного дома.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartRoom {
    ///
    /// Идентификатор комнаты умного дома.
    ///
    room_id: Uuid,

    ///
    /// Идентификатор умного дома.
    ///
    house_id: Uuid,

    ///
    /// Данные комнаты умного дома.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    data: Option<SmartRoomData>,

    ///
    /// Пул запросов.
    ///
    #[serde(skip)]
    pool: Option<SqlitePool>,
}

impl SmartRoom {
    ///
    /// Создать комнату умного дома.
    ///
    #[inline]
    pub(crate) fn new(room_id: Uuid, house_id: Uuid, pool: SqlitePool) -> Self {
        Self {
            room_id,
            house_id,
            data: None,
            pool: Some(pool),
        }
    }

    ///
    /// Получить идентификатор комнаты умного дома
    ///
    #[inline]
    pub fn room_id(&self) -> Uuid {
        self.room_id
    }

    ///
    /// Получить идентификатор умного дома.
    ///
    pub fn house_id(&self) -> Uuid {
        self.house_id
    }

    ///
    /// Создать умную розетку в комнате умного дома.
    ///
    pub async fn create_socket<S: AsRef<str>>(&self, name: S) -> Result<SmartDevice, Error> {
        if let Some(ref pool) = self.pool {
            let device_name = name.as_ref().to_string();

            let mut tx = pool.begin().await?;

            let sockets = sqlx::query_as::<_, SmartSocketRow>(
                "
                SELECT * FROM sockets WHERE name = $1 AND room_id = $2;
                ",
            )
            .bind(device_name.as_str())
            .bind(self.room_id)
            .fetch_all(&mut tx)
            .await?;

            if !sockets.is_empty() {
                return Err(Error::IllegalSocketName(device_name));
            }

            let device_id = Uuid::new_v4();
            sqlx::query(
                "
                INSERT INTO sockets VALUES ($1, $2, $3, FALSE, 0.0);
                ",
            )
            .bind(device_id)
            .bind(device_name.as_str())
            .bind(self.room_id)
            .execute(&mut tx)
            .await?;

            tx.commit().await?;

            Ok(SmartDevice::with_data(
                device_id,
                self.room_id,
                SmartDeviceData::Socket {
                    name: device_name,
                    state: false,
                    power: 0.0,
                },
                pool.clone(),
            ))
        } else {
            Err(Error::DataIntegrityError)
        }
    }

    ///
    /// Создать умный термометр в комнате умного дома.
    ///
    pub async fn create_thermometer<S: AsRef<str>>(&self, name: S) -> Result<SmartDevice, Error> {
        if let Some(ref pool) = self.pool {
            let mut rng = thread_rng();
            let device_name = name.as_ref().to_string();

            let mut tx = pool.begin().await?;

            let thermometers = sqlx::query_as::<_, SmartThermometerRow>(
                "
                SELECT * FROM thermometers WHERE name = $1 AND room_id = $2;
                ",
            )
            .bind(device_name.as_str())
            .bind(self.room_id)
            .fetch_all(&mut tx)
            .await?;

            if !thermometers.is_empty() {
                return Err(Error::IllegalThermometerName(device_name));
            }

            let device_id = Uuid::new_v4();
            let temperature = rng.gen_range(10..=50) as f64;
            sqlx::query(
                "
                INSERT INTO thermometers VALUES ($1, $2, $3, $4);
                ",
            )
            .bind(device_id)
            .bind(device_name.as_str())
            .bind(self.room_id)
            .bind(temperature)
            .execute(&mut tx)
            .await?;

            tx.commit().await?;
            Ok(SmartDevice::with_data(
                device_id,
                self.room_id,
                SmartDeviceData::Thermometer {
                    name: device_name,
                    temperature,
                },
                pool.clone(),
            ))
        } else {
            Err(Error::DataIntegrityError)
        }
    }

    ///
    /// Получить устройство с заданным идентификатором для комнаты умного дома.
    ///
    #[inline]
    pub fn get(&self, device_id: Uuid) -> Result<SmartDevice, Error> {
        if let Some(ref pool) = self.pool {
            Ok(SmartDevice::new(device_id, self.room_id, pool.clone()))
        } else {
            Err(Error::DataIntegrityError)
        }
    }

    ///
    /// Получить все устройства комнаты умного дома.
    ///
    pub async fn all(&self) -> Result<Vec<SmartDevice>, Error> {
        if let Some(ref pool) = self.pool {
            let mut devices = sqlx::query_as::<_, SmartSocketRow>(
                "
            SELECT * FROM sockets WHERE room_id = $1;
            ",
            )
            .bind(self.room_id)
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|r| {
                SmartDevice::with_data(
                    r.id,
                    r.room_id,
                    SmartDeviceData::Socket {
                        name: r.name,
                        state: r.state,
                        power: r.power,
                    },
                    pool.clone(),
                )
            })
            .collect::<Vec<_>>();

            devices.extend(
                sqlx::query_as::<_, SmartThermometerRow>(
                    "
                    SELECT * FROM thermometers WHERE room_id = $1;
                    ",
                )
                .bind(self.room_id)
                .fetch_all(pool)
                .await?
                .into_iter()
                .map(|r| {
                    SmartDevice::with_data(
                        r.id,
                        r.room_id,
                        SmartDeviceData::Thermometer {
                            name: r.name,
                            temperature: r.temperature,
                        },
                        pool.clone(),
                    )
                }),
            );

            devices.shrink_to_fit();
            Ok(devices)
        } else {
            Err(Error::DataIntegrityError)
        }
    }

    ///
    /// Загрузить данные комнаты умного дома.
    ///
    pub async fn load(&mut self) -> Result<(), Error> {
        if let Some(ref pool) = self.pool {
            let room = sqlx::query_as::<_, SmartRoomRow>(
                "
                SELECT * FROM rooms WHERE id = $1 AND house_id = $2;
                ",
            )
            .bind(self.room_id)
            .bind(self.house_id)
            .fetch_optional(pool)
            .await?;

            if let Some(room) = room {
                if room.id != self.room_id || room.house_id != self.house_id {
                    return Err(Error::DataIntegrityError);
                }

                self.data = Some(SmartRoomData {
                    name: room.name,
                    devices: self.all().await?,
                });

                return Ok(());
            }

            Err(Error::IllegalRoomId(self.room_id))
        } else {
            Err(Error::DataIntegrityError)
        }
    }

    ///
    /// Изменить имя комнаты.
    ///
    pub async fn set_name<S: AsRef<str>>(&mut self, name: S) -> Result<(), Error> {
        if let Some(ref pool) = self.pool {
            let mut tx = pool.begin().await?;

            sqlx::query(
                "
                UPDATE rooms SET name = $1 WHERE id = $2 AND house_id = $3;
                ",
            )
            .bind(name.as_ref())
            .bind(self.room_id)
            .bind(self.house_id)
            .execute(&mut tx)
            .await?;

            tx.commit().await?;

            if let Some(ref mut data) = self.data {
                data.name = name.as_ref().to_string();
            }

            Ok(())
        } else {
            Err(Error::DataIntegrityError)
        }
    }

    ///
    /// Удалить комнату умного дома.
    ///
    pub async fn delete(&self) -> Result<(), Error> {
        if let Some(ref pool) = self.pool {
            let mut tx = pool.begin().await?;

            sqlx::query(
                "
                DELETE FROM rooms WHERE id = $1 AND house_id = $2;
                ",
            )
            .bind(self.room_id)
            .bind(self.house_id)
            .execute(&mut tx)
            .await?;

            tx.commit().await?;
            Ok(())
        } else {
            Err(Error::DataIntegrityError)
        }
    }
}
