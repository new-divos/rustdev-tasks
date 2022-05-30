use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    IllegalRoomName,
    IllegalDeviceName,
    IllegalRoomId,
    IllegalDeviceId,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    ///
    /// Форматировать объект ошибки.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::IllegalRoomName => {
                write!(f, "Illegal room name")
            }
            Error::IllegalDeviceName => {
                write!(f, "Illegal device name")
            }
            Error::IllegalRoomId => {
                write!(f, "Illegal room identifier")
            }
            Error::IllegalDeviceId => {
                write!(f, "Illegal device identifier")
            }
        }
    }
}
