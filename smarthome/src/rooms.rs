use std::collections::LinkedList;
use std::{fmt, iter, ops};

use uuid::Uuid;

use crate::devices::Device;
use crate::errors::Error;

///
/// Структура, описывающая комнату "умного" дома.
///
pub struct SmartRoom {
    ///
    /// Идентификатор комнаты "умного" дома.
    ///
    id: Uuid,

    ///
    /// Наименование комнаты "умного" дома.
    ///
    name: String,

    ///
    /// Список устройств комнаты "умного" дома.
    ///
    devices: LinkedList<Box<dyn Device>>,
}

impl fmt::Display for SmartRoom {
    ///
    /// Получить информацию о комнате "умного" дома и ее устойствах
    /// с помощью форматирования.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = vec![format!("Комната {} ({}). Устройства: ", self.name, self.id)];
        for device_ref in self.devices.iter() {
            v.push(format!("\t- {};", *device_ref));
        }

        write!(f, "{}", v.join("\n"))
    }
}

impl SmartRoom {
    /**
    Создать комнату "умного" дома с заданным именем.
    */
    pub(crate) fn new(name: &str) -> Self {
        SmartRoom {
            id: Uuid::new_v4(),
            name: name.to_string(),
            devices: LinkedList::new(),
        }
    }

    ///
    /// Получить идентификатор комнаты "умного" дома.
    ///
    pub fn id(&self) -> Uuid {
        self.id
    }

    ///
    /// Получить имя комнаты "умного" дома.
    ///
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Добавить устройство для комнаты "умного" дома.
    ///
    pub fn add_device<T: 'static + Device>(&mut self, device: T) -> Result<Uuid, Error> {
        let count = self
            .devices
            .iter()
            .filter(|&item| item.name() == device.name())
            .count();
        if count > 0 {
            return Err(Error::IllegalDeviceName);
        }

        let device_id = device.id();
        self.devices.push_back(Box::new(device));

        Ok(device_id)
    }

    ///
    /// Удалить устройство с заданным идентификатором.
    ///
    pub fn remove_device(&mut self, device_id: Uuid) {
        let mut devices: LinkedList<Box<dyn Device>> = LinkedList::new();
        while let Some(device_ref) = self.devices.pop_back() {
            if device_ref.id() != device_id {
                devices.push_front(device_ref);
            }
        }

        self.devices = devices;
    }

    ///
    /// Удалить устройство с заданным именем.
    ///
    pub fn remove_device_by_name(&mut self, device_name: &str) {
        let mut devices: LinkedList<Box<dyn Device>> = LinkedList::new();
        while let Some(device_ref) = self.devices.pop_back() {
            if device_ref.name() != device_name {
                devices.push_front(device_ref);
            }
        }

        self.devices = devices;
    }

    ///
    /// Запросить список идентификаторов и имен всех устройств.
    ///
    pub fn devices(&self) -> Vec<(Uuid, &str)> {
        self.devices
            .iter()
            .map(|device| (device.id(), device.name()))
            .collect()
    }
}

///
/// Структура, описывающая "умный" дом.
///
pub struct SmartHouse {
    ///
    /// Идентификатор "умного" дома.
    ///
    id: Uuid,

    ///
    /// Наименование "умного" дома.
    ///
    name: String,

    ///
    /// Список комнат "умного" дома.
    ///
    rooms: LinkedList<SmartRoom>,
}

impl fmt::Display for SmartHouse {
    ///
    /// Получить информацию об "умном" доме и его устойствах
    /// с помощью форматирования.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v = vec![format!("Умный дом {} ({})", self.name, self.id)];
        for (idx, room) in self.rooms.iter().enumerate() {
            v.push(format!("{} {}", idx + 1, *room));
        }

        write!(f, "{}", v.join("\n\n"))
    }
}

impl ops::Index<Uuid> for SmartHouse {
    type Output = SmartRoom;

    ///
    /// Получить ссылку на комнату "умного" дома по ее идентификатору.
    ///
    fn index(&self, index: Uuid) -> &Self::Output {
        if let Some(room_ref) = self.get(index) {
            room_ref
        } else {
            panic!("Illegal a room identifier {}", index)
        }
    }
}

impl ops::IndexMut<Uuid> for SmartHouse {
    ///
    /// Получить изменяемую ссылку на комнату "умного" дома по ее идентификатору.
    ///
    fn index_mut(&mut self, index: Uuid) -> &mut Self::Output {
        if let Some(room_ref) = self.get_mut(index) {
            room_ref
        } else {
            panic!("Illegal a room identifier {}", index)
        }
    }
}

impl SmartHouse {
    ///
    /// Создать "умный" дом с заданным именем.
    ///
    pub fn new(name: &str) -> Self {
        SmartHouse {
            id: Uuid::new_v4(),
            name: name.to_string(),
            rooms: LinkedList::new(),
        }
    }

    ///
    /// Получить идентификатор "умного" дома.
    ///
    pub fn id(&self) -> Uuid {
        self.id
    }

