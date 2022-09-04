use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{db::model::house::SmartHouse, error::ErrorInfo};

pub mod room;

///
/// Структура с описанием статуса успешно выполненной операции.
///
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct RequestSuccess {
    status: String,
}

impl RequestSuccess {
    ///
    /// Создать сообщение со статусом успешно выполненной операции.
    ///
    #[inline]
    pub(crate) fn new<S: AsRef<str>>(message: S) -> Self {
        Self {
            status: message.as_ref().to_string(),
        }
    }
}

///
/// Роут по умолчанию.
///
pub async fn not_found(_: web::Data<SmartHouse>, _: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().json(ErrorInfo::new("route not found"))
}
