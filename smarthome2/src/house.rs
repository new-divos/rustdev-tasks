use std::collections::LinkedList;
use std::{fmt, iter, ops};

use uuid::Uuid;

use crate::device::{DeviceState, Event};
use crate::error::Error;
use crate::room::SmartRoom;

///
/// Типаж, позволяющий получить комнату "умного" дома.
///
pub trait RoomGetter<T> {
    type Output;

    ///
    /// Получить ссылку на комнату "умного" дома.
    ///
    fn get(&self, idx: T) -> Option<&Self::Output>;

    ///
    /// Получить изменяемую ссылку на комнату "умного" дома.
    ///
    fn get_mut(&mut self, idx: T) -> Option<&mut Self::Output>;
}

///
/// Типаж, описывающий получение информации об устройстве.
///
pub trait DeviceInfo<U, V> {
    ///
    /// Получить текстовую информацию об устройстве.
    ///
    fn info(&self, idx1: U, idx2: V) -> Result<String, Error>;
}

///
/// Типаж, описывающий обработку события некоторым устройством.
///
pub trait DeviceNotifier<U, V> {
    ///
    /// Обработать событие заданным устройством.
    ///
    fn notify(&mut self, idx1: U, idx2: V, e: &dyn Event) -> Result<DeviceState, Error>;
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
        let mut v = vec![format!("Умный дом \"{}\" ({}):", self.name, self.id)];
        for (idx, room) in self.rooms.iter().enumerate() {
            v.push(format!("{}. {}", idx + 1, *room));
        }

        write!(f, "{}", v.join("\n\n"))
    }
}

impl ops::AddAssign<SmartRoom> for SmartHouse {
    ///
    /// Добавить комнату в "умный" дом.
    ///
    fn add_assign(&mut self, room: SmartRoom) {
        if self.rooms.iter().all(|item| item.name() != room.name()) {
            self.rooms.push_back(room);
        }
    }
}

impl ops::SubAssign<Uuid> for SmartHouse {
    ///
    /// Удалить комнату с заданным идентификаторм из "умного" дома.
    ///
    fn sub_assign(&mut self, room_id: Uuid) {
        let mut rooms: LinkedList<SmartRoom> = LinkedList::new();
        while let Some(room) = self.rooms.pop_back() {
            if room.id() != room_id {
                rooms.push_front(room);
            }
        }

        self.rooms = rooms;
    }
}

impl ops::SubAssign<&str> for SmartHouse {
    ///
    /// Удалить комнату с заданным именем из "умного" дома.
    ///
    fn sub_assign(&mut self, room_name: &str) {
        let mut rooms: LinkedList<SmartRoom> = LinkedList::new();
        while let Some(room) = self.rooms.pop_back() {
            if room.name() != room_name {
                rooms.push_front(room);
            }
        }

        self.rooms = rooms;
    }
}

impl RoomGetter<Uuid> for SmartHouse {
    type Output = SmartRoom;

    ///
    /// Получить ссылку на комнату "умного" дома по ее идентификатору.
    ///
    fn get(&self, room_id: Uuid) -> Option<&Self::Output> {
        for room_ref in self.rooms.iter() {
            if room_ref.id() == room_id {
                return Some(room_ref);
            }
        }

        None
    }

    ///
    /// Получить изменяемую ссылку на комнату "умного" дома по ее идентификатору.
    ///
    fn get_mut(&mut self, room_id: Uuid) -> Option<&mut Self::Output> {
        for room_ref in self.rooms.iter_mut() {
            if room_ref.id() == room_id {
                return Some(room_ref);
            }
        }

        None
    }
}

impl RoomGetter<&str> for SmartHouse {
    type Output = SmartRoom;

    ///
    /// Получить ссылку на комнату "умного" дома по ее имени.
    ///
    fn get(&self, room_name: &str) -> Option<&Self::Output> {
        for room_ref in self.rooms.iter() {
            if room_ref.name() == room_name {
                return Some(room_ref);
            }
        }

        None
    }

    ///
    /// Получить изменяемую ссылку на комнату "умного" дома по ее имени.
    ///
    fn get_mut(&mut self, room_name: &str) -> Option<&mut Self::Output> {
        for room_ref in self.rooms.iter_mut() {
            if room_ref.name() == room_name {
                return Some(room_ref);
            }
        }

        None
    }
}

impl DeviceInfo<Uuid, Uuid> for SmartHouse {
    ///
    /// Получить информацию об устройстве по идентификатору комнаты
    /// и идентификатору устройства.
    ///
    fn info(&self, room_id: Uuid, device_id: Uuid) -> Result<String, Error> {
        if let Some(room) = self.get(room_id) {
            for device_ref in room.devices.iter() {
                if device_ref.id() == device_id {
                    return Ok(format!("{}", *device_ref));
                }
            }

            Err(Error::IllegalDeviceId(device_id))
        } else {
            Err(Error::IllegalRoomId(room_id))
        }
    }
}

impl DeviceInfo<Uuid, &str> for SmartHouse {
    ///
    /// Получить информацию об устройстве по идентификатору комнаты
    /// и имени устройства.
    ///
    fn info(&self, room_id: Uuid, device_name: &str) -> Result<String, Error> {
        if let Some(room) = self.get(room_id) {
            for device_ref in room.devices.iter() {
                if device_ref.name() == device_name {
                    return Ok(format!("{}", *device_ref));
                }
            }

            Err(Error::IllegalDeviceName(device_name.to_owned()))
        } else {
            Err(Error::IllegalRoomId(room_id))
        }
    }
}

