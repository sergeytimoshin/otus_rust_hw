mod providers;
pub use providers::{BorrowingDeviceInfoProvider, DeviceInfoProvider, OwningDeviceInfoProvider};

mod models {
    mod house;
    mod socket;
    mod thermometer;

    pub use house::SmartHouse;
    pub use socket::SmartSocket;
    pub use thermometer::SmartThermometer;
}

use models::{SmartHouse, SmartSocket, SmartThermometer};
use std::collections::{HashMap, HashSet};

pub fn run() {
    let socket1_name = "socket_1";
    let socket2_name = "socket_2";
    let thermo_name = "thermo_1";

    let socket1 = SmartSocket::new(socket1_name, true, 100);
    let socket2 = SmartSocket::new(socket2_name, false, 20);
    let thermo = SmartThermometer::new(thermo_name, 40);

    let devices1 = HashSet::from([socket1_name.to_string(), socket2_name.to_string()]);
    let devices2 = HashSet::from([thermo_name.to_string()]);

    let rooms = HashMap::from([
        ("Hall".to_string(), devices1),
        ("Kitchen".to_string(), devices2),
    ]);

    let house = SmartHouse::new("Home", rooms);
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report(&info_provider_1);
    println!("{report1}");

    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = house.create_report(&info_provider_2);
    println!("{report2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owning_device_report() {
        let socket_name = "socket";
        let socket = SmartSocket::new(socket_name, true, 1);
        let socket_status = socket.to_string();
        let devices = HashSet::from([socket_name.to_string()]);
        let room_name = "hall";
        let rooms = HashMap::from([(room_name.to_string(), devices)]);
        let house = SmartHouse::new("home", rooms);
        let provider = OwningDeviceInfoProvider { socket };

        let report = house.create_report(&provider);
        let report_emulated = format!(
            "Room: {}, Device: {}, Status: {}\n",
            &room_name, &socket_name, socket_status
        );
        assert_eq!(report, report_emulated);
    }

    #[test]
    fn test_borrowing_device_report() {
        let socket_name = "socket";
        let thermo_name = "thermo";
        let socket = SmartSocket::new(socket_name, true, 1);
        let socket_status = socket.to_string();

        let thermo = SmartThermometer::new(thermo_name, 36);
        let thermo_status = thermo.to_string();

        let devices = HashSet::from([socket_name.to_string(), thermo_name.to_string()]);
        let room_name = "hall";
        let rooms = HashMap::from([(room_name.to_string(), devices)]);
        let house = SmartHouse::new("home", rooms);
        let provider = BorrowingDeviceInfoProvider {
            socket: &socket,
            thermo: &thermo,
        };

        let report = house.create_report(&provider);
        let report_emulated = format!(
            "Room: {}, Device: {}, Status: {}\nRoom: {}, Device: {}, Status: {}\n",
            &room_name, &socket_name, socket_status, &room_name, &thermo_name, thermo_status
        );
        assert_eq!(report, report_emulated);
    }
}
