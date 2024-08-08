pub mod two_sum;
pub mod add_two_numbers;
pub mod longest_substring_without_repeating_characters;
pub mod median_of_two_sorted_arrays;
pub mod longest_palindromic_substring;
pub mod roman_to_integer;
pub mod longest_common_prefix;
pub mod zigzag_conversion;
pub mod reverse_integer;

use colored::Colorize;

pub struct Tests {
    tracker: u32,
}

impl Tests {
    pub fn new() -> Self {
        Self { tracker: 1 }
    }
    pub fn add_test(&mut self, result: bool) {
        print!("Test {}: ", self.tracker);
        self.tracker += 1;
        if !result {
            println!("{}", "Failure".red());
            return;
        }
        println!("{}", "Success".green());
    }
    pub fn add_test_wlabel(&mut self, label: &str, result: bool) {
        print!("{}: ", label);
        self.tracker += 1;
        if !result {
            println!("Failure");
            return;
        }
        println!("Success");
    }
}