impl DeviceInfo<&str, Uuid> for SmartHouse {
    ///
    /// Получить информацию об устройстве по имени комнаты
    /// и идентификатору устройства.
    ///
    fn info(&self, room_name: &str, device_id: Uuid) -> Result<String, Error> {
        if let Some(room) = self.get(room_name) {
            for device_ref in room.devices.iter() {
                if device_ref.id() == device_id {
                    return Ok(format!("{}", *device_ref));
                }
            }

            Err(Error::IllegalDeviceId(device_id))
        } else {
            Err(Error::IllegalRoomName(room_name.to_owned()))
        }
    }
}

impl DeviceInfo<&str, &str> for SmartHouse {
    ///
    /// Получить информацию об устройстве по имени комнаты
    /// и идентификатору устройства.
    ///
    fn info(&self, room_name: &str, device_name: &str) -> Result<String, Error> {
        if let Some(room) = self.get(room_name) {
            for device_ref in room.devices.iter() {
                if device_ref.name() == device_name {
                    return Ok(format!("{}", *device_ref));
                }
            }

            Err(Error::IllegalDeviceName(device_name.to_owned()))
        } else {
            Err(Error::IllegalRoomName(room_name.to_owned()))
        }
    }
}

impl DeviceNotifier<Uuid, Uuid> for SmartHouse {
    ///
    /// Обработать событие заданным устройством по идентификатору комнаты
    /// и идентификатору устройства.
    ///
    fn notify(
        &mut self,
        room_id: Uuid,
        device_id: Uuid,
        e: &dyn Event,
    ) -> Result<DeviceState, Error> {
        if let Some(room) = self.get_mut(room_id) {
            for device_ref in room.devices.iter_mut() {
                if device_ref.id() == device_id {
                    return device_ref.notify(e);
                }
            }

            Err(Error::IllegalDeviceId(device_id))
        } else {
            Err(Error::IllegalRoomId(room_id))
        }
    }
}

impl DeviceNotifier<Uuid, &str> for SmartHouse {
    ///
    /// Обработать событие заданным устройством по идентификатору комнаты
    /// и имени устройства.
    ///
    fn notify(
        &mut self,
        room_id: Uuid,
        device_name: &str,
        e: &dyn Event,
    ) -> Result<DeviceState, Error> {
        if let Some(room) = self.get_mut(room_id) {
            for device_ref in room.devices.iter_mut() {
                if device_ref.name() == device_name {
                    return device_ref.notify(e);
                }
            }

            Err(Error::IllegalDeviceName(device_name.to_owned()))
        } else {
            Err(Error::IllegalRoomId(room_id))
        }
    }
}

impl DeviceNotifier<&str, Uuid> for SmartHouse {
    ///
    /// Обработать событие заданным устройством по имени комнаты
    /// и идентификатору устройства.
    ///
    fn notify(
        &mut self,
        room_name: &str,
        device_id: Uuid,
        e: &dyn Event,
    ) -> Result<DeviceState, Error> {
        if let Some(room) = self.get_mut(room_name) {
            for device_ref in room.devices.iter_mut() {
                if device_ref.id() == device_id {
                    return device_ref.notify(e);
                }
            }

            Err(Error::IllegalDeviceId(device_id))
        } else {
            Err(Error::IllegalRoomName(room_name.to_owned()))
        }
    }
}

impl DeviceNotifier<&str, &str> for SmartHouse {
    ///
    /// Обработать событие заданным устройством по имени комнаты
    /// и имени устройства.
    ///
    fn notify(
        &mut self,
        room_name: &str,
        device_name: &str,
        e: &dyn Event,
    ) -> Result<DeviceState, Error> {
        if let Some(room) = self.get_mut(room_name) {
            for device_ref in room.devices.iter_mut() {
                if device_ref.name() == device_name {
                    return device_ref.notify(e);
                }
            }

            Err(Error::IllegalDeviceName(device_name.to_owned()))
        } else {
            Err(Error::IllegalRoomName(room_name.to_owned()))
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
    /// Запросить список идентификаторов и имен всех помещений.
    ///
    pub fn rooms(&self) -> impl iter::Iterator<Item = (Uuid, &str)> {
        self.rooms.iter().map(|room| (room.id(), room.name()))
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

    ///
    /// Обработать событие всеми устройствами "умного" дома.
    ///
    pub fn notify_all<'a>(
        &'a mut self,
        e: &'a dyn Event,
    ) -> impl iter::Iterator<Item = DeviceState> + 'a {
        self.rooms
            .iter_mut()
            .flat_map(|it| it.devices.iter_mut())
            .map(|device_ref| device_ref.notify(e))
            .filter_map(|r| r.ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smart_house_test() {
        let mut house1 = SmartHouse::new("House1");
        assert_eq!(house1.name, "House1");
        assert_eq!(house1.rooms.len(), 0);

        let room1 = SmartRoom::new("Room1");
        let room1_id = room1.id();
        house1 += room1;
        assert_eq!(house1.rooms.len(), 1);

        let room2 = SmartRoom::new("Room2");
        let room2_id = room2.id();
        house1 += room2;
        assert_eq!(house1.rooms.len(), 2);

        let room2_ex = SmartRoom::new("Room2");
        house1 += room2_ex;
        assert_eq!(house1.rooms.len(), 2);

        for ((id1, name1), (id2, name2)) in house1
            .rooms()
            .zip([(room1_id, "Room1"), (room2_id, "Room2")])
        {
            assert_eq!(id1, id2);
            assert_eq!(name1, name2);
        }

        house1 -= room1_id;
        assert_eq!(house1.rooms.len(), 1);
        for ((id1, name1), (id2, name2)) in house1.rooms().zip([(room2_id, "Room2")]) {
            assert_eq!(id1, id2);
            assert_eq!(name1, name2);
        }

        house1 -= "Room2";
        assert_eq!(house1.rooms.len(), 0);
    }
}
