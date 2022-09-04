use actix_web::{web, HttpResponse, ResponseError};
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::{
    db::model::house::{HouseInfo, NewRoom, RoomInfo, SmartHouse},
    routes::RequestSuccess,
};

///
/// Роут для добавления комнаты.
///
pub async fn new_room(house: web::Data<SmartHouse>, new_room: web::Json<NewRoom>) -> HttpResponse {
    match house.into_inner().create_room(new_room.name()).await {
        Ok(room) => match RoomInfo::with_room(room).await {
            Ok(room_info) => HttpResponse::Ok().json(room_info),
            Err(error) => error.error_response(),
        },
        Err(error) => error.error_response(),
    }
}

///
/// Роут для получения всех комнат.
///
pub async fn all_rooms(house: web::Data<SmartHouse>) -> HttpResponse {
    let house_id = house.house_id();
    let house_name = house.house_name().to_string();

    match house.into_inner().all_rooms().await {
        Ok(rooms) => {
            let rooms_info = stream::iter(rooms.into_iter())
                .then(RoomInfo::with_room)
                .filter_map(|rr| async move { rr.ok() })
                .collect::<Vec<_>>()
                .await;
            HttpResponse::Ok().json(HouseInfo::new(house_id, house_name, rooms_info.into_iter()))
        }
        Err(error) => error.error_response(),
    }
}

///
/// Роут для получения информации о комнате.
///
pub async fn get_room(house: web::Data<SmartHouse>, room_id: web::Path<Uuid>) -> HttpResponse {
    match house.into_inner().get_room(*room_id).await {
        Ok(room) => match RoomInfo::with_room(room).await {
            Ok(room_info) => HttpResponse::Ok().json(room_info),
            Err(error) => error.error_response(),
        },
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
        Ok(room) => match RoomInfo::with_room(room).await {
            Ok(room_info) => HttpResponse::Ok().json(room_info),
            Err(error) => error.error_response(),
        },
        Err(error) => error.error_response(),
    }
}
