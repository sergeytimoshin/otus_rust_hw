use std::fmt::{self};

pub struct SmartSocket {
    pub name: String,
    pub on: bool,
    pub power: i32,
}

impl SmartSocket {
    pub fn new(name: &str, on: bool, power: i32) -> SmartSocket {
        SmartSocket {
            name: name.to_string(),
            on,
            power,
        }
    }

    #[allow(dead_code)]
    fn toggle(&mut self) {
        self.on = !self.on;
    }

    #[allow(dead_code)]
    fn power_info(&self) {
        println!("Socket power is {}.\n", self.power)
    }
}

impl fmt::Display for SmartSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Socket with power {} turned {}.",
            self.power,
            if self.on { "on" } else { "off" }
        )
    }
}
