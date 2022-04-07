pub mod structs {
    pub mod app {
        pub mod lights_main;
    }
    pub mod domain {
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
        pub mod collections_main;
    }
    // pub mod domain {}
    mod infrastructure {
        pub mod exercises;
        pub mod hashmap;
        pub mod string;
        pub mod vector;
    }
}

pub mod errors {
    pub mod app {
        pub mod errors_main;
    }
}

pub mod traits_generics {
    pub mod app {
        pub mod traits_main;
    }
    mod domain {
        pub mod structs {
            pub mod news_article;
            pub mod tweet;
            pub mod generics;
            pub mod lifetimes;
        }
        pub mod traits {
            pub mod summary;
        }
    }
    mod infrastructure {
        pub mod generics;
        pub mod lifetimes;
    }
}
