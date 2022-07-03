use std::{
    net::ToSocketAddrs,
    sync::{Arc, Mutex},
};

use crate::{
    control::{
        message::{ControlRequest, ControlRequestData, ControlResponse},
        protocol::server::{Connection, Server},
    },
    device::{
        socket::{SwitchOffEvent, SwitchOnEvent},
        StateEvent,
    },
    error::{BindError, ConnectionError, DeviceError},
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
    /// Блокирующий итератор для входящих соединений.
    ///
    #[inline]
    pub fn incoming(&self) -> impl Iterator<Item = Result<Connection, ConnectionError>> + '_ {
        self.server.incoming()
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
        }
    }
}
