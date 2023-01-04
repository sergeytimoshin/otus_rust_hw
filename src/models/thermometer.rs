use std::fmt::{self};

pub struct SmartThermometer {
    pub name: String,
    pub temperature: i32,
}

impl SmartThermometer {
    pub fn new(name: &str, temperature: i32) -> SmartThermometer {
        SmartThermometer {
            name: name.to_string(),
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
impl fmt::Display for SmartThermometer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Thermometer's temperature is {}.", self.temperature)
    }
}
