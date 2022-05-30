use smarthome::devices::{SmartSocket, SmartThermometer};
use smarthome::rooms::SmartHouse;

fn main() {
    let mut smart_house = SmartHouse::new("Дом, милый дом");

    let bedroom_id = smart_house.add_room("Спальная").unwrap();
    let kitchen_id = smart_house.add_room("Кухня").unwrap();

    let socket1 = SmartSocket::new("Первая розетка");
    println!("Информация о розетке 1: {}", socket1);

    smart_house.add_device(bedroom_id, socket1).unwrap();

    let mut socket2 = SmartSocket::new("Вторая розетка");
    socket2.plug(1000.0);
    println!("Информация о розетке 2: {}", socket2);

    socket2.switch_off();
    println!("Информация о розетке 2: {}", socket2);

    socket2.switch_on();
    println!("Информация о розетке 2: {}", socket2);
    smart_house.add_device(kitchen_id, socket2).unwrap();

    let thermometer = SmartThermometer::new("Термометр", 20.0);
    println!("Информация о термометре: {}", thermometer);
    smart_house.add_device(kitchen_id, thermometer).unwrap();

    print!("\n\n{}", smart_house);
}
