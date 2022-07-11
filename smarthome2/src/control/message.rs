use std::{error::Error, fmt, iter};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    control::protocol::{
        consts::{
            CONTROL_REQUEST_ID, CONTROL_RESPONSE_ID, TEXT_MESSAGE_ID, THERMOMETER_MESSAGE_ID,
        },
        Message, ProtocolVersion,
    },
    device::DeviceState,
};

///
/// Сообщение для обмена тестовыми данными.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    text: String,
}

impl fmt::Display for TextMessage {
    ///
    /// Выполнить форматирование сообщения.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Message for TextMessage {
    ///
    /// Идентификатор типа сообщения.
    ///
    const TYPE: u16 = TEXT_MESSAGE_ID;
}

impl TextMessage {
    ///
    /// Создать сообщение с заданным текстом.
    ///
    pub fn new<D: AsRef<str>>(text: D) -> Self {
        Self {
            text: text.as_ref().to_owned(),
        }
    }
}

///
/// Данные запроса управления "умным" домом.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ControlRequestData {
    // Запрос на получение списка комнат.
    AcquireRooms,

    // Запрос на получение списка устройств для комнаты.
    AcquireDevices(Uuid),

    // Запрос на получение состояния устройства.
    AcquireDeviceState(Uuid, Uuid),

    // Запрос на получение состояния удаленного устройства.
    AcquireRemoteDeviceState,

    // Запрос на получение информации об устройства.
    AcquireDeviceInfo(Uuid, Uuid),

    // Запрос на получение идентификатора и имени удаленного устройства.
    AcquireRemoteDeviceName,

    // Запрос на включение устройства.
    SwitchOnDevice(Uuid, Uuid),

    // Запрос на включение удаоенного устройства.
    SwitchOnRemoteDevice,

    // Запрос на выключение устройства.
    SwitchOffDevice(Uuid, Uuid),

    // Запрос на выключение удаленного устройства.
    SwitchOffRemoteDevice,
}

///
/// Запрос на управление "умным" домом.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlRequest {
    // Версия протокола.
    version: ProtocolVersion,

    // Данные запроса.
    pub(crate) data: ControlRequestData,
}

impl Message for ControlRequest {
    ///
    /// Идентификатор типа сообщения.
    ///
    const TYPE: u16 = CONTROL_REQUEST_ID;
}

impl ControlRequest {
    ///
    /// Создать запрос для получения списка комнат.
    ///
    #[inline]
    pub fn acquire_rooms() -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlRequestData::AcquireRooms,
        }
    }

    ///
    /// Создать запрос для получения списка устройств.
    ///
    #[inline]
    pub fn acquire_devices(room_id: Uuid) -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlRequestData::AcquireDevices(room_id),
        }
    }

    ///
    /// Создать запрос для получения состояния устройства.
    ///
    #[inline]
    pub fn acquire_device_state(room_id: Uuid, device_id: Uuid) -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlRequestData::AcquireDeviceState(room_id, device_id),
        }
    }

    ///
    /// Создать запрос для получения состояния удаленного устройства.
    ///
    #[inline]
    pub fn acquire_remote_device_state() -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlRequestData::AcquireRemoteDeviceState,
        }
    }

    ///
    /// Создать запрос для получения информации об устройстве.
    ///
    #[inline]
    pub fn acquire_device_info(room_id: Uuid, device_id: Uuid) -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlRequestData::AcquireDeviceInfo(room_id, device_id),
        }
    }

    ///
    /// Создать запрос на получение идентификатора и имени удаленного устройства.
    ///
    #[inline]
    pub fn acquire_remote_device_name() -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlRequestData::AcquireRemoteDeviceName,
        }
    }

    ///
    /// Создать запрос для включения устройства.
    ///
    #[inline]
    pub fn switch_on_device(room_id: Uuid, device_id: Uuid) -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlRequestData::SwitchOnDevice(room_id, device_id),
        }
    }

    ///
    /// Создать запрос для включения удаленного устройства.
    ///
    #[inline]
    pub fn switch_on_remote_device() -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlRequestData::SwitchOnRemoteDevice,
        }
    }

    ///
    /// Создать запрос для выключения устройства.
    ///
    #[inline]
    pub fn switch_off_device(room_id: Uuid, device_id: Uuid) -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlRequestData::SwitchOffDevice(room_id, device_id),
        }
    }

    ///
    /// Создать запрос для выключения удаленного устройства.
    ///
    #[inline]
    pub fn switch_off_remote_device() -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlRequestData::SwitchOffRemoteDevice,
        }
    }
}

