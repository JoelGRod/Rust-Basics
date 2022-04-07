use crate::basics::main::structs::domain::structs::{traffic_light as traffic, house_light as home};
use crate::basics::main::structs::domain::traits::light::{self};

pub fn lights() {
    let traffic_light = traffic::TrafficLight::new();
    let house_light = home::HouseLight::new();
    print_state(&traffic_light);
    print_state(&house_light);
}

fn print_state(light: &impl light::Light) {
    println!("{}'s state is : {:?}", light.get_name(), light.get_state());
}