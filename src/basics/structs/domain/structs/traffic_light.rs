use crate::basics::structs::domain::enums::traffic_light_color as color;
use crate::basics::structs::domain::traits::light;

#[derive(Debug)]
pub struct TrafficLight {
    color: color::TrafficLightColor,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: color::TrafficLightColor::Red,
        }
    }
    pub fn turn_green(&mut self) {
        self.color = color::TrafficLightColor::Green
    }
}

impl light::Light for TrafficLight {
    fn get_name(&self) -> &str {
        "Traffic light"
    }
    fn get_state(&self) -> &dyn std::fmt::Debug {
        &self.color
    }
}

impl std::fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Traffic light is {}", self.color)
    }
}