///
/// Данные ответа на запрос управления "умным" домом.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ControlResponseData {
    // Вектор с данными о комнатах или устройствах.
    List(Vec<(Uuid, String)>),

    // Информация о состоянии устройства.
    State(DeviceState),

    // Информация об устройстве в текстовом виде.
    Info(String),

    // Идентификатор и имя устройства.
    Name(Uuid, String),

    // Текстовая информация об ошибке.
    Error(String),
}

///
/// Ответ на запрос управления "умным" домом.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlResponse {
    // Версия протокола.
    version: ProtocolVersion,

    // Данные ответа на запрос.
    pub(crate) data: ControlResponseData,
}

impl<'a> iter::FromIterator<(Uuid, &'a str)> for ControlResponse {
    ///
    /// Сформировать ответ на запрос управления "умным" домом из
    /// итератора.
    ///
    fn from_iter<T: IntoIterator<Item = (Uuid, &'a str)>>(iter: T) -> Self {
        let v: Vec<(Uuid, String)> = iter
            .into_iter()
            .map(|(id, name)| (id, name.to_owned()))
            .collect();

        Self {
            version: ProtocolVersion::V1_0,
            data: ControlResponseData::List(v),
        }
    }
}

impl Message for ControlResponse {
    ///
    /// Идентификатор типа сообщения.
    ///
    const TYPE: u16 = CONTROL_RESPONSE_ID;
}

impl ControlResponse {
    ///
    /// Создать ответ с состоянием устройства.
    ///
    #[inline]
    pub fn with_state(state: DeviceState) -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlResponseData::State(state),
        }
    }

    ///
    /// Создать ответ с информацией об устройстве.
    ///
    #[inline]
    pub fn with_info<D: AsRef<str>>(info: D) -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlResponseData::Info(info.as_ref().to_owned()),
        }
    }

    ///
    /// Создать ответ с идентификатором и именем устройства.
    ///
    #[inline]
    pub fn with_name<D: AsRef<str>>(id: Uuid, name: D) -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlResponseData::Name(id, name.as_ref().to_owned()),
        }
    }

    ///
    /// Создать ответ с информацией об ошибке.
    ///
    #[inline]
    pub fn with_error<E: Error>(error: E) -> Self {
        Self {
            version: ProtocolVersion::V1_0,
            data: ControlResponseData::Error(format!("Error: {}", error)),
        }
    }

    ///
    /// Получить состояние устройства.
    ///
    pub fn state(&self) -> Option<DeviceState> {
        if let ControlResponseData::State(state) = self.data {
            Some(state)
        } else {
            None
        }
    }

    ///
    /// Получить информацию об устройстве.
    ///
    pub fn info(&self) -> Option<&str> {
        if let ControlResponseData::Info(ref info) = self.data {
            Some(info.as_str())
        } else {
            None
        }
    }

    ///
    /// Получить идентификатор и имя устройства.
    ///
    pub fn name(&self) -> Option<(Uuid, &str)> {
        if let ControlResponseData::Name(id, ref name) = self.data {
            Some((id, name.as_str()))
        } else {
            None
        }
    }
}

///
/// Сообщение с данными автономного термометра.
///
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ThermometerMessage {
    ///
    /// Значение температуры автономного термометра.
    ///
    temperature: f64,

    ///
    /// Идентификатор автономного термометра.
    ///
    id: Uuid,
}

impl Message for ThermometerMessage {
    ///
    /// Идентификатор типа сообщения.
    ///
    const TYPE: u16 = THERMOMETER_MESSAGE_ID;
}

impl ThermometerMessage {
    ///
    /// Создать сообщение с заданными идентификатором автономного
    /// термометра и значением температуры.
    ///
    #[inline]
    pub fn new(id: Uuid, temperature: f64) -> Self {
        Self { temperature, id }
    }

    ///
    /// Получить значение температуры.
    ///
    #[inline]
    pub fn temperature(&self) -> f64 {
        self.temperature
    }

    ///
    /// Получить идентификатор автономного термометра.
    ///
    #[inline]
    pub fn id(&self) -> Uuid {
        self.id
    }
}
