use std::{
    fmt,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Weak,
    },
    time,
};

use async_trait::async_trait;
use bincode::{self, Options};
use futures::executor::block_on;
use log;
use rand::{thread_rng, Rng};
use statrs::distribution::Normal;
use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    sync::{Mutex, RwLock},
};
use uuid::Uuid;

use crate::{
    control::message::ThermometerMessage,
    device::{AsyncDevice, DeviceState, Event, StateEvent},
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

#[async_trait]
impl AsyncDevice for SmartThermometer {
    ///
    /// Получить идентификатор "умного" термометра.
    ///
    #[inline]
    fn id(&self) -> Uuid {
        self.id
    }

    ///
    /// Получить имя "умного" термометра.
    ///
    #[inline]
    fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Обработать событие устройством.
    ///
    async fn async_notify(&mut self, e: Pin<Box<dyn Event>>) -> Result<DeviceState, DeviceError> {
        if e.id() == StateEvent::ID {
            Ok(DeviceState::for_thermometer(
                self.id,
                e.id(),
                self.temperature,
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
    thermometer: Arc<Mutex<SmartThermometer>>,

    ///
    /// Добавлять шум к показаниям температуры.
    ///
    noisy: bool,

    ///
    /// Флаг для завершения работы сервера.
    ///
    working: Arc<AtomicBool>,
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
    pub async fn run(&self) -> Result<(), DeviceError> {
        let duration = time::Duration::from_secs(3);

        let mut rng = thread_rng();
        let normal = Normal::new(0.0, 1.0).unwrap();

        while (*self.working).load(Ordering::Relaxed) {
            let (mut temperature, id) = {
                let mut guard = self.thermometer.lock().await;
                let state = guard.async_notify(Box::pin(StateEvent::new())).await?;
                (state.themperature().unwrap(), state.device_id())
            };
            if self.noisy {
                temperature += rng.sample(normal);
            }

            let message = ThermometerMessage::new(id, temperature);
            let bytes = bincode::options().with_big_endian().serialize(&message)?;

            log::info!(
                "Sending temperature {} °C of the device {} ...",
                temperature,
                id
            );
            self.socket.send(&bytes[..]).await?;

            tokio::time::sleep(duration).await;
        }

        Ok(())
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
    pub async fn build(
        self,
        thermometer: SmartThermometer,
    ) -> Result<(AutonomousThermometer, Weak<AtomicBool>), DeviceError> {
        let working = Arc::new(AtomicBool::new(true));
        let t = AutonomousThermometer {
            socket: UdpSocket::bind(self.addr).await?,
            thermometer: Arc::new(Mutex::new(thermometer)),
            noisy: self.noisy,
            working: working.clone(),
        };
        t.socket.connect(self.remote_addr).await?;

        Ok((t, Arc::downgrade(&working)))
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
            let guard = block_on(self.data.read());
            *guard
        };

        write!(
            f,
            "умный термометр \"{}\" ({}). Температура: {} °C.",
            self.name, id, temperature
        )
    }
}

#[async_trait]
impl AsyncDevice for RemoteThermometer {
    ///
    /// Получить идентификатор удаленного "умного" термометра.
    ///
    #[inline]
    fn id(&self) -> Uuid {
        block_on(self.get_id())
    }

    ///
    /// Получить имя удаленного "умного" термометра.
    ///
    #[inline]
    fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Обработать событие устройством.
    ///
    async fn async_notify(&mut self, e: Pin<Box<dyn Event>>) -> Result<DeviceState, DeviceError> {
        if e.id() == StateEvent::ID {
            let (id, temperature) = {
                let guard = self.data.read().await;
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
    /// Получить идентификатор удаленного "умного" термометра.
    ///
    async fn get_id(&self) -> Uuid {
        let guard = self.data.read().await;
        guard.0
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
    pub async fn build(self) -> RemoteThermometer {
        let addr = self.addr;
        let remote_addr = self.remote_addr;
        let duration = time::Duration::from_millis(50);

        let working = Arc::new(AtomicBool::new(true));
        let control = Arc::downgrade(&working);

        let data = Arc::new(RwLock::new((Uuid::nil(), 0.0)));
        let cloned = data.clone();

        tokio::spawn(async move {
            if let Ok(socket) = UdpSocket::bind(addr).await {
                if socket.connect(remote_addr).await.is_ok() {
                    let mut buf = [0u8; 512];
                    while (*working).load(Ordering::Relaxed) {
                        if let Ok(received) = socket.recv(&mut buf).await {
                            if let Ok(message) =
                                bincode::options()
                                    .with_big_endian()
                                    .deserialize::<ThermometerMessage>(&buf[..received])
                            {
                                let mut guard = cloned.write().await;
                                *guard = (message.id(), message.temperature());
                            } else {
                                log::error!("Message deserialization error");
                            }
                        }

                        tokio::time::sleep(duration).await;
                    }
                }
            }
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
