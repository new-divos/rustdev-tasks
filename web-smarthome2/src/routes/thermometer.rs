use actix_web::{web, HttpResponse, ResponseError};
use uuid::Uuid;

use crate::{
    db::model::{
        house::SmartHouse,
        thermometer::{NewThermometer, ThermometerData, ThermometersInfo},
    },
    routes::RequestSuccess,
};

///
/// Роут для добавления термометра.
///
pub async fn new_thermometer(
    house: web::Data<SmartHouse>,
    room_id: web::Path<Uuid>,
    new_thermometer: web::Json<NewThermometer>,
) -> HttpResponse {
    match house.into_inner().get_room(*room_id).await {
        Ok(room) => match room
            .create_thermometer(new_thermometer.name(), new_thermometer.temperature())
            .await
        {
            Ok(thermometer_info) => HttpResponse::Ok().json(thermometer_info),
            Err(error) => error.error_response(),
        },
        Err(error) => error.error_response(),
    }
}

///
/// Роут для получения информации о всех термометрах комнаты умного дома.
///
pub async fn all_thermometers(
    house: web::Data<SmartHouse>,
    room_id: web::Path<Uuid>,
) -> HttpResponse {
    match house.into_inner().get_room(*room_id).await {
        Ok(room) => match room.all_thermometers().await {
            Ok(thermometers) => HttpResponse::Ok().json(ThermometersInfo::new(
                room.room_id(),
                room.room_name(),
                thermometers.into_iter(),
            )),
            Err(error) => error.error_response(),
        },
        Err(error) => error.error_response(),
    }
}

///
/// Роут для получения информации о термометре умного дома.
///
pub async fn get_thermometer(
    house: web::Data<SmartHouse>,
    ids: web::Path<(Uuid, Uuid)>,
) -> HttpResponse {
    let (room_id, thermometer_id) = *ids;
    match house.into_inner().get_room(room_id).await {
        Ok(room) => match room.get_thermometer(thermometer_id).await {
            Ok(thermometer) => HttpResponse::Ok().json(thermometer),
            Err(error) => error.error_response(),
        },
        Err(error) => error.error_response(),
    }
}

///
/// Роут для удаления термометра умного дома.
///
pub async fn delete_thermometer(
    house: web::Data<SmartHouse>,
    ids: web::Path<(Uuid, Uuid)>,
) -> HttpResponse {
    let (room_id, thermometer_id) = *ids;
    match house.into_inner().get_room(room_id).await {
        Ok(room) => match room.delete_thermometer(thermometer_id).await {
            Ok(_) => HttpResponse::Ok().json(RequestSuccess::new(format!(
                "the thermometer {} of the room {} was deleted",
                thermometer_id, room_id
            ))),
            Err(error) => error.error_response(),
        },
        Err(error) => error.error_response(),
    }
}

///
/// Роут для обновления значений термометра умного дома.
///
pub async fn update_thermometer(
    house: web::Data<SmartHouse>,
    ids: web::Path<(Uuid, Uuid)>,
    data: web::Json<ThermometerData>,
) -> HttpResponse {
    let (room_id, thermometer_id) = *ids;
    match house.into_inner().get_room(room_id).await {
        Ok(room) => match room
            .update_thermometer(thermometer_id, data.name.as_deref(), data.temperature)
            .await
        {
            Ok(thermometer) => HttpResponse::Ok().json(thermometer),
            Err(error) => error.error_response(),
        },
        Err(error) => error.error_response(),
    }
}
