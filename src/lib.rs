// Gral App Modules
mod basics;

// Importa
use crate::basics::{
    structs::app::lights_main as structs, 
    collections::app::collections_main as collections,
    errors::app::errors_main as errors,
    traits_generics::app::traits_main as traits
};

pub fn structs_example() -> bool {
    structs::lights();
    true
}

pub fn collections_example() -> bool {
    collections::collections();
    true
}

pub fn errors_example() -> bool {
    errors::errors_examples();
    true
}

pub fn traits_example() -> bool {
    traits::traits_examples();
    true
}