use std::fmt::{self};

pub struct Thermo {
    pub temperature: i32,
}

impl Thermo {
    pub fn new(temperature: i32) -> Thermo {
        Thermo {
            temperature,
        }
    }

    #[allow(dead_code)]
    fn set_temperature(&mut self, temperature: i32) {
        self.temperature = temperature;
    }

    #[allow(dead_code)]
    fn get_temperature(&self) {
        println!("{}", self);
    }
}
impl fmt::Display for Thermo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VALUE: {}", self.temperature)
    }
}