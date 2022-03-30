use crate::lights::structs::{house_light as home, traffic_light as traffic};

mod lights {
    pub mod structs {
        pub mod house_light;
        pub mod traffic_light;
    }
    pub mod enums {
        pub mod traffic_light_color;
    }
    pub mod traits {
        pub mod light;
    }
}

mod collections {
    pub mod vector;
    pub mod string;
    pub mod hashmap;
}

use crate::lights::traits::light;

use crate::collections::{vector, string, hashmap};

fn main() {
    let traffic_light = traffic::TrafficLight::new();
    let house_light = home::HouseLight::new();
    print_state(&traffic_light);
    print_state(&house_light);

    println!("---------- Vectors ----------");
    vector::vector_examples();

    println!("---------- Strings ----------");
    string::string_examples();

}

fn print_state(light: &impl light::Light) {
    println!("{}'s state is : {:?}", light.get_name(), light.get_state());
}



















