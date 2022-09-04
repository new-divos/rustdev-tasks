use std::iter::FromIterator;

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use statrs::distribution::Normal;
use uuid::Uuid;

///
/// Структура с данными о новом термометре.
///
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NewThermometer {
    ///
    /// Имя нового термометра.
    ///
    name: String,

    ///
    /// Начальное значение температуры нового термометра.
    ///
    temperature: f64,
}

impl NewThermometer {
    ///
    /// Получить имя нового термометра.
    ///
    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Получить начальное значение температуры.
    ///
    #[inline]
    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}

///
/// Структура с информацией о комнате в базе данных.
///
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, FromRow)]
pub(crate) struct ThermometerRow {
    ///
    /// Идентификатор умного термометра.
    ///
    id: Uuid,

    ///
    /// Наименование умного термометра.
    ///
    name: String,

    ///
    /// Идентификатор комнаты умного дома.
    ///
    room_id: Uuid,

    ///
    /// Значение температуры.
    ///
    temperature: f64,
}

///
/// Структура с информацией об умном термометре.
///
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ThermometerInfo {
    ///
    /// Идентификатор умного термометра.
    ///
    id: Uuid,

    ///
    /// Наименование умного термометра.
    ///
    name: String,

    ///
    /// Значение температуры.
    ///
    temperature: f64,
}

impl From<ThermometerRow> for ThermometerInfo {
    ///
    /// Преобразовать данные из базы данных в информацию для отображения.
    ///
    fn from(thermometer: ThermometerRow) -> Self {
        let mut rng = thread_rng();
        let normal = Normal::new(0.0, 1.0).unwrap();

        Self {
            id: thermometer.id,
            name: thermometer.name,
            temperature: thermometer.temperature + rng.sample(normal),
        }
    }
}

impl ThermometerInfo {
    ///
    /// Создать экземпляр структуры с информацией об умном термометре.
    ///
    #[inline]
    pub fn new<S: AsRef<str>>(id: Uuid, name: S, temperature: f64) -> Self {
        Self {
            id,
            name: name.as_ref().to_string(),
            temperature,
        }
    }
}

///
/// Структура с информацией об умном термометре.
///
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ThermometersInfo {
    ///
    /// Идентификатор комнаты умного дома.
    ///
    id: Uuid,

    ///
    /// Наименование комнаты умного дома.
    ///
    name: String,

    ///
    /// Список с информацией о термометрах.
    ///
    thermometers: Vec<ThermometerInfo>,
}

impl ThermometersInfo {
    ///
    /// Сформировать информацию о списке термометров.
    ///
    #[inline]
    pub fn new<S: AsRef<str>>(
        id: Uuid,
        name: S,
        thermometers: impl Iterator<Item = ThermometerInfo>,
    ) -> Self {
        Self {
            id,
            name: name.as_ref().to_string(),
            thermometers: Vec::<ThermometerInfo>::from_iter(thermometers),
        }
    }
}
