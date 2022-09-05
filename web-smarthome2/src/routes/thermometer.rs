use actix_web::{web, HttpResponse, ResponseError};
use uuid::Uuid;

use crate::db::model::{
    house::SmartHouse,
    thermometer::{NewThermometer, ThermometersInfo},
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
    room_id: web::Path<Uuid>,
    thermometer_id: web::Path<Uuid>,
) -> HttpResponse {
    match house.into_inner().get_room(*room_id).await {
        Ok(room) => match room.get_thermometer(*thermometer_id).await {
            Ok(thermometer) => HttpResponse::Ok().json(thermometer),
            Err(error) => error.error_response(),
        },
        Err(error) => error.error_response(),
    }
}
