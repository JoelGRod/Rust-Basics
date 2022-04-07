// Gral App Modules
mod basics;

// Importa
use crate::basics::{
    structs::app::lights_main as structs, 
    collections::app::collections_main as collections,
    errors::app::errors_main as errors
};

// Entry Point
fn main() {
    structs::lights();
    collections::collections();
    errors::errors_examples();
}
