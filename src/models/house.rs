use crate::providers::DeviceInfoProvider;
use std::collections::{HashMap, HashSet};

pub struct SmartHouse {
    #[allow(dead_code)]
    name: String,
    rooms: HashMap<String, HashSet<String>>, // K - room name, V - list of devices for room
}

impl SmartHouse {
    pub fn new(name: &str, rooms: HashMap<String, HashSet<String>>) -> Self {
        SmartHouse {
            name: name.to_string(),
            rooms,
        }
    }

    fn get_rooms(&self) -> Vec<&String> {
        let mut rooms = Vec::from_iter(self.rooms.keys());
        rooms.sort();
        rooms
    }

    fn get_devices(&self, room: &str) -> Vec<&String> {
        let mut devices = Vec::from_iter(self.rooms.get(room).expect("Room not found"));
        devices.sort();
        devices
    }

    pub fn create_report(&self, provider: &dyn DeviceInfoProvider) -> String {
        let mut report: String = String::new();
        for room in self.get_rooms().iter() {
            for device in self.get_devices(room).iter() {
                if provider.devices().contains(device) {
                    report.push_str(&provider.status(room, device));
                    report.push('\n');
                }
            }
        }
        report
    }
}
