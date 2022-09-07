use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{db::model::house::SmartHouse, error::ErrorInfo};

///
/// Структура с описанием статуса успешно выполненной операции.
///
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Info {
    ///
    /// Сообщение о выполненной операции.
    ///
    message: String,
}

impl Info {
    ///
    /// Создать сообщение со статусом успешно выполненной операции.
    ///
    #[inline]
    pub(crate) fn new<S: AsRef<str>>(message: S) -> Self {
        Self {
            message: message.as_ref().to_string(),
        }
    }

    ///
    /// Получить сообщение со статусом успешно выполненного запроса.
    ///
    #[inline]
    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

///
/// Роут по умолчанию.
///
pub async fn not_found(_: web::Data<SmartHouse>, _: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().json(ErrorInfo::new("route not found"))
}
