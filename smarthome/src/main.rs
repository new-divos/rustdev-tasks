use smarthome::devices::{SmartSocket, SmartThermometer};

fn main() {
    let socket1 = SmartSocket::new("Первая розетка");
    println!("Информация о розетке 1: {}", socket1);

    let mut socket2 = SmartSocket::new("Вторая розетка");
    socket2.plug(1000.0);
    println!("Информация о розетке 2: {}", socket2);

    socket2.switch_off();
    println!("Информация о розетке 2: {}", socket2);

    socket2.switch_on();
    println!("Информация о розетке 2: {}", socket2);

    let thermometer = SmartThermometer::new("Термометр", 20.0);
    println!("Информация о термометре: {}", thermometer);
}
