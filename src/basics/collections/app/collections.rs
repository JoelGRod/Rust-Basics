use crate::basics::collections::infrastructure::{vector, string, hashmap};

pub fn collections() {
    println!("---------- Vectors ----------");
    vector::vector_examples();

    println!("---------- Strings ----------");
    string::string_examples();
    
    println!("---------- HashMaps ----------");
    hashmap::hashmap_examples();
}