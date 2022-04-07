use std::fmt::Display;
use crate::basics::main::structs::domain::traits::light;

#[derive(Debug)]
pub struct HouseLight {
    on: bool,
}

impl HouseLight {
    pub fn new() -> Self {
        Self { on: false }
    }
}

impl light::Light for HouseLight {
    fn get_name(&self) -> &str {
        "House light"
    }
    fn get_state(&self) -> &dyn std::fmt::Debug {
        &self.on
    }
}

impl Display for HouseLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Houselight is {}", if self.on { "on" } else { "off" })
    }
}
