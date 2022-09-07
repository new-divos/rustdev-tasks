use actix_web::{web, HttpResponse};
use futures::stream::{self, StreamExt};
use uuid::Uuid;

use crate::{
    db::model::house::{HouseInfo, NewRoom, RoomData, RoomInfo, SmartHouse},
    error::Error,
    routes::Success,
};

///
/// Роут для добавления комнаты.
///
pub async fn new_room(
    house: web::Data<SmartHouse>,
    new_room: web::Json<NewRoom>,
) -> Result<HttpResponse, Error> {
    let room = house.into_inner().create_room(new_room.name()).await?;
    Ok(HttpResponse::Ok().json(RoomInfo::with_room(room).await?))
}

///
/// Роут для получения всех комнат.
///
pub async fn all_rooms(house: web::Data<SmartHouse>) -> Result<HttpResponse, Error> {
    let house = house.into_inner();

    let rooms = stream::iter(house.all_rooms().await?.into_iter())
        .then(RoomInfo::with_room)
        .filter_map(|ri| async move { ri.ok() })
        .collect::<Vec<_>>()
        .await;

    Ok(HttpResponse::Ok().json(HouseInfo::new(
        house.house_id(),
        house.house_name(),
        rooms.into_iter(),
    )))
}

///
/// Роут для удаления информации о всех комнатах.
///
pub async fn delete_rooms(house: web::Data<SmartHouse>) -> Result<HttpResponse, Error> {
    house.into_inner().delete_rooms().await?;
    Ok(HttpResponse::Ok().json(Success::new("all house rooms were deleted")))
}

///
/// Роут для получения информации о комнате.
///
pub async fn get_room(
    house: web::Data<SmartHouse>,
    room_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let room = house.into_inner().get_room(*room_id).await?;
    Ok(HttpResponse::Ok().json(RoomInfo::with_room(room).await?))
}

///
/// Роут для удаления информации о комнате.
///
pub async fn delete_room(
    house: web::Data<SmartHouse>,
    room_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    house.into_inner().delete_room(*room_id).await?;
    Ok(HttpResponse::Ok().json(Success::new(format!("the room {} was deleted", *room_id))))
}

///
/// Роут для изменения наименования комнаты.
///
pub async fn update_room(
    house: web::Data<SmartHouse>,
    room_id: web::Path<Uuid>,
    data: web::Json<RoomData>,
) -> Result<HttpResponse, Error> {
    match data.name {
        Some(ref name) => {
            let room = house.into_inner().update_room_name(*room_id, name).await?;
            Ok(HttpResponse::Ok().json(RoomInfo::with_room(room).await?))
        }
        None => get_room(house, room_id).await,
    }
}
