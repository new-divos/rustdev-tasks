use thiserror::Error;
use uuid::Uuid;

///
/// Ошибка при работе с программой.
///
#[derive(Error, Debug)]
pub enum Error {
    #[error("application initialization error")]
    AppInitError,

    #[error("illegal room name \"{0}\"")]
    IllegalRoomName(String),

    #[error("illegal device name \"{0}\"")]
    IllegalDeviceName(String),

    #[error("illegal room identifier {0}")]
    IllegalRoomId(Uuid),

    #[error("illegal device identifier {0}")]
    IllegalDeviceId(Uuid),

    #[error("the event {0} is not implemented")]
    NotImplementedEvent(Uuid),

    #[error("unexpected message")]
    UnexpectedMessage,

    #[error("io error {0}")]
    IOError(#[from] std::io::Error),

    #[error("UUID error {0}")]
    UuidError(#[from] uuid::Error),

    #[error("configuration deserialization error {0}")]
    ConfigParseError(#[from] toml::de::Error),

    #[error("configuration serialization error {0}")]
    ConfigSerializeError(#[from] toml::ser::Error),
}
