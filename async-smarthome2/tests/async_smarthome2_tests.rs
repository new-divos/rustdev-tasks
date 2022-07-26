#![allow(dead_code)]

use uuid::Uuid;

use async_smarthome2::{
    device::{
        socket::{SmartSocket, SwitchOffEvent, SwitchOnEvent},
        thermometer::SmartThermometer,
        AsyncDevice, StateEvent,
    },
    house::{DeviceInfo, DeviceNotifier, RoomGetter, SmartHouse},
    room::SmartRoom,
};

#[tokio::test]
async fn smart_home_test() {
    let socket1 = SmartSocket::new("Socket1");
    let socket1_id = socket1.id();

    let thermometer1 = SmartThermometer::new("Thermometer1", 20.0);
    let thermometer1_id = thermometer1.id();

    let mut room1 = SmartRoom::new("Room1");
    let room1_id = room1.id();
    room1 += socket1;
    room1 += thermometer1;

    let socket2 = SmartSocket::new("Socket2");
    let socket2_id = socket2.id();

    let thermometer2 = SmartThermometer::new("Thermometer2", 25.0);
    let thermometer2_id = thermometer2.id();

    let mut room2 = SmartRoom::new("Room2");
    let room2_id = room2.id();
    room2 += socket2;
    room2 += thermometer2;

    let mut house1 = SmartHouse::new("House1");
    house1 += room1;
    house1 += room2;

    let room_ref = house1.get(room1_id).unwrap();
    assert_eq!(room_ref.id(), room1_id);
    assert_eq!(room_ref.name(), "Room1");

    let room_ref = house1.get("Room2").unwrap();
    assert_eq!(room_ref.id(), room2_id);
    assert_eq!(room_ref.name(), "Room2");

    let room_ref = house1.get_mut(room2_id).unwrap();
    assert_eq!(room_ref.id(), room2_id);
    assert_eq!(room_ref.name(), "Room2");

    let socket3 = SmartSocket::new("Socket3");
    *room_ref += socket3;
    assert_eq!(room_ref.devices().count(), 3);
    assert!(room_ref.devices().any(|(_, name)| name == "Socket3"));

    let room_ref = house1.get_mut("Room1").unwrap();
    assert_eq!(room_ref.id(), room1_id);
    assert_eq!(room_ref.name(), "Room1");

    let thermometer3 = SmartThermometer::new("Thermometer3", 30.0);

    *room_ref += thermometer3;
    assert_eq!(room_ref.devices().count(), 3);
    assert!(room_ref.devices().any(|(_, name)| name == "Thermometer3"));

    assert!(house1.get(Uuid::new_v4()).is_none());
    assert!(house1.get_mut(Uuid::new_v4()).is_none());
    assert!(house1.get("Room1814").is_none());
    assert!(house1.get_mut("Room1814").is_none());

    let info = house1.info(room1_id, socket1_id).unwrap();
    assert!(info.contains("Socket1"));
    assert!(info.contains(socket1_id.to_string().as_str()));

    let info = house1.info(room1_id, "Thermometer1").unwrap();
    assert!(info.contains("Thermometer1"));
    assert!(info.contains(thermometer1_id.to_string().as_str()));

    let info = house1.info("Room2", socket2_id).unwrap();
    assert!(info.contains("Socket2"));
    assert!(info.contains(socket2_id.to_string().as_str()));

    let info = house1.info("Room2", "Thermometer2").unwrap();
    assert!(info.contains("Thermometer2"));
    assert!(info.contains(thermometer2_id.to_string().as_str()));

    assert!(house1.info(room1_id, Uuid::new_v4()).is_err());
    assert!(house1.info(room1_id, "Mixer").is_err());
    assert!(house1.info(Uuid::new_v4(), Uuid::new_v4()).is_err());
    assert!(house1.info(Uuid::new_v4(), "Mixer").is_err());
    assert!(house1.info("Room1", Uuid::new_v4()).is_err());
    assert!(house1.info("Room1", "Mixer").is_err());
    assert!(house1.info("Room1814", Uuid::new_v4()).is_err());
    assert!(house1.info("Room1814", "Mixer").is_err());

    let state = house1
        .async_notify(room1_id, socket1_id, Box::pin(StateEvent::new()))
        .await
        .unwrap();
    assert!(!state.enabled().unwrap());

    let state = house1
        .async_notify(room1_id, socket1_id, Box::pin(SwitchOnEvent::new()))
        .await
        .unwrap();
    assert!(state.enabled().unwrap());

    let state = house1
        .async_notify(room1_id, socket1_id, Box::pin(SwitchOffEvent::new()))
        .await
        .unwrap();
    assert!(!state.enabled().unwrap());

    let state = house1
        .async_notify(room1_id, thermometer1_id, Box::pin(StateEvent::new()))
        .await
        .unwrap();
    assert_eq!(state.themperature().unwrap(), 20.0);

    let state = house1
        .async_notify(room1_id, "Socket1".to_string(), Box::pin(StateEvent::new()))
        .await
        .unwrap();
    assert!(!state.enabled().unwrap());

    let state = house1
        .async_notify(
            room1_id,
            "Socket1".to_string(),
            Box::pin(SwitchOnEvent::new()),
        )
        .await
        .unwrap();
    assert!(state.enabled().unwrap());

    let state = house1
        .async_notify(
            room1_id,
            "Socket1".to_string(),
            Box::pin(SwitchOffEvent::new()),
        )
        .await
        .unwrap();
    assert!(!state.enabled().unwrap());

    let state = house1
        .async_notify(
            room1_id,
            "Thermometer1".to_string(),
            Box::pin(StateEvent::new()),
        )
        .await
        .unwrap();
    assert_eq!(state.themperature().unwrap(), 20.0);

    let state = house1
        .async_notify("Room1".to_string(), socket1_id, Box::pin(StateEvent::new()))
        .await
        .unwrap();
    assert!(!state.enabled().unwrap());

    let state = house1
        .async_notify(
            "Room1".to_string(),
            socket1_id,
            Box::pin(SwitchOnEvent::new()),
        )
        .await
        .unwrap();
    assert!(state.enabled().unwrap());

    let state = house1
        .async_notify(
            "Room1".to_string(),
            socket1_id,
            Box::pin(SwitchOffEvent::new()),
        )
        .await
        .unwrap();
    assert!(!state.enabled().unwrap());

    let state = house1
        .async_notify(
            "Room1".to_string(),
            thermometer1_id,
            Box::pin(StateEvent::new()),
        )
        .await
        .unwrap();
    assert_eq!(state.themperature().unwrap(), 20.0);

    let state = house1
        .async_notify(
            "Room1".to_string(),
            "Socket1".to_string(),
            Box::pin(StateEvent::new()),
        )
        .await
        .unwrap();
    assert!(!state.enabled().unwrap());

    let state = house1
        .async_notify(
            "Room1".to_string(),
            "Socket1".to_string(),
            Box::pin(SwitchOnEvent::new()),
        )
        .await
        .unwrap();
    assert!(state.enabled().unwrap());

    let state = house1
        .async_notify(
            "Room1".to_string(),
            "Socket1".to_string(),
            Box::pin(SwitchOffEvent::new()),
        )
        .await
        .unwrap();
    assert!(!state.enabled().unwrap());

    let state = house1
        .async_notify(
            "Room1".to_string(),
            "Thermometer1".to_string(),
            Box::pin(StateEvent::new()),
        )
        .await
        .unwrap();
    assert_eq!(state.themperature().unwrap(), 20.0);
}