    ///
    /// Получить имя "умного" дома.
    ///
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    ///
    /// Добавить комнату с заданным именем.
    ///
    pub fn add_room(&mut self, room_name: &str) -> Result<Uuid, Error> {
        let count = self
            .rooms
            .iter()
            .filter(|&room| room.name() == room_name)
            .count();
        if count > 0 {
            return Err(Error::IllegalRoomName);
        }

        let new_room = SmartRoom::new(room_name);
        let room_id = new_room.id();

        self.rooms.push_back(new_room);
        Ok(room_id)
    }

    ///
    /// Удалить комнату с заданным идентификатором.
    ///
    pub fn remove_room(&mut self, room_id: Uuid) {
        let mut rooms: LinkedList<SmartRoom> = LinkedList::new();
        while let Some(room) = self.rooms.pop_back() {
            if room.id != room_id {
                rooms.push_front(room);
            }
        }

        self.rooms = rooms;
    }

    ///
    /// Удалить комнату с заданным именем.
    ///
    pub fn remove_room_by_name(&mut self, room_name: &str) {
        let mut rooms: LinkedList<SmartRoom> = LinkedList::new();
        while let Some(room) = self.rooms.pop_back() {
            if room.name() != room_name {
                rooms.push_front(room);
            }
        }

        self.rooms = rooms;
    }

    ///
    /// Добавить устройство к комнате с заданным идентификатором.
    ///
    pub fn add_device<T: 'static + Device>(
        &mut self,
        room_id: Uuid,
        device: T,
    ) -> Result<Uuid, Error> {
        if let Some(room_ref) = self.get_mut(room_id) {
            room_ref.add_device(device)
        } else {
            Err(Error::IllegalRoomId)
        }
    }

    ///
    /// Удалить устройство из комнаты с заданным идентификатором.
    ///
    pub fn remove_device(&mut self, room_id: Uuid, device_id: Uuid) {
        if let Some(room_ref) = self.get_mut(room_id) {
            room_ref.remove_device(device_id)
        }
    }

    ///
    /// Получить список идентификаторов и имен устройств для комнаты
    /// с заданным идентификатором.
    ///
    pub fn devices(&self, room_id: Uuid) -> Result<Vec<(Uuid, &str)>, Error> {
        if let Some(room_ref) = self.get(room_id) {
            Ok(room_ref.devices())
        } else {
            Err(Error::IllegalRoomId)
        }
    }

    ///
    /// Получить информацию об устройстве по идентификатору комнаты
    /// и идентификатору устройства.
    ///
    pub fn device_info(&self, room_id: Uuid, device_id: Uuid) -> Result<String, Error> {
        if let Some(room_ref) = self.get(room_id) {
            for device_ref in room_ref.devices.iter() {
                if device_ref.id() == device_id {
                    return Ok(format!("{}", *device_ref));
                }
            }

            Err(Error::IllegalDeviceId)
        } else {
            Err(Error::IllegalRoomId)
        }
    }

    ///
    /// Получить информацию об устройстве по имени комнаты
    /// и имени устройства.
    ///
    pub fn device_info_by_name(&self, room_name: &str, device_name: &str) -> Result<String, Error> {
        for room_ref in self.rooms.iter() {
            if room_ref.name() == room_name {
                for device_ref in room_ref.devices.iter() {
                    if device_ref.name() == device_name {
                        return Ok(format!("{}", *device_ref));
                    }
                }

                return Err(Error::IllegalDeviceName);
            }
        }

        Err(Error::IllegalRoomName)
    }

    ///
    /// Получить ссылку на комнату "умного" дома по ее идентификатору.
    ///
    pub fn get(&self, id: Uuid) -> Option<&SmartRoom> {
        for room_ref in self.rooms.iter() {
            if room_ref.id == id {
                return Some(room_ref);
            }
        }

        None
    }

    ///
    /// Получить изменяемую ссылку на комнату "умного" дома по ее идентификатору.
    ///
    pub fn get_mut(&mut self, id: Uuid) -> Option<&mut SmartRoom> {
        for room_ref in self.rooms.iter_mut() {
            if room_ref.id == id {
                return Some(room_ref);
            }
        }

        None
    }

    ///
    /// Запросить список идентификаторов и имен всех помещений.
    ///
    pub fn rooms(&self) -> Vec<(Uuid, &str)> {
        self.rooms
            .iter()
            .map(|room| (room.id, room.name()))
            .collect()
    }

    ///
    /// Получить неизменяемый итератор для перебора всех комнат.
    ///
    pub fn iter(&self) -> impl iter::Iterator<Item = &SmartRoom> {
        self.rooms.iter()
    }

    ///
    /// Получить изменяемый итератор для перебора всех комнат.
    ///
    pub fn iter_mut(&mut self) -> impl iter::Iterator<Item = &mut SmartRoom> {
        self.rooms.iter_mut()
    }
}
