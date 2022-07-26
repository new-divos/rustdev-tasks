use async_smarthome2::{
    device::{socket::SmartSocket, thermometer::SmartThermometer},
    house::{DeviceInfo, SmartHouse},
    room::SmartRoom,
};

fn main() {
    let mut sweet_home = SmartHouse::new("Милый дом");

    let mut bathroom = SmartRoom::new("Ванная");
    let socket1 = SmartSocket::new("Розетка для фена");
    bathroom += socket1;
    sweet_home += bathroom;

    let mut living_room = SmartRoom::new("Гостинная");

    let mut socket2 = SmartSocket::new("Розетка для кондиционера");
    socket2.switch_on();
    socket2.plug(2500.0);
    living_room += socket2;

    let thermometer1 = SmartThermometer::new("Термометр гостинной", 20.0);
    living_room += thermometer1;
    sweet_home += living_room;

    let mut kitchen = SmartRoom::new("Кухня");

    let socket3 = SmartSocket::new("Розетка для блендера");
    kitchen += socket3;

    let thermometer2 = SmartThermometer::new("Термометр кухни", 25.0);
    kitchen += thermometer2;
    sweet_home += kitchen;

    // Получить информацию о розетке кондиционера.
    match sweet_home.info("Гостинная", "Розетка для кондиционера") {
        Ok(info) => println!("Состояние розетки кондиционера: {}", info),
        Err(err) => eprintln!("Информация недоступна. Ошибка {}", err),
    }

    // Отобразить отчет о всех устройствах дома.
    println!("\n-----\n\n{}", sweet_home);
}
