use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex, Weak,
};

use log;
use tokio::net::ToSocketAddrs;

use crate::{
    control::{
        message::{ControlRequest, ControlRequestData, ControlResponse},
        protocol::server::Server,
    },
    device::{
        socket::{SmartSocket, SwitchOffEvent, SwitchOnEvent},
        Device, StateEvent,
    },
    error::{BindError, DeviceError},
};

///
/// Сервер управления "умной" розеткой.
///
pub struct SmartSocketServer {
    ///
    /// Сервер обмена сообщениями.
    ///
    server: Server,

    ///
    /// Экземпляр "умной" розетки.
    ///
    socket: Arc<Mutex<SmartSocket>>,

    ///
    /// Флаг завершения работы сервера.
    ///
    working: Arc<AtomicBool>,
}

impl SmartSocketServer {
    ///
    /// Выполнить привязку сервера к сокету и экземпляру "умной" розетки.
    ///
    pub async fn bind<A>(
        addrs: A,
        socket: SmartSocket,
    ) -> Result<(Self, Weak<AtomicBool>), BindError>
    where
        A: ToSocketAddrs,
    {
        let working = Arc::new(AtomicBool::new(true));

        Ok((
            Self {
                server: Server::bind(addrs).await?,
                socket: Arc::new(Mutex::new(socket)),
                working: working.clone(),
            },
            Arc::downgrade(&working),
        ))
    }

    ///
    /// Запустить сервер для обработки сообщений.
    ///
    pub async fn run(&self) {
        while (*self.working).load(Ordering::Relaxed) {
            let connection = match self.server.accept().await {
                Ok(c) => c,
                Err(e) => {
                    log::error!("Cannot establish connection {}", e);
                    continue;
                }
            };

            let addr = match connection.peer_addr() {
                Ok(addr) => addr.to_string(),
                Err(_) => "unknown".to_owned(),
            };

            log::info!("New client connected: {}", addr);

            let socket = self.socket.clone();
            tokio::spawn(async move {
                loop {
                    let request = connection.recv::<ControlRequest>().await;
                    let request = match request {
                        Ok(r) => r,
                        Err(_) => {
                            log::warn!("Connection lost when receiving data");
                            break;
                        }
                    };

                    let response = Self::dispatch(socket.clone(), request.as_ref()).await;
                    if connection.send(response).await.is_err() {
                        log::warn!("Connection lost when sending data");
                        break;
                    }
                }
            });
        }
    }

    ///
    /// Выполнить диспетчеризацию запроса.
    ///
    async fn dispatch(socket: Arc<Mutex<SmartSocket>>, req: &ControlRequest) -> ControlResponse {
        match req.data {
            ControlRequestData::AcquireRemoteDeviceState => {
                let mut lock = socket.lock().unwrap();
                log::info!("Requesting device {} state", lock.id());

                match lock.notify(&StateEvent::new()) {
                    Ok(s) => ControlResponse::with_state(s),
                    Err(e) => ControlResponse::with_error(e),
                }
            }

            ControlRequestData::AcquireRemoteDeviceName => {
                let lock = socket.lock().unwrap();
                log::info!("Obtaining device {} name \"{}\"", lock.id(), lock.name());

                ControlResponse::with_name(lock.id(), lock.name())
            }

            ControlRequestData::SwitchOnRemoteDevice => {
                let mut lock = socket.lock().unwrap();
                log::info!("Switching on device {}", lock.id());

                match lock.notify(&SwitchOnEvent::new()) {
                    Ok(s) => ControlResponse::with_state(s),
                    Err(e) => ControlResponse::with_error(e),
                }
            }

            ControlRequestData::SwitchOffRemoteDevice => {
                let mut lock = socket.lock().unwrap();
                log::info!("Switching off device {}", lock.id());

                match lock.notify(&SwitchOffEvent::new()) {
                    Ok(s) => ControlResponse::with_state(s),
                    Err(e) => ControlResponse::with_error(e),
                }
            }

            _ => ControlResponse::with_error(DeviceError::UnexpectedMessage),
        }
    }
}
