// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use core::num;

/// Returns the expected amount of time the lasagna should be in the oven.
pub fn expected_minutes_in_oven() -> i32 {
    40
}

/// Takes the actual minutes the lasagna has been in the oven as a parameter and 
/// returns how many minutes the lasagna still has to remain in the oven, 
/// based on the expected oven time in minutes from the previous task.
pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    expected_minutes_in_oven() - actual_minutes_in_oven
}

/// Takes the number of layers you added to the lasagna as a parameter and 
/// returns how many minutes you spent preparing the lasagna, 
/// assuming each layer takes you 2 minutes to prepare.
pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * 2
}

/// Takes two parameters: the first parameter is the number of layers you added to the lasagna, 
/// and the second parameter is the number of minutes the lasagna has been in the oven. 
/// Returns how many minutes you've worked on cooking the lasagna, 
/// which is the sum of the preparation time in minutes, and the time in minutes the lasagna 
/// has spent in the oven at the moment.
pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}
