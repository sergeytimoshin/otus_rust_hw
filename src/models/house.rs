use super::room::Room;
use crate::Device;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum HouseError {
    RoomsNotFound,
}

impl fmt::Display for HouseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rooms not found!")
    }
}

impl std::error::Error for HouseError {}

pub struct House {
    rooms: HashMap<String, Room>,
}

impl House {
    pub fn new(rooms: HashMap<String, Room>) -> Self {
        House { rooms }
    }

    fn get_room_names(&self) -> Vec<&String> {
        let mut rooms = Vec::from_iter(self.rooms.keys());
        rooms.sort();
        rooms
    }

    pub fn create_report(&self) -> Result<String, HouseError> {
        let mut report: String = String::new();

        if self.rooms.keys().len() == 0 {
            return Err(HouseError::RoomsNotFound);
        }

        for room_name in self.get_room_names().iter() {
            let room = self.rooms.get(*room_name).expect("Room not found");
            report.push_str(&format!("Room: {}\n", room_name));
            for device_name in room.get_devices_names().iter() {
                let device = room.devices.get(*device_name).expect("Device not found");
                match device {
                    Device::SocketDevice(socket) => {
                        report
                            .push_str(&format!("Socket: {}, status: ({})\n", device_name, socket));
                    }
                    Device::ThermoDevice(socket) => {
                        report
                            .push_str(&format!("Thermo: {}, status: ({})\n", device_name, socket));
                    }
                }
            }
        }
        Ok(report)
    }
}
