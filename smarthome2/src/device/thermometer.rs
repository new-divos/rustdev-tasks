#![allow(clippy::type_complexity)]

use std::{
    fmt,
    net::{ToSocketAddrs, UdpSocket},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock, Weak,
    },
    thread, time,
};

use bincode::{self, Options};
use log;
use rand::{thread_rng, Rng};
use statrs::distribution::Normal;
use uuid::Uuid;

use crate::{
    control::message::ThermometerMessage,
    device::{Device, DeviceState, Event, StateEvent},
    error::DeviceError,
};

///
/// Структура, описывающая взаимодействие с "умным" термометром.
///
#[derive(Debug)]
pub struct SmartThermometer {
    ///
    /// Идентификатор "умного" термометра.
    ///
    id: Uuid,

    ///
    /// Имя "умного" термометра.
    ///
    name: String,

    ///
    /// Текущее значение температуры.
    ///
    temperature: f64,
}

impl fmt::Display for SmartThermometer {
    ///
    /// Получить информацию об "умном" термометре с помощью форматирования.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "умный термометр \"{}\" ({}). Температура: {} °C.",
            self.name, self.id, self.temperature
        )
    }
}

impl Device for SmartThermometer {
    ///
    /// Получить идентификатор "умного" термометра.
    ///
    fn id(&self) -> Uuid {
        self.id
    }

    ///
    /// Получить имя "умного" термометра.
    ///
    fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Обработать событие устройством.
    ///
    fn notify(&mut self, e: &dyn Event) -> Result<DeviceState, DeviceError> {
        if e.id() == StateEvent::ID {
            Ok(DeviceState::for_thermometer(
                self.id(),
                e.id(),
                self.temperature(),
            ))
        } else {
            Err(DeviceError::NotImplementedEvent(e.id()))
        }
    }
}

impl SmartThermometer {
    ///
    /// Создать термометр с заданным значением температуры.
    ///
    pub fn new(name: &str, temperature: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            temperature,
        }
    }

    ///
    /// Получить текущее значение температуры.
    ///
    pub fn temperature(&self) -> f64 {
        self.temperature
    }
}

///
/// Структура, описывающая взаимодействие с автономным "умным" термометром.
///
#[derive(Debug)]
pub struct AutonomousThermometer {
    ///
    /// Сокет для удаленного получения значений температуры.
    ///
    socket: UdpSocket,

    ///
    /// Экземпляр "умного" термометра.
    ///
    thermometer: Arc<RwLock<SmartThermometer>>,

    ///
    /// Добавлять шум к показаниям температуры.
    ///
    noisy: bool,
}

impl AutonomousThermometer {
    ///
    /// Создать объект по умолчанию для построения экземпляра автономного
    /// "умного" термометра.
    ///
    #[inline]
    pub fn builder() -> AutonomousThermometerBuilder<&'static str, &'static str> {
        AutonomousThermometerBuilder::<&str, &str>::new()
    }

    ///
    /// Запустить отдельный поток для отправки дейтаграмм со значениями темепературы.
    ///
    pub fn run(
        &self,
    ) -> Result<
        (
            thread::JoinHandle<Result<(), DeviceError>>,
            Weak<AtomicBool>,
        ),
        DeviceError,
    > {
        let working = Arc::new(AtomicBool::new(true));
        let control = Arc::downgrade(&working);

        let socket = self.socket.try_clone()?;
        let thermometer = self.thermometer.clone();
        let noisy = self.noisy;

        Ok((
            thread::spawn(move || {
                let duration = time::Duration::from_secs(3);

                let mut rng = thread_rng();
                let normal = Normal::new(0.0, 1.0).unwrap();

                while (*working).load(Ordering::Relaxed) {
                    let (mut temperature, id) = {
                        let guard = thermometer.read().unwrap();
                        (guard.temperature(), guard.id())
                    };
                    if noisy {
                        temperature += rng.sample(normal);
                    }

                    let message = ThermometerMessage::new(id, temperature);
                    let bytes = bincode::options().with_big_endian().serialize(&message)?;

                    log::info!(
                        "Sending temperature {} °C of the device {} ...",
                        temperature,
                        id
                    );
                    socket.send(&bytes[..])?;

                    thread::sleep(duration);
                }

                Ok(())
            }),
            control,
        ))
    }
}

///
/// Структура для построения экзкмпляра автономного "умного" термометра.
///
pub struct AutonomousThermometerBuilder<BA: ToSocketAddrs, RA: ToSocketAddrs> {
    ///
    /// Адес привязки UDP-сокета.
    ///
    addr: BA,

    ///
    /// Адрес подключения удаленного термометра.
    ///
    remote_addr: RA,

