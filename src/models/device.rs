use super::{Socket, Thermo};

pub enum Device {
    SocketDevice(Socket),
    ThermoDevice(Thermo)
}

// pub trait Device {
//     fn state(&self) -> String;
// }