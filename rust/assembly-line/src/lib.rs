// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use core::panic;

/// Calculates the hourly production rate at the given speed
pub fn production_rate_per_hour(speed: u8) -> f64 {
    total_cars_produced(speed) * percent_failed(speed)
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let per_minute = production_rate_per_hour(speed) / 60.0;
    per_minute.floor() as u32
}

/// Calculate the total number of cars that would be produced
/// at the given speed if there were no failures
fn total_cars_produced(speed: u8) -> f64 {
    f64::from(speed) * 221.0
}

/// Calculates the percent of failed items produced at the given speed
fn percent_failed(speed: u8) -> f64 {
    match speed {
        s if s >= 1 && s <= 4 => 1.0,
        s if s >= 5 && s <= 8 => 0.9,
        s if s == 9 || s == 10 => 0.77,
        s if s > 10 => panic!("Can't go faster than 10!"),
        _ => 0.0
    }
}