    ///
    /// Добавлять шум к показаниям температуры.
    ///
    noisy: bool,
}

impl<BA: ToSocketAddrs, RA: ToSocketAddrs> AutonomousThermometerBuilder<BA, RA> {
    ///
    /// Установить адрес привязки сокета автономного "умного" термометра.
    ///
    #[inline]
    pub fn bind<BA2: ToSocketAddrs>(self, addr: BA2) -> AutonomousThermometerBuilder<BA2, RA> {
        AutonomousThermometerBuilder::<BA2, RA> {
            addr,
            remote_addr: self.remote_addr,
            noisy: self.noisy,
        }
    }

    ///
    /// Установить адрес удаленного "умного" термометра.
    ///
    #[inline]
    pub fn connect<RA2: ToSocketAddrs>(self, addr: RA2) -> AutonomousThermometerBuilder<BA, RA2> {
        AutonomousThermometerBuilder::<BA, RA2> {
            addr: self.addr,
            remote_addr: addr,
            noisy: self.noisy,
        }
    }

    ///
    /// Добавлять нормальный шум к передаваемым данным.
    ///
    #[inline]
    pub fn with_noise(self) -> Self {
        Self {
            addr: self.addr,
            remote_addr: self.remote_addr,
            noisy: true,
        }
    }

    ///
    /// Выполнить построение экзкмпляра автономного "умного" термометра.
    ///
    pub fn build(
        self,
        thermometer: SmartThermometer,
    ) -> Result<AutonomousThermometer, DeviceError> {
        let t = AutonomousThermometer {
            socket: UdpSocket::bind(self.addr)?,
            thermometer: Arc::new(RwLock::new(thermometer)),
            noisy: self.noisy,
        };
        t.socket.connect(self.remote_addr)?;

        Ok(t)
    }
}

impl Default for AutonomousThermometerBuilder<&str, &str> {
    ///
    /// Создать экземпляр по умолчанию построителя автономного
    /// "умного" термометра.
    ///
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl AutonomousThermometerBuilder<&str, &str> {
    ///
    /// Создать экземпляр с настройками по умолчанию построителя
    /// автономного "умного" термометра.
    ///
    #[inline]
    pub fn new() -> Self {
        Self {
            addr: "127.0.0.1:8000",
            remote_addr: "127.0.0.1:8888",
            noisy: false,
        }
    }
}

///
/// Структура, описывающая взаимодействие с удаленным "умным" термометром.
///
#[derive(Debug)]
pub struct RemoteThermometer {
    ///
    /// Имя удаленного "умного" термометра.
    ///
    name: String,

    ///
    /// Данные удаленного "умного" термометра.
    ///
    data: Arc<RwLock<(Uuid, f64)>>,

    ///
    /// Флаг для завершения связанного с удаленным "умным" термометром потока.
    ///
    control: Weak<AtomicBool>,
}

impl Drop for RemoteThermometer {
    ///
    /// Выполнить остановку потока при удалении экземпляра удаленного
    /// "умного" термометра.
    ///
    fn drop(&mut self) {
        if let Some(w) = self.control.upgrade() {
            (*w).store(false, Ordering::Relaxed);
        }
    }
}

impl fmt::Display for RemoteThermometer {
    ///
    /// Получить информацию об удаленном "умном" термометре с помощью форматирования.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (id, temperature) = {
            let guard = self.data.read().unwrap();
            *guard
        };

        write!(
            f,
            "умный термометр \"{}\" ({}). Температура: {} °C.",
            self.name, id, temperature
        )
    }
}

impl Device for RemoteThermometer {
    ///
    /// Получить идентификатор удаленного "умного" термометра.
    ///
    fn id(&self) -> Uuid {
        let guard = self.data.read().unwrap();
        let (id, _) = *guard;

        id
    }

    ///
    /// Получить имя удаленного "умного" термометра.
    ///
    fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Обработать событие устройством.
    ///
    fn notify(&mut self, e: &dyn Event) -> Result<DeviceState, DeviceError> {
        if e.id() == StateEvent::ID {
            let (id, temperature) = {
                let guard = self.data.read().unwrap();
                *guard
            };

            Ok(DeviceState::for_thermometer(id, e.id(), temperature))
        } else {
            Err(DeviceError::NotImplementedEvent(e.id()))
        }
    }
}

impl RemoteThermometer {
    ///
    /// Создать объект по умолчанию для построения экземпляра удаленного
    /// "умного" термометра.
    ///
    #[inline]
    pub fn builder() -> RemoteThermometerBuilder<&'static str, &'static str> {
        RemoteThermometerBuilder::<&str, &str>::default()
    }

