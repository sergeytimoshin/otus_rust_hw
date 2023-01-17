use super::{Socket, Thermo};

pub enum Device {
    SocketDevice(Socket),
    ThermoDevice(Thermo)
}