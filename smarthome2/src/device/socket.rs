use std::fmt;

use uuid::Uuid;

use crate::{
    device::{Device, DeviceState, Event, StateEvent},
    error::DeviceError,
};

///
/// Структура, описывающая взаимодействие с "умной" розеткой.
///
pub struct SmartSocket {
    ///
    /// Идентификатор "умной" розетки.
    ///
    id: Uuid,

    ///
    /// Имя "умной" розетки.
    ///
    name: String,

    ///
    /// Текущее состояние розетки.
    ///
    enabled: bool,

    ///
    /// Потребляемая мощность.
    ///
    power: f64,
}

impl fmt::Display for SmartSocket {
    ///
    /// Получить информацию об "умной" розетке с помощью форматирования.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = vec![format!(
            "умная розетка \"{}\" ({}). Состояние: ",
            self.name, self.id
        )];

        if self.enabled {
            v.push(format!(
                "включена, потребляемая мощность {} Вт.",
                self.power
            ));
        } else {
            v.push("выключена.".to_string());
        }

        write!(f, "{}", v.join(""))
    }
}

impl Device for SmartSocket {
    ///
    /// Получить идентификатор "умной" розетки.
    ///
    fn id(&self) -> Uuid {
        self.id
    }

    ///
    /// Получить имя "умной" розетки.
    ///
    fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Обработать событие устройством.
    ///
    fn notify(&mut self, e: &dyn Event) -> Result<DeviceState, DeviceError> {
        match e.id() {
            StateEvent::ID => Ok(self.get_device_state(e.id())),

            SwitchOnEvent::ID => {
                self.switch_on();
                Ok(self.get_device_state(e.id()))
            }

            SwitchOffEvent::ID => {
                self.switch_off();
                Ok(self.get_device_state(e.id()))
            }

            _ => Err(DeviceError::NotImplementedEvent(e.id())),
        }
    }
}

impl SmartSocket {
    ///
    /// Создать "умную" розетку в выключенном состоянии.
    ///
    pub fn new(name: &str) -> Self {
        SmartSocket {
            id: Uuid::new_v4(),
            name: name.to_string(),
            enabled: false,
            power: 0.0,
        }
    }

    ///
    /// Включить "умную" розетку.
    ///
    pub fn switch_on(&mut self) {
        self.enabled = true;
    }

    ///
    /// Выключить "умную" розетку.
    ///
    pub fn switch_off(&mut self) {
        self.enabled = false;
    }

    ///
    /// Проверить, включена ли "умная" розетка.
    ///
    pub fn is_switched_on(&self) -> bool {
        self.enabled
    }

    ///
    /// Получить текущее значение потребляемой мощности.
    ///
    pub fn power(&self) -> Option<f64> {
        if self.enabled {
            Some(self.power)
        } else {
            None
        }
    }

    ///
    /// Подключить нагрузку с заданной мощностью.
    ///
    pub fn plug(&mut self, power: f64) {
        self.power = power;
    }

    // Получить состояние устройства для события с заданным идентификатором класса.
    #[inline]
    fn get_device_state(&self, event_id: Uuid) -> DeviceState {
        DeviceState::for_socket(self.id(), event_id, self.enabled, self.power)
    }
}

///
/// Событие, для включения "умной" розетки.
///
pub struct SwitchOnEvent {}

impl Event for SwitchOnEvent {
    ///
    /// Получить идентификатор класса события.
    ///
    fn id(&self) -> Uuid {
        Self::ID
    }
}

impl Default for SwitchOnEvent {
    ///
    /// Экземпляр события по умолчанию.
    ///
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl SwitchOnEvent {
    // Идентификатор класса события.
    pub(crate) const ID: Uuid = uuid::uuid!("56848c21-6600-48d9-a50a-9a0f83486408");

    ///
    /// Создать событие, для для включения "умной" розетки.
    ///
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

///
/// Событие, для выключения "умной" розетки.
///
pub struct SwitchOffEvent {}

impl Event for SwitchOffEvent {
    ///
    /// Получить идентификатор класса события.
    ///
    fn id(&self) -> Uuid {
        Self::ID
    }
}

impl Default for SwitchOffEvent {
    ///
    /// Экземпляр события по умолчанию.
    ///
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl SwitchOffEvent {
    // Идентификатор класса события.
    pub(crate) const ID: Uuid = uuid::uuid!("4ca18a36-38e0-410a-9c71-ccf4f109ebd4");

    ///
    /// Создать событие, для для включения "умной" розетки.
    ///
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smart_socket_test() {
        let mut socket1 = SmartSocket::new("Socket1");
        assert_eq!(socket1.name.as_str(), "Socket1");
        assert!(!socket1.enabled);
        assert_eq!(socket1.power, 0.0);

        socket1.switch_on();
        assert!(socket1.enabled);

        socket1.plug(1000.0);
        assert_eq!(socket1.power, 1000.0);

        socket1.switch_off();
        assert!(!socket1.enabled);
    }
}
