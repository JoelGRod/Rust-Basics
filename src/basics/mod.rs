pub mod structs {
    pub mod app {
        pub mod lights;
    }
    mod domain {
        pub mod structs {
            pub mod house_light;
            pub mod traffic_light;
        }
        mod enums {
            pub mod traffic_light_color;
        }
        pub mod traits {
            pub mod light;
        }
    }
    // mod infrastructure {}
}

pub mod collections {
    pub mod app {
        pub mod collections;
    }
    // pub mod domain {}
    mod infrastructure {
        pub mod exercises;
        pub mod hashmap;
        pub mod string;
        pub mod vector;
    }
}
