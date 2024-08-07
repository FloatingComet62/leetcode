use getopts::{Options, HasArg, Occur};
use std::env;
use colored::Colorize;
use leetcode::*;

const TESTS: [&str; 7] = [
    "two_sum",
    "add_two_numbers",
    "longest_substring_without_repeating_characters",
    "median_of_two_sorted_arrays",
    "longest_palindromic_substring",
    "roman_to_integer",
    "longest_common_prefix",
];

fn print_help(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}\nProgram not in the list\nThe List:\n", opts.usage(&brief));
    for i in 0..TESTS.len() {
        println!("{} {}", i, TESTS[i]);
    }
}

struct Tests {
    tracker: u32,
}

#[allow(dead_code)]
impl Tests {
    fn new() -> Self {
        Self { tracker: 1 }
    }
    fn add_test(&mut self, result: bool) {
        print!("Test {}: ", self.tracker);
        self.tracker += 1;
        if !result {
            println!("{}", "Failure".red());
            return;
        }
        println!("{}", "Success".green());
    }
    fn add_test_wlabel(&mut self, label: &str, result: bool) {
        print!("{}: ", label);
        self.tracker += 1;
        if !result {
            println!("Failure");
            return;
        }
        println!("Success");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
    let mut options = Options::new();
    options.opt("p", "program", "Program to test", "\"all\" / NAME / INDEX(number)", HasArg::Yes, Occur::Req);
    options.optflag("h", "help", "Print this help message");

    let matches = match options.parse(&args[1..]) {
        Ok(m) => { m },
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

fn test_program(program: &str) {
    println!("Program Running: {}", program);

    let mut test = Tests::new();
    if program == TESTS[0] {
        test.add_test(two_sum::solution(vec![2, 7, 11, 15], 9) == vec![0, 1]);
        test.add_test(two_sum::solution(vec![3, 2, 4], 6) == vec![1, 2]);
        test.add_test(two_sum::solution(vec![3, 3], 6) == vec![0, 1]);
    } else if program == TESTS[1] {
        test.add_test(
            add_two_numbers::solution(
                add_two_numbers::generate_nodes(vec![2, 4, 3]),
                add_two_numbers::generate_nodes(vec![5, 6, 4]),
            ) ==
            add_two_numbers::generate_nodes(vec![7, 0, 8]),
        );
        test.add_test(
            add_two_numbers::solution(
                add_two_numbers::generate_nodes(vec![0]),
                add_two_numbers::generate_nodes(vec![0]),
            ) ==
            add_two_numbers::generate_nodes(vec![0]),
        );
        test.add_test(
            add_two_numbers::solution(
                add_two_numbers::generate_nodes(vec![9, 9, 9, 9, 9, 9, 9]),
                add_two_numbers::generate_nodes(vec![9, 9, 9, 9]),
            ) ==
            add_two_numbers::generate_nodes(vec![8, 9, 9, 9, 0, 0, 0, 1]),
        );
    } else if program == TESTS[2] {
        test.add_test(
            longest_substring_without_repeating_characters::solution("abcabcbb".to_string())
            == 3
        );
        test.add_test(
            longest_substring_without_repeating_characters::solution("bbbbb".to_string())
            == 1
        );
        test.add_test(
            longest_substring_without_repeating_characters::solution("pwwkew".to_string())
            == 3
        );
    } else if program == TESTS[3] {
        test.add_test(
            median_of_two_sorted_arrays::solution(vec![1, 3], vec![2])
            == 2.0
        );
        test.add_test(
            median_of_two_sorted_arrays::solution(vec![1, 2], vec![3, 4])
            == 2.5
        );
    } else if program == TESTS[4] {
        test.add_test(longest_palindromic_substring::solution("babad".to_string()) == "bab");
        test.add_test(longest_palindromic_substring::solution("cbbd".to_string()) == "bb");
        test.add_test(longest_palindromic_substring::solution("a".to_string()) == "a");
        test.add_test(longest_palindromic_substring::solution("bb".to_string()) == "bb");
        test.add_test(longest_palindromic_substring::solution(
            "lejyqjcpluiggwlmnumqpxljlcwdsirzwlygexejhvojztcztectzrepsvwssiixfmpbzshpilmojehqyqpzdylxptsbvkgatzdlzphohntysrbrcdgeaiypmaaqilthipjbckkfbxtkreohabrjpmelxidlwdajmkndsdbbaypcemrwlhwbwaljacijjmsaqembgtdcskejplifnuztlmvasbqcyzmvczpkimpbbwxdtviptzaenkbddaauyvqppagvqfpednnckooxzcpuudckakutqyknuqrxjgfdtsxsoztjkqvfvelrklforpjnrbvyyvxigjhkjmxcphjzzilvbjbvwiwnnkbmboiqamgoimujtswdqesighoxsprhnsceshotakvmoxqkqjvbpqucvafiuqwmrlfjpjijbctfupywkbawquchbclgvhxbanybret"
        .to_string()) == "vbjbv");
        test.add_test(longest_palindromic_substring::solution(
            "kztakrekvefgchersuoiuatzlmwynzjhdqqftjcqmntoyckqfawikkdrnfgbwtdpbkymvwoumurjdzygyzsbmwzpcxcdmmpwzmeibligwiiqbecxwyxigikoewwrczkanwwqukszsbjukzumzladrvjefpegyicsgctdvldetuegxwihdtitqrdmygdrsweahfrepdcudvyvrggbkthztxwicyzazjyeztytwiyybqdsczozvtegodacdokczfmwqfmyuixbeeqluqcqwxpyrkpfcdosttzooykpvdykfxulttvvwnzftndvhsvpgrgdzsvfxdtzztdiswgwxzvbpsjlizlfrlgvlnwbjwbujafjaedivvgnbgwcdbzbdbprqrflfhahsvlcekeyqueyxjfetkxpapbeejoxwxlgepmxzowldsmqllpzeymakcshfzkvyykwljeltutdmrhxcbzizihzinywggzjctzasvefcxmhnusdvlderconvaisaetcdldeveeemhugipfzbhrwidcjpfrumshbdofchpgcsbkvaexfmenpsuodatxjavoszcitjewflejjmsuvyuyrkumednsfkbgvbqxfphfqeqozcnabmtedffvzwbgbzbfydiyaevoqtfmzxaujdydtjftapkpdhnbmrylcibzuqqynvnsihmyxdcrfftkuoymzoxpnashaderlosnkxbhamkkxfhwjsyehkmblhppbyspmcwuoguptliashefdklokjpggfiixozsrlwmeksmzdcvipgkwxwynzsvxnqtchgwwadqybkguscfyrbyxudzrxacoplmcqcsmkraimfwbauvytkxdnglwfuvehpxd"
        .to_string()) == "dtzztd");
    } else if program == TESTS[5] {
        test.add_test(roman_to_integer::solution("III".to_string()) == 3);
        test.add_test(roman_to_integer::solution("LVIII".to_string()) == 58);
        test.add_test(roman_to_integer::solution("MCMXCIV".to_string()) == 1994);
    } else if program == TESTS[6] {
        test.add_test(longest_common_prefix::solution(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ]) == "fl");
        test.add_test(longest_common_prefix::solution(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ]) == "");
    } else {
        println!("Program not in the list\nThe List:");
        for i in 0..TESTS.len() {
            println!("{} {}", i, TESTS[i]);
        }
        std::process::exit(1);
    }
}
