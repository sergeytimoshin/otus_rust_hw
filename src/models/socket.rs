use std::fmt::{self};

pub struct Socket {
    pub on: bool,
    pub power: i32,
}

impl Socket {
    pub fn new(on: bool, power: i32) -> Socket {
        Socket { on, power }
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

impl fmt::Display for Socket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "POWER: {}, ON: {}",
            self.power,
            if self.on { "on" } else { "off" }
        )
    }
}
