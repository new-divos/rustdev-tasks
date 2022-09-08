use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::model::house::SmartHouse, error::Error, routes::Info};

///
/// Структура с данными нового устройства.
///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewSmartDevice {
    ///
    /// Имя нового устройства.
    ///
    name: String,
}

impl NewSmartDevice {
    ///
    /// Создать данные о новом устройстве.
    ///
    #[inline]
    pub fn with_name<S: AsRef<str>>(name: S) -> Self {
        Self {
            name: name.as_ref().to_string(),
        }
    }

    ///
    /// Получить имя нового устройства.
    ///
    #[inline]
    pub fn device_name(&self) -> &str {
        self.name.as_str()
    }
}

///
/// Структура для обновления информации об устройстве умного дома.
///
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SmartDevicePatch {
    ///
    /// Новое имя устройства умного дома.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    ///
    /// Новый статус устройства умного дома.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<bool>,
}

impl SmartDevicePatch {
    ///
    /// Создать обновление по умолчанию.
    ///
    #[inline]
    pub fn new() -> Self {
        Self {
            name: None,
            state: None,
        }
    }

    ///
    /// Задать имя устройства для обновления.
    ///
    #[inline]
    pub fn with_name<S: AsRef<str>>(self, name: S) -> Self {
        Self {
            name: Some(name.as_ref().to_string()),
            state: self.state,
        }
    }

    ///
    /// Задать статус устройства для обновления.
    ///
    #[inline]
    pub fn with_state(self, state: bool) -> Self {
        Self {
            name: self.name,
            state: Some(state),
        }
    }
}

///
/// Создать новую розетку в комнате умного дома.
///
pub async fn new_socket(
    house: web::Data<SmartHouse>,
    id: web::Path<Uuid>,
    new_device: web::Json<NewSmartDevice>,
) -> Result<HttpResponse, Error> {
    let room_id = *id;
    let room = house.into_inner().get(room_id)?;
    let socket = room.create_socket(new_device.name.as_str()).await?;

    Ok(HttpResponse::Ok().json(socket))
}

///
/// Создать новый термометер в комнате умного дома.
///
pub async fn new_thermometer(
    house: web::Data<SmartHouse>,
    id: web::Path<Uuid>,
    new_device: web::Json<NewSmartDevice>,
) -> Result<HttpResponse, Error> {
    let room_id = *id;
    let room = house.into_inner().get(room_id)?;
    let thermometer = room.create_thermometer(new_device.name.as_str()).await?;

    Ok(HttpResponse::Ok().json(thermometer))
}

///
/// Получить информацию о всех устройствах комнаты умного дома.
///
pub async fn all(house: web::Data<SmartHouse>, id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let room_id = *id;
    let room = house.into_inner().get(room_id)?;
    let devices = room.all().await?;

    Ok(HttpResponse::Ok().json(devices))
}

///
/// Получить информацию об устройстве с заданным идентификатором.
///
pub async fn get(
    house: web::Data<SmartHouse>,
    ids: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, Error> {
    let (room_id, device_id) = *ids;
    let room = house.into_inner().get(room_id)?;
    let mut device = room.get(device_id)?;
    device.load().await?;

    Ok(HttpResponse::Ok().json(device))
}

///
/// Удалить устройство с заданным идентификатором.
///
pub async fn delete(
    house: web::Data<SmartHouse>,
    ids: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, Error> {
    let (room_id, device_id) = *ids;
    let room = house.into_inner().get(room_id)?;
    let device = room.get(device_id)?;
    device.delete().await?;

    Ok(HttpResponse::Ok().json(Info::new(format!("the device {device_id} was deleted"))))
}

///
/// Обновить информацию об устройстве с заданным идентификатором.
///
pub async fn update(
    house: web::Data<SmartHouse>,
    ids: web::Path<(Uuid, Uuid)>,
    patch: web::Json<SmartDevicePatch>,
) -> Result<HttpResponse, Error> {
    let (room_id, device_id) = *ids;
    let room = house.into_inner().get(room_id)?;
    let mut device = room.get(device_id)?;

    match patch.into_inner() {
        SmartDevicePatch {
            name: None,
            state: None,
        } => (),

        SmartDevicePatch {
            name: Some(ref name),
            state: None,
        } => device.set_name(name).await?,

        SmartDevicePatch {
            name: None,
            state: Some(state),
        } => device.set_state(state).await?,

        SmartDevicePatch {
            name: Some(ref name),
            state: Some(state),
        } => {
            device.set_name(name).await?;
            device.set_state(state).await?;
        }
    }

    device.load().await?;
    Ok(HttpResponse::Ok().json(device))
}
