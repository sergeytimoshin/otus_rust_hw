mod models {
    mod house;
    mod room;
    mod device;
    mod socket;
    mod thermo;

    pub use house::SmartHouse;
    pub use room::Room;
    pub use device::Device;
    pub use socket::Socket;
    pub use thermo::Thermo;
}

use models::{SmartHouse, Socket, Thermo, Room, Device};
use std::collections::HashMap;

pub fn run() {
    let socket1 = Socket::new(true, 100);
    let socket2 = Socket::new(false, 20);
    let thermo1 = Thermo::new(40);
    
    let hall = Room::new(HashMap::from([
        ("socket1".to_string(), Device::SocketDevice(socket1)),
        ("thermo1".to_string(), Device::ThermoDevice(thermo1)),
    ]));

    let kitchen = Room::new(HashMap::from([
        ("Socket2".to_string(),Device::SocketDevice(socket2)),
    ]));

    let rooms = HashMap::from([
        ("Hall".to_string(), hall),
        ("Kitchen".to_string(), kitchen),
    ]);

    let house = SmartHouse::new(rooms);

    let report = house.create_report();
    println!("{report}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_report() {
        let socket_name = "socket";
        let socket = Socket::new(true, 1);
        let socket_status = socket.to_string();

        let room = Room::new(HashMap::from([
            (socket_name.to_string(), Device::SocketDevice(socket)),
        ]));

        let room_name = "room";
        let rooms = HashMap::from([
            (room_name.to_string(), room),
        ]);

        let house = SmartHouse::new(rooms);
        let report = house.create_report();

        let report_emulated = format!(
            "Room: {}\nSocket: {}, status: ({})\n",
            room_name, socket_name, socket_status
        );
        assert_eq!(report, report_emulated);
    }
}
