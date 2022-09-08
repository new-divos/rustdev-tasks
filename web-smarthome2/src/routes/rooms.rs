use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db::model::house::SmartHouse, error::Error, routes::Info};

///
/// Структура с данными новой комнаты.
///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewSmartRoom {
    ///
    /// Имя новой комнаты.
    ///
    name: String,
}

impl NewSmartRoom {
    ///
    /// Создать данные о новой комнате.
    ///
    #[inline]
    pub fn with_name<S: AsRef<str>>(name: S) -> Self {
        Self {
            name: name.as_ref().to_string(),
        }
    }

    ///
    /// Получить имя новой комнаты.
    ///
    #[inline]
    pub fn room_name(&self) -> &str {
        self.name.as_str()
    }
}

///
/// Структура для обновления информации о комнате умного дома.
///
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SmartRoomPatch {
    ///
    /// Новое имя комнаты умного дома.
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl SmartRoomPatch {
    ///
    /// Задать имя комнаты для обновления.
    ///
    #[inline]
    pub fn with_name<S: AsRef<str>>(name: S) -> Self {
        Self {
            name: Some(name.as_ref().to_string()),
        }
    }

    ///
    /// Получить имя комнаты для обновления.
    ///
    #[inline]
    pub fn room_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

///
/// Создать новую комнату умного дома.
///
pub async fn new(
    house: web::Data<SmartHouse>,
    new_room: web::Json<NewSmartRoom>,
) -> Result<HttpResponse, Error> {
    let room = house
        .into_inner()
        .create_room(new_room.name.as_str())
        .await?;
    Ok(HttpResponse::Ok().json(room))
}

///
/// Получить данные о всех комнатах и устройствах умного дома.
///
pub async fn all(house: web::Data<SmartHouse>) -> Result<HttpResponse, Error> {
    let rooms = house.into_inner().all().await?;
    Ok(HttpResponse::Ok().json(rooms))
}

///
/// Удалить все комнаты и устройства умного дома.
///
pub async fn delete_all(house: web::Data<SmartHouse>) -> Result<HttpResponse, Error> {
    house.into_inner().delete().await?;
    Ok(HttpResponse::Ok().json(Info::new("all rooms and devices were deleted")))
}

///
/// Получить информацию о комнате умного дома.
///
pub async fn get(house: web::Data<SmartHouse>, id: web::Path<Uuid>) -> Result<HttpResponse, Error> {
    let room_id = *id;
    let mut room = house.into_inner().get(room_id)?;
    room.load().await?;

    Ok(HttpResponse::Ok().json(room))
}

///
/// Удалить информацию о комнате умного дома.
///
pub async fn delete(
    house: web::Data<SmartHouse>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let room_id = *id;
    let room = house.into_inner().get(room_id)?;
    room.delete().await?;

    Ok(HttpResponse::Ok().json(Info::new(format!("the room {room_id} was deleted"))))
}

///
/// Обновить информацию о комнате умного дома.
///
pub async fn update(
    house: web::Data<SmartHouse>,
    id: web::Path<Uuid>,
    patch: web::Json<SmartRoomPatch>,
) -> Result<HttpResponse, Error> {
    let room_id = *id;
    let mut room = house.into_inner().get(room_id)?;

    if let Some(ref name) = patch.name {
        room.set_name(name).await?;
    }

    room.load().await?;
    Ok(HttpResponse::Ok().json(room))
}
