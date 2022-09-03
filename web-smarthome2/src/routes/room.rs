use std::sync::Arc;

use actix_web::{web, HttpResponse};

use crate::db::model::house::{NewRoom, SmartHouse};

pub async fn new_room(
    house: web::Data<Arc<SmartHouse>>,
    new_room: web::Json<NewRoom>,
) -> HttpResponse {
    match house.into_inner().create_room(new_room.name()).await {
        Ok(room) => HttpResponse::Ok().json(room),
        Err(_) => todo!(),
    }
}
