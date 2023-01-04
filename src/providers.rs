use crate::models::{SmartSocket, SmartThermometer};

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

pub trait DeviceInfoProvider {
    fn status(&self, room: &str, device: &str) -> String;
    fn devices(&self) -> Vec<&String>;
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn status(&self, room: &str, device: &str) -> String {
        if self.socket.name == device {
            format!(
                "Room: {}, Device: {}, Status: {}",
                room, device, self.socket
            )
        } else {
            format!("Room: {}, Device: {}, Status: not found", room, device)
        }
    }

    fn devices(&self) -> Vec<&String> {
        vec![&self.socket.name]
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn status(&self, room: &str, device: &str) -> String {
        if self.socket.name == device {
            format!(
                "Room: {}, Device: {}, Status: {}",
                room, device, self.socket
            )
        } else if self.thermo.name == device {
            format!(
                "Room: {}, Device: {}, Status: {}",
                room, device, self.thermo
            )
        } else {
            format!("Room: {}, Device: {}, Status: not found", room, device)
        }
    }

    fn devices(&self) -> Vec<&String> {
        vec![&self.socket.name, &self.thermo.name]
    }
}
