use std::{fmt, pin::Pin};

use async_trait::async_trait;
use futures::executor::block_on;
use tokio::net::ToSocketAddrs;
use uuid::Uuid;

use crate::{
    control::{client::ControlClient, message::ControlRequest},
    device::{AsyncDevice, Device, DeviceState, Event, StateEvent},
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

#[async_trait]
impl AsyncDevice for SmartSocket {
    ///
    /// Получить идентификатор "умной" розетки.
    ///
    async fn id(&self) -> Uuid {
        self.id
    }

    ///
    /// Получить имя "умной" розетки.
    ///
    async fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Обработать событие устройством.
    ///
    async fn async_notify(
        &mut self,
        e: Pin<Box<dyn Event + Send + Sync>>,
    ) -> Result<DeviceState, DeviceError> {
        match e.id() {
            StateEvent::ID => Ok(DeviceState::for_socket(
                self.id,
                e.id(),
                self.enabled,
                self.power(),
            )),

            SwitchOnEvent::ID => {
                self.switch_on();
                Ok(DeviceState::for_socket(
                    self.id,
                    e.id(),
                    self.enabled,
                    self.power(),
                ))
            }

            SwitchOffEvent::ID => {
                self.switch_off();
                Ok(DeviceState::for_socket(
                    self.id,
                    e.id(),
                    self.enabled,
                    self.power(),
                ))
            }

            id => Err(DeviceError::NotImplementedEvent(id)),
        }
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
            StateEvent::ID => Ok(DeviceState::for_socket(
                self.id,
                e.id(),
                self.enabled,
                self.power(),
            )),

            SwitchOnEvent::ID => {
                self.switch_on();
                Ok(DeviceState::for_socket(
                    self.id,
                    e.id(),
                    self.enabled,
                    self.power(),
                ))
            }

            SwitchOffEvent::ID => {
                self.switch_off();
                Ok(DeviceState::for_socket(
                    self.id,
                    e.id(),
                    self.enabled,
                    self.power(),
                ))
            }

            id => Err(DeviceError::NotImplementedEvent(id)),
        }
    }
}

impl SmartSocket {
    ///
    /// Создать "умную" розетку в выключенном состоянии.
    ///
    #[inline]
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
    #[inline]
    pub fn switch_on(&mut self) {
        self.enabled = true;
    }

    ///
    /// Выключить "умную" розетку.
    ///
    #[inline]
    pub fn switch_off(&mut self) {
        self.enabled = false;
    }

    ///
    /// Проверить, включена ли "умная" розетка.
    ///
    #[inline]
    pub fn enabled(&self) -> bool {
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
}

///
/// Структура, описывающая взаимодействие с удаленной "умной" розеткой
/// по протоколу TCP.
///
pub struct RemoteSmartSocket {
    ///
    /// Идентификатор "умной" розетки.
    ///
    id: Uuid,

    ///
    /// Имя "умной" розетки.
    ///
    name: String,

    ///
    /// Клиент для взаимодействия с удаленной умной розеткой.
    ///
    client: ControlClient,
}

impl fmt::Display for RemoteSmartSocket {
    ///
    /// Получить информацию об "умной" розетке с помощью форматирования.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(response) = block_on(
            self.client
                .request(ControlRequest::acquire_remote_device_state()),
        ) {
            if let Some(state) = response.state() {
                if state.device_id() == self.id {
                    if let Some(enabled) = state.enabled() {
                        let power = state.power().unwrap_or(0.0);

                        let mut v = vec![format!(
                            "умная розетка \"{}\" ({}). Состояние: ",
                            self.name, self.id
                        )];

                        if enabled {
                            v.push(format!("включена, потребляемая мощность {} Вт.", power));
                        } else {
                            v.push("выключена.".to_string());
                        }

                        return write!(f, "{}", v.join(""));
                    }
                }
            }
        }

        Err(fmt::Error)
    }
}

#[async_trait]
impl AsyncDevice for RemoteSmartSocket {
    ///
    /// Идентификатор удаленной "умной" розетки.
    ///
    async fn id(&self) -> Uuid {
        self.id
    }

    ///
    /// Получить имя удаленной "умной" розетки.
    ///
    async fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Обработать событие устройством.
    ///
    async fn async_notify(
        &mut self,
        e: Pin<Box<dyn Event + Send + Sync>>,
    ) -> Result<DeviceState, DeviceError> {
        let response = match e.id() {
            SwitchOnEvent::ID => {
                self.client
                    .request(ControlRequest::switch_on_remote_device())
                    .await?
            }

            SwitchOffEvent::ID => {
                self.client
                    .request(ControlRequest::switch_off_remote_device())
                    .await?
            }

            StateEvent::ID => {
                self.client
                    .request(ControlRequest::acquire_remote_device_state())
                    .await?
            }

            id => {
                return Err(DeviceError::NotImplementedEvent(id));
            }
        };

        if let Some(state) = response.state() {
            if state.device_id() == self.id {
                return Ok(state);
            }
        }

        Err(DeviceError::UnexpectedMessage)
    }
}

impl Device for RemoteSmartSocket {
    ///
    /// Идентификатор удаленной "умной" розетки.
    ///
    fn id(&self) -> Uuid {
        self.id
    }

    ///
    /// Получить имя удаленной "умной" розетки.
    ///
    fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Обработать событие устройством.
    ///
    fn notify(&mut self, e: &dyn Event) -> Result<DeviceState, DeviceError> {
        let response = match e.id() {
            StateEvent::ID => block_on(
                self.client
                    .request(ControlRequest::switch_on_remote_device()),
            )?,

            SwitchOnEvent::ID => block_on(
                self.client
                    .request(ControlRequest::switch_off_remote_device()),
            )?,

            SwitchOffEvent::ID => block_on(
                self.client
                    .request(ControlRequest::acquire_remote_device_state()),
            )?,

            id => {
                return Err(DeviceError::NotImplementedEvent(id));
            }
        };

        if let Some(state) = response.state() {
            if state.device_id() == self.id {
                return Ok(state);
            }
        }

        Err(DeviceError::UnexpectedMessage)
    }
}

impl RemoteSmartSocket {
    ///
    /// Подключиться к серверу с заданным адресом.
    ///
    pub async fn connect<A>(addrs: A) -> Result<Self, DeviceError>
    where
        A: ToSocketAddrs,
    {
        let client = ControlClient::connect(addrs).await?;

        let response = client
            .request(ControlRequest::acquire_remote_device_name())
            .await?;
        if let Some((id, name)) = response.name() {
            Ok(Self {
                id,
                name: name.to_owned(),
                client,
            })
        } else {
            Err(DeviceError::UnexpectedMessage)
        }
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
