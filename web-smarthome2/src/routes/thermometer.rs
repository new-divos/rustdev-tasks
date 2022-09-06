use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::{
    db::model::{
        house::SmartHouse,
        thermometer::{NewThermometer, ThermometerData, ThermometersInfo},
    },
    error::Error,
    routes::RequestSuccess,
};

///
/// Роут для добавления термометра.
///
pub async fn new_thermometer(
    house: web::Data<SmartHouse>,
    room_id: web::Path<Uuid>,
    new_thermometer: web::Json<NewThermometer>,
) -> Result<HttpResponse, Error> {
    let room = house.into_inner().get_room(*room_id).await?;
    Ok(HttpResponse::Ok().json(
        room.create_thermometer(new_thermometer.name(), new_thermometer.temperature())
            .await?,
    ))
}

///
/// Роут для получения информации о всех термометрах комнаты умного дома.
///
pub async fn all_thermometers(
    house: web::Data<SmartHouse>,
    room_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let room = house.into_inner().get_room(*room_id).await?;
    let thermometers = room.all_thermometers().await?;
    Ok(HttpResponse::Ok().json(ThermometersInfo::new(
        room.room_id(),
        room.room_name(),
        thermometers.into_iter(),
    )))
}

///
/// Роут для получения информации о термометре умного дома.
///
pub async fn get_thermometer(
    house: web::Data<SmartHouse>,
    ids: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, Error> {
    let (room_id, thermometer_id) = *ids;
    let room = house.into_inner().get_room(room_id).await?;
    Ok(HttpResponse::Ok().json(room.get_thermometer(thermometer_id).await?))
}

///
/// Роут для удаления термометра умного дома.
///
pub async fn delete_thermometer(
    house: web::Data<SmartHouse>,
    ids: web::Path<(Uuid, Uuid)>,
) -> Result<HttpResponse, Error> {
    let (room_id, thermometer_id) = *ids;
    let room = house.into_inner().get_room(room_id).await?;
    room.delete_thermometer(thermometer_id).await?;
    Ok(HttpResponse::Ok().json(RequestSuccess::new(format!(
        "the thermometer {} of the room {} was deleted",
        thermometer_id, room_id
    ))))
}

///
/// Роут для обновления значений термометра умного дома.
///
pub async fn update_thermometer(
    house: web::Data<SmartHouse>,
    ids: web::Path<(Uuid, Uuid)>,
    data: web::Json<ThermometerData>,
) -> Result<HttpResponse, Error> {
    let (room_id, thermometer_id) = *ids;
    let room = house.into_inner().get_room(room_id).await?;
    match (data.name.as_deref(), data.temperature) {
        (None, None) => Ok(HttpResponse::Ok().json(room.get_thermometer(thermometer_id).await?)),
        (None, Some(temperature)) => Ok(HttpResponse::Ok().json(
            room.update_thermometer_temperature(thermometer_id, temperature)
                .await?,
        )),
        (Some(name), None) => {
            Ok(HttpResponse::Ok().json(room.update_thermometer_name(thermometer_id, name).await?))
        }
        (Some(name), Some(temperature)) => Ok(HttpResponse::Ok().json(
            room.update_thermometer(thermometer_id, name, temperature)
                .await?,
        )),
    }
}
