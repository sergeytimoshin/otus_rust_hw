use std::collections::HashMap;
use super::device::Device;

pub struct Room {
    pub devices: HashMap<String, Device>,
}

impl Room {
    pub fn new(devices: HashMap<String, Device>) -> Room {
        Room {
            devices
        }
    }

    pub fn get_devices_names(&self) -> Vec<&String> {
        let mut device_names = Vec::from_iter(self.devices.keys());        
        device_names.sort();
        device_names
    }
}