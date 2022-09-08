#![allow(clippy::collapsible_match)]
#![allow(clippy::single_match)]

use std::fmt;

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use statrs::distribution::Normal;
use uuid::Uuid;

use crate::error::Error;

///
/// Структура с данными умной розетки из базы данных.
///
#[derive(Debug, Clone, FromRow)]
pub(crate) struct SmartSocketRow {
    ///
    /// Идентификатор умной розетки.
    ///
    pub(crate) id: Uuid,
    ///
    /// Наименование умной розетки.
    ///
    pub(crate) name: String,
    ///
    /// Идентификатор комнаты умного дома.
    ///
    pub(crate) room_id: Uuid,
    ///
    /// Состояние розетки (включена-выключена).
    ///
    pub(crate) state: bool,
    ///
    /// Потребляемая мощность.
    ///
    pub(crate) power: f64,
}

///
/// Структура с данными умного термометра из базы данных.
///
#[derive(Debug, Clone, FromRow)]
pub(crate) struct SmartThermometerRow {
    ///
    /// Идентификатор умного термометра.
    ///
    pub(crate) id: Uuid,
    ///
    /// Наименование умного термометра.
    ///
    pub(crate) name: String,
    ///
    /// Идентификатор комнаты умного дома.
    ///
    pub(crate) room_id: Uuid,
    ///
    /// Показания термометра.
    ///
    pub(crate) temperature: f64,
}

///
/// Перечисление с данными устройства.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SmartDeviceData {
    ///
    /// Данные умной розетки.
    ///
    #[serde(rename = "socket")]
    Socket {
        ///
        /// Наименование умной розетки.
        ///
        name: String,
        ///
        /// Состояние розетки (включена-выключена).
        ///
        state: bool,
        ///
        /// Потребляемая мощность.
        ///
        power: f64,
    },

    ///
    /// Данные термометра.
    ///
    #[serde(rename = "thermometer")]
    Thermometer {
        ///
        /// Наименование умного термометра.
        ///
        name: String,
        ///
        /// Показания термометра.
        ///
        temperature: f64,
    },

    ///
    /// Данные неизвестного устройства.
    ///
    #[serde(other)]
    #[serde(rename = "unknown")]
    Unknown,
}

///
/// Структура, описывающая умное устройство.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmartDevice {
    ///
    /// Идентификатор устройства.
    ///
    device_id: Uuid,

    ///
    /// Идентификатор комнаты умного дома.
    ///
    room_id: Uuid,

    ///
    /// Данные устройства.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    data: Option<SmartDeviceData>,

    ///
    /// Пул запросов.
    ///
    #[serde(skip)]
    pool: Option<SqlitePool>,
}

impl fmt::Display for SmartDevice {
    ///
    /// Получить отчет об умном устройстве.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref data) = self.data {
            match data {
                SmartDeviceData::Socket { name, state, power } => {
                    if *state {
                        write!(
                            f,
                            "Умная розетка {} ({}) в комнате {}. Включена, потребляемая мощность: {} Вт.",
                            self.device_id, name, self.room_id, *power
                        )
                    } else {
                        write!(
                            f,
                            "Умная розетка {} ({}) в комнате {}. Выключена.",
                            self.device_id, name, self.room_id
                        )
                    }
                }

                SmartDeviceData::Thermometer { name, temperature } => write!(
                    f,
                    "Умный термометр {} ({}) в комнате {}. Показания термометра: {} °C.",
                    self.device_id, name, self.room_id, *temperature
                ),

                SmartDeviceData::Unknown => write!(
                    f,
                    "Неизвестное умное устройство {} в комнате {}.",
                    self.device_id, self.room_id
                ),
            }
        } else {
            write!(
                f,
                "Отсутствуют сведения для умного устройства {} в комнате {}.",
                self.device_id, self.room_id
            )
        }
    }
}

impl SmartDevice {
    ///
    /// Создать устройство умного дома.
    ///
    #[inline]
    pub(crate) fn new(device_id: Uuid, room_id: Uuid, pool: SqlitePool) -> Self {
        Self {
            device_id,
            room_id,
            data: None,
            pool: Some(pool),
        }
    }

    ///
    /// Создать устройство умного дома с данными.
    ///
    #[inline]
    pub(crate) fn with_data(
        device_id: Uuid,
        room_id: Uuid,
        data: SmartDeviceData,
        pool: SqlitePool,
    ) -> Self {
        Self {
            device_id,
            room_id,
            data: Some(data),
            pool: Some(pool),
        }
    }

    ///
    /// Получить идентификатор устройства.
    ///
    #[inline]
    pub fn device_id(&self) -> Uuid {
        self.device_id
    }

    ///
    /// Получить идентификатор комнаты умного дома.
    ///
    #[inline]
    pub fn room_id(&self) -> Uuid {
        self.room_id
    }

