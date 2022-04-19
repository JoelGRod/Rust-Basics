use std::thread;
use std::time::Duration;

pub fn simulate_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating expensive task...");
    thread::sleep(Duration::from_secs(2));
    intensity
}