    ///
    /// Получить текущее значение температуры удаленного "умного" термометра.
    ///
    pub fn temperature(&self) -> Result<f64, DeviceError> {
        let guard = self.data.read().unwrap();
        let (_, temperature) = *guard;

        Ok(temperature)
    }
}

///
/// Структура для построения экзкмпляра удаленного "умного" термометра.
///
pub struct RemoteThermometerBuilder<BA, RA>
where
    BA: 'static + ToSocketAddrs + Send,
    RA: 'static + ToSocketAddrs + Send,
{
    ///
    /// Имя удаленного "умного" термометра.
    ///
    name: String,

    ///
    /// Адес привязки UDP-сокета.
    ///
    addr: BA,

    ///
    /// Адрес подключения автономного термометра.
    ///
    remote_addr: RA,
}

impl<BA: ToSocketAddrs + Send, RA: ToSocketAddrs + Send> RemoteThermometerBuilder<BA, RA> {
    ///
    /// Использовать имя удаленного "умного" термометра.
    ///
    #[inline]
    pub fn with_name<D: AsRef<str>>(self, name: D) -> Self {
        Self {
            name: name.as_ref().to_string(),
            addr: self.addr,
            remote_addr: self.remote_addr,
        }
    }

    ///
    /// Установить адрес привязки сокета удаленного "умного" термометра.
    ///
    #[inline]
    pub fn bind<BA2: 'static + ToSocketAddrs + Send>(
        self,
        addr: BA2,
    ) -> RemoteThermometerBuilder<BA2, RA> {
        RemoteThermometerBuilder::<BA2, RA> {
            name: self.name,
            addr,
            remote_addr: self.remote_addr,
        }
    }

    ///
    /// Установить адрес автономного "умного" термометра.
    ///
    #[inline]
    pub fn connect<RA2: 'static + ToSocketAddrs + Send>(
        self,
        addr: RA2,
    ) -> RemoteThermometerBuilder<BA, RA2> {
        RemoteThermometerBuilder::<BA, RA2> {
            name: self.name,
            addr: self.addr,
            remote_addr: addr,
        }
    }

    ///
    /// Выполнить построение экзкмпляра удаленного "умного" термометра.
    ///
    pub fn build(self) -> RemoteThermometer {
        let addr = self.addr;
        let remote_addr = self.remote_addr;
        let duration = time::Duration::from_millis(50);

        let working = Arc::new(AtomicBool::new(true));
        let control = Arc::downgrade(&working);

        let data = Arc::new(RwLock::new((Uuid::nil(), 0.0)));
        let cloned = data.clone();

        thread::spawn(move || -> Result<(), DeviceError> {
            let socket = UdpSocket::bind(addr)?;
            socket.connect(remote_addr)?;

            let mut buf = [0u8; 512];
            while (*working).load(Ordering::Relaxed) {
                if let Ok(received) = socket.recv(&mut buf) {
                    if let Ok(message) = bincode::options()
                        .with_big_endian()
                        .deserialize::<ThermometerMessage>(&buf[..received])
                    {
                        let mut guard = cloned.write().unwrap();
                        *guard = (message.id(), message.temperature());
                    } else {
                        log::error!("Message deserialization error");
                    }
                }

                thread::sleep(duration);
            }

            Ok(())
        });

        RemoteThermometer {
            name: self.name,
            data,
            control,
        }
    }
}

impl Default for RemoteThermometerBuilder<&str, &str> {
    ///
    /// Создать экземпляр по умолчанию построителя удаленного
    /// "умного" термометра.
    ///
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl RemoteThermometerBuilder<&str, &str> {
    ///
    /// Создать экземпляр с настройками по умолчанию построителя
    /// удаленного "умного" термометра.
    ///
    #[inline]
    pub fn new() -> Self {
        Self {
            name: "Untitled".to_owned(),
            addr: "127.0.0.1:8888",
            remote_addr: "127.0.0.1:8000",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smart_thermometer_test() {
        let thermometer1 = SmartThermometer::new("Thermometer1", 20.0);
        assert_eq!(thermometer1.name.as_str(), "Thermometer1");
        assert_eq!(thermometer1.temperature, 20.0);
    }

    #[test]
    fn autonomous_thermometer_builder_test() {
        let builder = AutonomousThermometer::builder()
            .bind("192.168.0.1:55334")
            .connect("192.168.0.2:55335")
            .with_noise();

        assert_eq!(builder.addr, "192.168.0.1:55334");
        assert_eq!(builder.remote_addr, "192.168.0.2:55335");
        assert!(builder.noisy);
    }
}
