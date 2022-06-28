use std::{error, fmt};

use uuid::Uuid;

#[derive(Debug)]
pub enum Error {
    IllegalRoomName(String),
    IllegalDeviceName(String),
    IllegalRoomId(Uuid),
    IllegalDeviceId(Uuid),
    NotImplementedEvent(Uuid),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    ///
    /// Форматировать объект ошибки.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::IllegalRoomName(ref name) => {
                write!(f, "Illegal room name \"{}\"", name)
            }
            Error::IllegalDeviceName(ref name) => {
                write!(f, "Illegal device name \"{}\"", name)
            }
            Error::IllegalRoomId(id) => {
                write!(f, "Illegal room identifier {}", id)
            }
            Error::IllegalDeviceId(id) => {
                write!(f, "Illegal device identifier {}", id)
            }
            Error::NotImplementedEvent(id) => {
                write!(f, "The event {} is not implemented", id)
            }
        }
    }
}
