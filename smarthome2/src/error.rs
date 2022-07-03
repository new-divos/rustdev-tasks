use std::io;

use bincode;
use thiserror::Error;
use uuid::Uuid;

///
/// Ошибка при работе с устройствами.
///
#[derive(Error, Debug)]
pub enum DeviceError {
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
}

///
/// Ошибка отпраки данных.
///
#[derive(Debug, Error)]
pub enum SendError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("binary error: {0}")]
    Bin(#[from] bincode::Error),
}

///
/// Ошибка получения данных.
///
#[derive(Debug, Error)]
pub enum RecvError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("binary error: {0}")]
    Bin(#[from] bincode::Error),

    #[error("bad message type {0}")]
    BadType(u16),
}

///
/// Ошибка соединения. Включает ошибки ввода-вывода и handshake.
///
#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("unexpected handshake response")]
    BadHandshake,

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

///
/// Ошибка привязки сокета.
///
#[derive(Debug, Error)]
pub enum BindError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

///
/// Ошибка обработки запроса.
///
#[derive(Debug, Error)]
pub enum RequestError {
    #[error(transparent)]
    Send(#[from] SendError),

    #[error(transparent)]
    Recv(#[from] RecvError),

    #[error("server side error {0}")]
    Srv(String),
}
