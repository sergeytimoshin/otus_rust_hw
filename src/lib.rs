mod models {
    mod device;
    mod house;
    mod room;
    mod socket;
    mod thermo;

    pub use device::Device;
    pub use house::{House, HouseError};
    pub use room::Room;
    pub use socket::Socket;
    pub use thermo::Thermo;
}

#[allow(unused_imports)]
use models::{Device, House, HouseError, Room, Socket, Thermo};
use std::collections::HashMap;

pub fn run() {
    let socket1 = Socket::new(true, 100);
    let socket2 = Socket::new(false, 20);
    let thermo1 = Thermo::new(40);

    let hall = Room::new(HashMap::from([
        ("socket1".to_string(), Device::SocketDevice(socket1)),
        ("thermo1".to_string(), Device::ThermoDevice(thermo1)),
    ]));

    let kitchen = Room::new(HashMap::from([(
        "Socket2".to_string(),
        Device::SocketDevice(socket2),
    )]));

    let rooms = HashMap::from([("Hall".to_string(), hall), ("Kitchen".to_string(), kitchen)]);

    let house = House::new(rooms);

    let report = house.create_report();
    match report {
        Ok(report) => println!("{report}"),
        Err(err) => println!("{:?}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_report() {
        let socket_name = "socket";
        let socket = Socket::new(true, 1);
        let socket_status = socket.to_string();

        let room = Room::new(HashMap::from([(
            socket_name.to_string(),
            Device::SocketDevice(socket),
        )]));

        let room_name = "room";
        let rooms = HashMap::from([(room_name.to_string(), room)]);

        let house = House::new(rooms);
        let report = house.create_report().expect("Rooms not found");

        let report_emulated = format!(
            "Room: {}\nSocket: {}, status: ({})\n",
            room_name, socket_name, socket_status
        );
        assert_eq!(report, report_emulated);
    }

    #[test]
    fn test_no_rooms_report() {
        let house = House::new(HashMap::new());
        let report = house.create_report();
        let err = Err(HouseError::RoomsNotFound);
        assert_eq!(err, report);
    }
}
