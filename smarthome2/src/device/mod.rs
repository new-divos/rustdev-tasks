use std::fmt;

use uuid::Uuid;

use crate::error::Error;

pub mod socket;
pub mod thermometer;

///
/// Типаж, описывающий устройство.
///
pub trait Device: fmt::Display {
    ///
    /// Получить идентификатор устройства.
    ///
    fn id(&self) -> Uuid;

    ///
    /// Получить имя устройства.
    ///
    fn name(&self) -> &str;
}

///
/// Типаж, описывающий информацию об устройстве.
///
pub trait DeviceInfo<U, V> {
    ///
    /// Получить текстовую информацию об устройстве.
    ///
    fn info(&self, idx1: U, idx2: V) -> Result<String, Error>;
}
