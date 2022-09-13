use pyo3::{
    exceptions::{PyIOError, PyRuntimeError},
    prelude::*,
};
use thiserror::Error;

use smarthome2::error;

///
/// Ошибка пакета взаимодействия с умной розеткой.
///
#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("device error {0}")]
    DeviceError(#[from] error::DeviceError),

    #[error("device is disconnected")]
    DeviceIsDisconnected,
}

impl From<Error> for PyErr {
    ///
    /// Преобразовать ошибку пакета в ошибку языка программирования Python.
    ///
    fn from(error: Error) -> Self {
        match &error {
            // Преобразовать ошибку устройства в ошибку языка программирования Python.
            Error::DeviceError(device_error) => match device_error {
                error::DeviceError::ConnectionError(_)
                | error::DeviceError::RequestError(_)
                | error::DeviceError::IoError(_) => PyIOError::new_err(error.to_string()),

                _ => PyRuntimeError::new_err(error.to_string()),
            },

            // Отсутствует соединение с умной розеткой.
            Error::DeviceIsDisconnected => {
                PyRuntimeError::new_err("Соединение с умной розеткой не установлено")
            }
        }
    }
}
