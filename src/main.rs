// Gral App Modules
mod basics;

// Importa
use crate::basics::{
    structs::app::lights_main as structs, 
    collections::app::collections_main as collections,
    errors::app::errors_main as errors,
    traits_generics::app::traits_main as traits
};

// Entry Point
fn main() {
    structs::lights();
    collections::collections();
    errors::errors_examples();
    traits::traits_examples();
}
