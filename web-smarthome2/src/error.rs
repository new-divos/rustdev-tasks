use actix_web::{error, http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

///
/// Ошибка при работе с программой.
///
#[derive(Error, Debug)]
pub enum Error {
    #[error("application initialization error")]
    AppInitError,

    #[error("illegal room identifier {0}")]
    IllegalRoomId(Uuid),

    #[error("illegal room name {0}")]
    IllegalRoomName(String),

    #[error("illegal socket identifier {0}")]
    IllegalSocketId(Uuid),

    #[error("illegal socket name {0}")]
    IllegalSocketName(String),

    #[error("illegal thermometer identifier {0}")]
    IllegalThermometerId(Uuid),

    #[error("illegal thermometer name {0}")]
    IllegalThermometerName(String),

    #[error("bad request")]
    BadRequest,

    #[error("io error {0}")]
    IOError(#[from] std::io::Error),

    #[error("UUID error {0}")]
    UuidError(#[from] uuid::Error),

    #[error("configuration deserialization error {0}")]
    ConfigParseError(#[from] toml::de::Error),

    #[error("configuration serialization error {0}")]
    ConfigSerializeError(#[from] toml::ser::Error),

    #[error("SQL error {0}")]
    SQLError(#[from] sqlx::Error),
}

impl error::ResponseError for Error {
    ///
    /// Получить код статуса в зависимости от типа ошибки.
    ///
    fn status_code(&self) -> StatusCode {
        match *self {
            Error::IllegalRoomId(_)
            | Error::IllegalSocketId(_)
            | Error::IllegalThermometerId(_) => StatusCode::NOT_FOUND,

            Error::IllegalRoomName(_)
            | Error::IllegalSocketName(_)
            | Error::IllegalThermometerName(_) => StatusCode::FORBIDDEN,

            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    ///
    /// Получить HTTP-ответ для заданного типа ошибки.
    ///
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorInfo::with_error(self))
    }
}

///
/// Структура с информацией об ошибке при работе с программой.
///
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct ErrorInfo {
    ///
    /// Информация об ошибке.
    ///
    error: String,
}

impl ErrorInfo {
    ///
    /// Создать информациб об ошибке.
    ///
    #[inline]
    pub(crate) fn new<S: AsRef<str>>(message: S) -> Self {
        Self {
            error: message.as_ref().to_string(),
        }
    }

    ///
    /// Получить информацию об ошибке.
    ///
    #[inline]
    pub(crate) fn with_error(error: &Error) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}
