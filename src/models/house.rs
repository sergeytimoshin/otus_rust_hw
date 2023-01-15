use super::room::Room;
use std::collections::HashMap;
use crate::Device;

pub struct SmartHouse {
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    pub fn new(rooms: HashMap<String, Room>) -> Self {
        SmartHouse { rooms }
    }

    fn get_room_names(&self) -> Vec<&String> {
        let mut rooms = Vec::from_iter(self.rooms.keys());
        rooms.sort();
        rooms.sort();
        rooms
    }

    pub fn create_report(&self) -> String {
        let mut report: String = String::new();

        for room_name in self.get_room_names().iter() {
            let room = self.rooms.get(*room_name).expect("Room not found");
            report.push_str(&format!("Room: {}\n", room_name));
            for device_name in room.get_devices_names().iter() {
                let device = room.devices.get(*device_name).expect("Device not found");
                // let device_state = device.
                match device {
                    Device::SocketDevice(socket) => {
                        report.push_str(&format!("Socket: {}, status: ({})\n", device_name, socket));
                    },
                    Device::ThermoDevice(socket) => {
                        report.push_str(&format!("Thermo: {}, status: ({})\n", device_name, socket));
                    }
                }
                
            }
        }
        report
    }
}
