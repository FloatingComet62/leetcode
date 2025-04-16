pub mod add_two_numbers;
pub mod contains_duplicate;
pub mod group_anagrams;
pub mod longest_common_prefix;
pub mod longest_palindromic_substring;
pub mod longest_substring_without_repeating_characters;
pub mod median_of_two_sorted_arrays;
pub mod reverse_integer;
pub mod roman_to_integer;
pub mod string_to_integer_atoi;
pub mod top_k_frequent_elements;
pub mod two_sum;
pub mod valid_anagram;
pub mod zigzag_conversion;
pub mod product_of_array_except_self;

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
