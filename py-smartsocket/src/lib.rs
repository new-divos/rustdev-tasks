use pyo3::prelude::*;

use smarthome2::device::{
    socket::{RemoteSmartSocket, SwitchOffEvent, SwitchOnEvent},
    Device, DeviceState, StateEvent,
};

pub(crate) mod error;

///
/// Класс языка программирования Python клиента умной розетки.
///
#[pyclass]
struct SmartSocketClient {
    // Объект для управления удаленной розеткой.
    socket: Option<RemoteSmartSocket>,

    // Состояние удаленной розетки.
    socket_state: Option<DeviceState>,

    // Адрес подключения удаленной розетки.
    socket_addrs: Option<String>,
}

#[pymethods]
impl SmartSocketClient {
    ///
    /// Конструктор клиента умной розетки.
    ///
    #[new]
    fn new() -> Self {
        Self {
            socket: None,
            socket_state: None,
            socket_addrs: None,
        }
    }

    ///
    /// Подключиться к серверу умной розетки.
    ///
    fn connect(&mut self, addrs: &str) -> PyResult<()> {
        match RemoteSmartSocket::connect(addrs) {
            Ok(mut socket) => match socket.notify(&StateEvent::new()) {
                Ok(device_state) => {
                    self.socket = Some(socket);
                    self.socket_state = Some(device_state);
                    self.socket_addrs = Some(addrs.to_string());

                    Ok(())
                }
                Err(e) => Err(PyErr::from(error::Error::DeviceError(e))),
            },
            Err(e) => Err(PyErr::from(error::Error::DeviceError(e))),
        }
    }

    ///
    /// Отключиться от сервера умной розетки.
    ///
    fn disconnect(&mut self) {
        self.socket = None;
        self.socket_state = None;
        self.socket_addrs = None;
    }

    ///
    /// Получить отладочную информацию об умной разетке.
    ///
    fn __repr__(&self) -> String {
        let mut properties: Vec<String> = Vec::new();

        if self.socket.is_some() {
            if let Some(ref addrs) = self.socket_addrs {
                properties.push(format!("addrs=\"{}\"", addrs));
            }

            if let Some(ref socket_state) = self.socket_state {
                if let Some(enabled) = socket_state.enabled() {
                    if enabled {
                        properties.push("switched_on".to_string());

                        if let Some(power) = socket_state.power() {
                            properties.push(format!("power={}", power));
                        }
                    } else {
                        properties.push("switched_off".to_string());
                    }
                }
            }
        }

        format!("SmartSocketClient({})", properties.join(", "))
    }

    ///
    /// Получить информацию об умной розетке в строковом виде.
    ///
    fn __str__(&self) -> String {
        let mut info: Vec<String> = Vec::new();

        if self.socket.is_some() {
            if let Some(ref addrs) = self.socket_addrs {
                info.push(format!(
                    "Установлено соединение с умной розеткой по адресу \"{}\"",
                    addrs
                ));

                if let Some(ref socket_state) = self.socket_state {
                    if let Some(enabled) = socket_state.enabled() {
                        if enabled {
                            info.push("розетка включена".to_string());

                            if let Some(power) = socket_state.power() {
                                info.push(format!("потребляемая мощьность {} Вт.", power));
                            }
                        } else {
                            info.push("розетка выключена".to_string());
                        }
                    }
                }
            }
        } else {
            info.push("Соединение с умной розеткой не установлено".to_string());
        }

        format!("{}", info.join(", "))
    }

    ///
    /// Включить умную розетку.
    ///
    fn switch_on(&mut self) -> PyResult<()> {
        if let Some(ref mut socket) = self.socket {
            match socket.notify(&SwitchOnEvent::new()) {
                Ok(device_state) => {
                    self.socket_state = Some(device_state);
                    Ok(())
                }
                Err(e) => Err(PyErr::from(error::Error::DeviceError(e))),
            }
        } else {
            Err(PyErr::from(error::Error::DeviceIsDisconnected))
        }
    }

    ///
    /// Выключить умную розетку.
    ///
    fn switch_off(&mut self) -> PyResult<()> {
        if let Some(ref mut socket) = self.socket {
            match socket.notify(&SwitchOffEvent::new()) {
                Ok(device_state) => {
                    self.socket_state = Some(device_state);
                    Ok(())
                }
                Err(e) => Err(PyErr::from(error::Error::DeviceError(e))),
            }
        } else {
            Err(PyErr::from(error::Error::DeviceIsDisconnected))
        }
    }

    ///
    /// Проверить, установлено ли соединение с умной розеткой.
    ///
    fn __bool__(&self) -> bool {
        self.socket.is_some()
    }

    ///
    /// Проверить, включена ли умная розетка.
    ///
    fn is_switched_on(&self) -> PyResult<Option<bool>> {
        if self.socket.is_some() {
            if let Some(ref socket_state) = self.socket_state {
                return Ok(socket_state.enabled());
            }

            Ok(None)
        } else {
            Err(PyErr::from(error::Error::DeviceIsDisconnected))
        }
    }

    ///
    /// Получить потребляемую мощность умной розетки.
    ///
    #[getter(power)]
    fn power(&self) -> PyResult<Option<f64>> {
        if self.socket.is_some() {
            if let Some(ref socket_state) = self.socket_state {
                if let Some(enabled) = socket_state.enabled() {
                    if enabled {
                        return Ok(socket_state.power());
                    }
                }
            }

            Ok(None)
        } else {
            Err(PyErr::from(error::Error::DeviceIsDisconnected))
        }
    }
}

///
/// Модуль языка программирования Python.
///
#[pymodule]
#[pyo3(name = "pysmartsocket")]
fn string_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<SmartSocketClient>()?;
    Ok(())
}
