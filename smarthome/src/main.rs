use smarthome::{SmartSocket, SmartThermometer};

fn main() {
    let socket1 = SmartSocket::new();
    println!("Информация о розетке 1: {}", socket1.info());

    let mut socket2 = SmartSocket::with_power(1000.0);
    println!("Информация о розетке 2: {}", socket2.info());

    socket2.switch_off();
    println!("Информация о розетке 2: {}", socket2.info());

    socket2.switch_on();
    println!("Информация о розетке 2: {}", socket2.info());

    let thermometer = SmartThermometer::with_temperature(20.0);
    println!("Информация о термометре: {}", thermometer.info());
}
