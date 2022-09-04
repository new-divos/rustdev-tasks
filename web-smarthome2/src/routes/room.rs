use actix_web::{web, HttpResponse, ResponseError};
use uuid::Uuid;

use super::RequestSuccess;
use crate::db::model::house::{NewRoom, RoomInfo, SmartHouse};

///
/// Роут для добавления комнаты.
///
pub async fn new_room(house: web::Data<SmartHouse>, new_room: web::Json<NewRoom>) -> HttpResponse {
    match house.into_inner().create_room(new_room.name()).await {
        Ok(room) => HttpResponse::Ok().json(RoomInfo::from(room)),
        Err(error) => error.error_response(),
    }
}

///
/// Роут для получения всех комнат.
///
pub async fn all_rooms(house: web::Data<SmartHouse>) -> HttpResponse {
    match house.into_inner().all_rooms().await {
        Ok(rooms) => {
            let rooms_info: Vec<_> = rooms.into_iter().map(RoomInfo::from).collect();
            HttpResponse::Ok().json(rooms_info)
        }
        Err(error) => error.error_response(),
    }
}

///
/// Роут для получения информации о комнате.
///
pub async fn get_room(house: web::Data<SmartHouse>, room_id: web::Path<Uuid>) -> HttpResponse {
    match house.into_inner().get_room(*room_id).await {
        Ok(room) => HttpResponse::Ok().json(RoomInfo::from(room)),
        Err(error) => error.error_response(),
    }
}

///
/// Роут для удаления информации о комнате.
///
pub async fn delete_room(house: web::Data<SmartHouse>, room_id: web::Path<Uuid>) -> HttpResponse {
    match house.into_inner().delete_room(*room_id).await {
        Ok(_) => HttpResponse::Ok().json(RequestSuccess::new(format!(
            "room {} was deleted",
            *room_id
        ))),
        Err(error) => error.error_response(),
    }
}

///
/// Роут для изменения наименования комнаты.
///
pub async fn update_room(
    house: web::Data<SmartHouse>,
    room_id: web::Path<Uuid>,
    new_room: web::Json<NewRoom>,
) -> HttpResponse {
    match house
        .into_inner()
        .update_room(*room_id, new_room.name())
        .await
    {
        Ok(room) => HttpResponse::Ok().json(RoomInfo::from(room)),
        Err(error) => error.error_response(),
    }
}
