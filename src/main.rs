use getopts::{HasArg, Occur, Options};
use leetcode::*;
use std::env;

const TESTS: [&str; 14] = [
    "two_sum",
    "add_two_numbers",
    "longest_substring_without_repeating_characters",
    "median_of_two_sorted_arrays",
    "longest_palindromic_substring",
    "roman_to_integer",
    "longest_common_prefix",
    "zigzag_conversion",
    "reverse_integer",
    "string_to_integer_atoi",
    "contains_duplicate",
    "valid_anagram",
    "group_anagram",
    "top_k_frequent_elements",
];
fn test_program(program: &str) {
    println!("Program Running: {}", program);

    let mut test = Tests::new();
    if program == TESTS[0] {
        two_sum::test(&mut test);
    } else if program == TESTS[1] {
        add_two_numbers::test(&mut test);
    } else if program == TESTS[2] {
        longest_substring_without_repeating_characters::test(&mut test);
    } else if program == TESTS[3] {
        median_of_two_sorted_arrays::test(&mut test);
    } else if program == TESTS[4] {
        longest_substring_without_repeating_characters::test(&mut test);
    } else if program == TESTS[5] {
        roman_to_integer::test(&mut test);
    } else if program == TESTS[6] {
        longest_common_prefix::test(&mut test);
    } else if program == TESTS[7] {
        zigzag_conversion::test(&mut test);
    } else if program == TESTS[8] {
        reverse_integer::test(&mut test);
    } else if program == TESTS[9] {
        string_to_integer_atoi::test(&mut test);
    } else if program == TESTS[10] {
        contains_duplicate::test(&mut test);
    } else if program == TESTS[11] {
        valid_anagram::test(&mut test);
    } else if program == TESTS[12] {
        group_anagrams::test(&mut test);
    } else if program == TESTS[13] {
        top_k_frequent_elements::test(&mut test);
    } else {
        println!("Program not in the list\nThe List:");
        for i in 0..TESTS.len() {
            println!("{} {}", i, TESTS[i]);
        }
        std::process::exit(1);
    }
}

fn print_help(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!(
        "{}\nProgram not in the list\nThe List:\n",
        opts.usage(&brief)
    );
    for i in 0..TESTS.len() {
        println!("{} {}", i, TESTS[i]);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut options = Options::new();
    options.opt(
        "p",
        "program",
        "Program to test",
        "\"all\" / NAME / INDEX(number)",
        HasArg::Yes,
        Occur::Req,
    );
    options.optflag("h", "help", "Print this help message");

    let matches = match options.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            print_help(&program, &options);
            std::process::exit(1);
        }
    };

    if matches.opt_present("h") {
        print_help(&program, &options);
    }

    let program_or_index = matches.opt_str("p").unwrap_or_else(|| {
        print_help(&program, &options);
        std::process::exit(1);
    });
    if program_or_index == "all" {
        for i in 0..TESTS.len() {
            test_program(TESTS[i]);
        }
        std::process::exit(0);
    }

    let maybe_index = program_or_index.parse::<usize>();
    match maybe_index {
        Ok(index) => {
            if index >= TESTS.len() {
                println!("Index too large");
                std::process::exit(1);
            }
            test_program(TESTS[index]);
        }
        Err(_) => {
            test_program(program_or_index.as_str());
        }
    }
}
