use std::{
    net::ToSocketAddrs,
    sync::{Arc, Mutex},
    thread,
};

use log;

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
    house::{DeviceInfo, DeviceNotifier, RoomGetter, SmartHouse},
};

///
/// Сервер подсистемы управления "умного" дома.
///
pub struct ControlServer {
    server: Server,
    house: Arc<Mutex<SmartHouse>>,
}

impl ControlServer {
    ///
    /// Выполнить привязку сервера к сокету и экземпляру "умного" дома.
    ///
    #[inline]
    pub fn bind<A>(addrs: A, house: SmartHouse) -> Result<Self, BindError>
    where
        A: ToSocketAddrs,
    {
        Ok(Self {
            server: Server::bind(addrs)?,
            house: Arc::new(Mutex::new(house)),
        })
    }

    ///
    /// Запустить сервер для обработки сообщений.
    ///
    pub fn run(&self) {
        for connection in self.server.incoming() {
            let mut connection = match connection {
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

            let house = self.house.clone();
            thread::spawn(move || loop {
                let request = connection.recv::<ControlRequest>();
                let request = match request {
                    Ok(r) => r,
                    Err(_) => {
                        log::warn!("Connection lost when receiving data");
                        break;
                    }
                };

                let response = Self::dispatch(house.clone(), request.as_ref());
                if connection.send(response).is_err() {
                    log::warn!("Connection lost when sending data");
                    break;
                }
            });
        }
    }

    ///
    /// Выполнить диспетчеризацию запроса.
    ///
    fn dispatch(house: Arc<Mutex<SmartHouse>>, req: &ControlRequest) -> ControlResponse {
        match req.data {
            ControlRequestData::AcquireRooms => house.lock().unwrap().rooms().collect(),

            ControlRequestData::AcquireDevices(room_id) => {
                let lock = house.lock().unwrap();
                if let Some(room_ref) = lock.get(room_id) {
                    room_ref.devices().collect()
                } else {
                    ControlResponse::with_error(DeviceError::IllegalRoomId(room_id))
                }
            }

            ControlRequestData::AcquireDeviceState(room_id, device_id) => {
                let mut lock = house.lock().unwrap();
                match lock.notify(room_id, device_id, &StateEvent::new()) {
                    Ok(s) => ControlResponse::with_state(s),
                    Err(e) => ControlResponse::with_error(e),
                }
            }

            ControlRequestData::AcquireDeviceInfo(room_id, device_id) => {
                let lock = house.lock().unwrap();
                match lock.info(room_id, device_id) {
                    Ok(s) => ControlResponse::with_info(s),
                    Err(e) => ControlResponse::with_error(e),
                }
            }

            ControlRequestData::SwitchOnDevice(room_id, device_id) => {
                let mut lock = house.lock().unwrap();
                match lock.notify(room_id, device_id, &SwitchOnEvent::new()) {
                    Ok(s) => ControlResponse::with_state(s),
                    Err(e) => ControlResponse::with_error(e),
                }
            }

            ControlRequestData::SwitchOffDevice(room_id, device_id) => {
                let mut lock = house.lock().unwrap();
                match lock.notify(room_id, device_id, &SwitchOffEvent::new()) {
                    Ok(s) => ControlResponse::with_state(s),
                    Err(e) => ControlResponse::with_error(e),
                }
            }

            _ => ControlResponse::with_error(DeviceError::UnexpectedMessage),
        }
    }
}

///
/// Сервер управления "умной" розеткой.
///
pub struct SmartSocketServer {
    server: Server,
    socket: Arc<Mutex<SmartSocket>>,
}

impl SmartSocketServer {
    ///
    /// Выполнить привязку сервера к сокету и экземпляру "умной" розетки.
    ///
    #[inline]
    pub fn bind<A>(addrs: A, socket: SmartSocket) -> Result<Self, BindError>
    where
        A: ToSocketAddrs,
    {
        Ok(Self {
            server: Server::bind(addrs)?,
            socket: Arc::new(Mutex::new(socket)),
        })
    }

    ///
    /// Запустить сервер для обработки сообщений.
    ///
    pub fn run(&self) {
        for connection in self.server.incoming() {
            let mut connection = match connection {
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
            thread::spawn(move || loop {
                let request = connection.recv::<ControlRequest>();
                let request = match request {
                    Ok(r) => r,
                    Err(_) => {
                        log::warn!("Connection lost when receiving data");
                        break;
                    }
                };

                let response = Self::dispatch(socket.clone(), request.as_ref());
                if connection.send(response).is_err() {
                    log::warn!("Connection lost when sending data");
                    break;
                }
            });
        }
    }

    ///
    /// Выполнить диспетчеризацию запроса.
    ///
    fn dispatch(socket: Arc<Mutex<SmartSocket>>, req: &ControlRequest) -> ControlResponse {
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
