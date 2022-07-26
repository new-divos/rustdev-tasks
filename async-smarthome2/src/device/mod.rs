use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::DeviceError;

pub mod socket;
pub mod thermometer;

///
/// Типаж, описывающий событие.
///
pub trait Event {
    ///
    /// Получить идентификатор класса события.
    ///
    fn id(&self) -> Uuid;
}

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

    ///
    /// Обработать событие устройством.
    ///
    fn notify(&mut self, e: &dyn Event) -> Result<DeviceState, DeviceError>;
}

///
/// Структура, содержащая состояние устройства после обработки
/// события.
///
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DeviceState {
    // Идентификатор устройства.
    device_id: Uuid,
    // Идентификатор события.
    event_id: Uuid,

    // Измеряемая температура.
    themperature: Option<f64>,
    // Устройства находится во включенном состоянии.
    enabled: Option<bool>,
    // Потребляемая мощность.
    power: Option<f64>,
}

impl DeviceState {
    ///
    /// Получить состояние устройства для розетки.
    ///
    #[inline]
    pub fn for_socket(device_id: Uuid, event_id: Uuid, enabled: bool, power: Option<f64>) -> Self {
        Self {
            device_id,
            event_id,
            themperature: None,
            enabled: Some(enabled),
            power,
        }
    }

    ///
    /// Получить состояние устройства для термометра.
    ///
    #[inline]
    pub fn for_thermometer(device_id: Uuid, event_id: Uuid, themperature: f64) -> Self {
        Self {
            device_id,
            event_id,
            themperature: Some(themperature),
            enabled: None,
            power: None,
        }
    }

    ///
    /// Получить идентификатор устройства.
    ///
    #[inline]
    pub fn device_id(&self) -> Uuid {
        self.device_id
    }

    ///
    /// Получить идентификатор класса события.
    ///
    #[inline]
    pub fn event_id(&self) -> Uuid {
        self.event_id
    }

    ///
    /// Получить измеряемую температуру устройства.
    ///
    #[inline]
    pub fn themperature(&self) -> Option<f64> {
        self.themperature
    }

    ///
    /// Определить, включено ли устройство.
    ///
    #[inline]
    pub fn enabled(&self) -> Option<bool> {
        self.enabled
    }

    ///
    /// Получить потребляемую мощность.
    ///
    #[inline]
    pub fn power(&self) -> Option<f64> {
        self.power
    }
}

///
/// Событие, для получение текущего состояния устройства.
///
pub struct StateEvent {}

impl Event for StateEvent {
    ///
    /// Получить идентификатор класса события.
    ///
    fn id(&self) -> Uuid {
        Self::ID
    }
}

impl Default for StateEvent {
    ///
    /// Экземпляр события по умолчанию.
    ///
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl StateEvent {
    // Идентификатор класса события.
    pub(crate) const ID: Uuid = uuid::uuid!("c346ee2a-4cd1-4e46-8ca7-5b329721187e");

    ///
    /// Создать событие, для получения текущего состояния устройства.
    ///
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}