    ///
    /// Загрузить данные устройства умного дома.
    ///
    pub async fn load(&mut self) -> Result<(), Error> {
        if let Some(ref pool) = self.pool {
            let mut rng = thread_rng();
            let normal = Normal::new(0.0, 1.0).unwrap();

            let socket_data = sqlx::query_as::<_, SmartSocketRow>(
                "
                SELECT * FROM sockets WHERE id = $1 AND room_id = $2;
                ",
            )
            .bind(self.device_id)
            .bind(self.room_id)
            .fetch_optional(pool)
            .await?;

            if let Some(socket_data) = socket_data {
                if socket_data.id != self.device_id || socket_data.room_id != self.room_id {
                    return Err(Error::DataIntegrityError);
                }

                self.data = Some(SmartDeviceData::Socket {
                    name: socket_data.name,
                    state: socket_data.state,
                    power: socket_data.power + rng.sample(normal),
                });

                return Ok(());
            }

            let thermometer_data = sqlx::query_as::<_, SmartThermometerRow>(
                "
                SELECT * FROM thermometers WHERE id = $1 AND room_id = $2;
                ",
            )
            .bind(self.device_id)
            .bind(self.room_id)
            .fetch_optional(pool)
            .await?;

            if let Some(thermometer_data) = thermometer_data {
                if thermometer_data.id != self.device_id || thermometer_data.room_id != self.room_id
                {
                    return Err(Error::DataIntegrityError);
                }

                self.data = Some(SmartDeviceData::Thermometer {
                    name: thermometer_data.name,
                    temperature: thermometer_data.temperature + rng.sample(normal),
                });

                return Ok(());
            }

            Err(Error::IllegalDeviceId(self.device_id, self.room_id))
        } else {
            Err(Error::DataIntegrityError)
        }
    }

    ///
    /// Изменить имя устройства.
    ///
    pub async fn set_name<S: AsRef<str>>(&mut self, name: S) -> Result<(), Error> {
        if let Some(ref pool) = self.pool {
            let mut tx = pool.begin().await?;

            sqlx::query(
                "
                UPDATE sockets SET name = $1 WHERE id = $2 AND room_id = $3;
                ",
            )
            .bind(name.as_ref())
            .bind(self.device_id)
            .bind(self.room_id)
            .execute(&mut tx)
            .await?;

            sqlx::query(
                "
                UPDATE thermometers SET name = $1 WHERE id = $2 AND room_id = $3
                ",
            )
            .bind(name.as_ref())
            .bind(self.device_id)
            .bind(self.room_id)
            .execute(&mut tx)
            .await?;

            tx.commit().await?;

            if let Some(ref mut data) = self.data {
                let device_name = name.as_ref().to_string();

                match data {
                    SmartDeviceData::Socket {
                        name,
                        state: _,
                        power: _,
                    } => *name = device_name,

                    SmartDeviceData::Thermometer {
                        name,
                        temperature: _,
                    } => *name = device_name,

                    SmartDeviceData::Unknown => (),
                }
            }

            Ok(())
        } else {
            Err(Error::DataIntegrityError)
        }
    }

    ///
    /// Изменить состояние устройства
    ///
    pub async fn set_state(&mut self, state: bool) -> Result<(), Error> {
        if let Some(ref pool) = self.pool {
            let mut tx = pool.begin().await?;

            let power = if state {
                let mut rng = thread_rng();

                rng.gen_range(200..=2000) as f64
            } else {
                0.0
            };

            sqlx::query(
                "
                UPDATE sockets SET state = $1, power = $2 WHERE id = $3 AND room_id = $4;
                ",
            )
            .bind(state)
            .bind(power)
            .bind(self.device_id)
            .bind(self.room_id)
            .execute(&mut tx)
            .await?;

            tx.commit().await?;

            if let Some(ref mut data) = self.data {
                match data {
                    SmartDeviceData::Socket {
                        name: _,
                        state: device_state,
                        power: device_power,
                    } => {
                        *device_state = state;
                        *device_power = power;
                    }

                    _ => (),
                }
            }

            Ok(())
        } else {
            Err(Error::DataIntegrityError)
        }
    }

    ///
    /// Удалить устройство умного дома.
    ///
    pub async fn delete(&self) -> Result<(), Error> {
        if let Some(ref pool) = self.pool {
            let mut tx = pool.begin().await?;

            sqlx::query(
                "
                DELETE FROM sockets WHERE id = $1 AND room_id = $2;
                ",
            )
            .bind(self.device_id)
            .bind(self.room_id)
            .execute(&mut tx)
            .await?;

            sqlx::query(
                "
                DELETE FROM thermometers WHERE id = $1 AND room_id = $2;
                ",
            )
            .bind(self.device_id)
            .bind(self.room_id)
            .execute(&mut tx)
            .await?;

            tx.commit().await?;
            Ok(())
        } else {
            Err(Error::DataIntegrityError)
        }
    }
}
