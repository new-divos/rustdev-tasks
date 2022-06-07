use std::collections::LinkedList;
use std::{fmt, iter, ops};

use uuid::Uuid;

use crate::device::Device;

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
    pub(crate) devices: LinkedList<Box<dyn Device>>,
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

impl<T: 'static + Device> ops::AddAssign<T> for SmartRoom {
    ///
    /// Добавить устройство для комнаты "умного" дома.
    ///
    fn add_assign(&mut self, device: T) {
        if self.devices.iter().all(|item| item.name() != device.name()) {
            self.devices.push_back(Box::new(device));
        }
    }
}

impl ops::SubAssign<Uuid> for SmartRoom {
    ///
    /// Удалить устройство с заданным идентификатором.
    ///
    fn sub_assign(&mut self, device_id: Uuid) {
        let mut devices: LinkedList<Box<dyn Device>> = LinkedList::new();
        while let Some(device_ref) = self.devices.pop_back() {
            if device_ref.id() != device_id {
                devices.push_front(device_ref);
            }
        }

        self.devices = devices;
    }
}

impl ops::SubAssign<&str> for SmartRoom {
    ///
    /// Удалить устройство с заданным именем.
    ///
    fn sub_assign(&mut self, device_name: &str) {
        let mut devices: LinkedList<Box<dyn Device>> = LinkedList::new();
        while let Some(device_ref) = self.devices.pop_back() {
            if device_ref.name() != device_name {
                devices.push_front(device_ref);
            }
        }

        self.devices = devices;
    }
}

impl SmartRoom {
    ///
    ///Создать комнату "умного" дома с заданным именем.
    ///
    pub fn new(name: &str) -> Self {
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
    /// Запросить список идентификаторов и имен всех устройств.
    ///
    pub fn devices(&self) -> impl iter::Iterator<Item = (Uuid, &str)> {
        self.devices
            .iter()
            .map(|device| (device.id(), device.name()))
    }
}

#[cfg(test)]
mod tests {
    use crate::device::socket::SmartSocket;
    use crate::device::thermometer::SmartThermometer;

    use super::*;

    #[test]
    fn smart_room_test() {
        let mut room1 = SmartRoom::new("Room1");
        assert_eq!(room1.name.as_str(), "Room1");

        let socket1 = SmartSocket::new("Socket1");
        let socket1_id = socket1.id();
        room1 += socket1;

        let thermometer1 = SmartThermometer::new("Thermometer1", 20.0);
        let thermometer1_id = thermometer1.id();
        room1 += thermometer1;

        for ((id1, name1), (id2, name2)) in room1
            .devices()
            .zip([(socket1_id, "Socket1"), (thermometer1_id, "Thermometer1")].iter())
        {
            assert_eq!(id1, *id2);
            assert_eq!(name1, *name2);
        }

        room1 -= thermometer1_id;
        for ((id1, name1), (id2, name2)) in room1.devices().zip([(socket1_id, "Socket1")].iter()) {
            assert_eq!(id1, *id2);
            assert_eq!(name1, *name2);
        }

        room1 -= "Socket1";
        assert_eq!(room1.devices().count(), 0);
    }
}
