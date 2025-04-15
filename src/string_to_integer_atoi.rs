/*
Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer.

The algorithm for myAtoi(string s) is as follows:

Whitespace: Ignore any leading whitespace (" ").
Signedness: Determine the sign by checking if the next character is '-' or '+', assuming positivity is neither present.
Conversion: Read the integer by skipping leading zeros until a non-digit character is encountered or the end of the string is reached. If no digits were read, then the result is 0.
Rounding: If the integer is out of the 32-bit signed integer range [-2^31, 2^31 - 1], then round the integer to remain in the range. Specifically, integers less than -2^31 should be rounded to -2^31, and integers greater than 2^31 - 1 should be rounded to 2^31 - 1.
Return the integer as the final result.

Example 1:

Input: s = "42"
Output: 42

Explanation:

The underlined characters are what is read in and the caret is the current reader position.
Step 1: "42" (no characters read because there is no leading whitespace)
         ^
Step 2: "42" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "42" ("42" is read in)
           ^
Example 2:

Input: s = " -042"
Output: -42

Explanation:

Step 1: "   -042" (leading whitespace is read and ignored)
            ^
Step 2: "   -042" ('-' is read, so the result should be negative)
             ^
Step 3: "   -042" ("042" is read in, leading zeros ignored in the result)
               ^
Example 3:

Input: s = "1337c0d3"
Output: 1337

Explanation:

Step 1: "1337c0d3" (no characters read because there is no leading whitespace)
         ^
Step 2: "1337c0d3" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "1337c0d3" ("1337" is read in; reading stops because the next character is a non-digit)
             ^
Example 4:

Input: s = "0-1"
Output: 0

Explanation:

Step 1: "0-1" (no characters read because there is no leading whitespace)
         ^
Step 2: "0-1" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "0-1" ("0" is read in; reading stops because the next character is a non-digit)
          ^
Example 5:

Input: s = "words and 987"
Output: 0

Explanation:

Reading stops at the first non-digit character 'w'.

Constraints:

0 <= s.length <= 200
s consists of English letters (lower-case and upper-case), digits (0-9), ' ', '+', '-', and '.'.
*/
use crate::Tests;

pub fn is_alphabet(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
}

pub fn solution(s: String) -> i32 {
    let mut sign = 2;
    let mut output = String::with_capacity(s.len());
    let mut processing_number = false;

    for c in s.chars() {
        if is_alphabet(c) {
            break;
        }
        if processing_number {
            if c == '-' || c == '+' || c == '.' || c == ' ' {
                break;
            }
        }
        if c == ' ' {
            continue;
        }
        if c == '-' {
            sign = -1;
            processing_number = true;
            continue;
        }
        if c == '+' {
            sign = 1;
            processing_number = true;
            continue;
        }

        processing_number = true;
        output.push(c);
    }

    if sign == 2 {
        sign = 1;
    }

    sign * match output.parse::<i32>() {
        Ok(num) => num,
        Err(e) => {
            if e.to_string() == "number too large to fit in target type" {
                if sign == 1 {
                    return i32::MAX;
                }
                return i32::MIN;
            }
            0
        }
    }
}

pub fn test(test: &mut Tests) {
    test.add_test(solution("42".to_string()) == 42);
    test.add_test(solution("1337c0d3".to_string()) == 1337);
    test.add_test(solution("0-1".to_string()) == 0);
    test.add_test(solution("words and 987".to_string()) == 0);
    test.add_test(solution("-91283472332".to_string()) == -2147483648);
    test.add_test(solution("91283472332".to_string()) == 2147483647);
    test.add_test(solution("3.14159".to_string()) == 3);
    test.add_test(solution("-+12".to_string()) == 0);
    test.add_test(solution("  -042".to_string()) == -42);
    test.add_test(solution("-5-".to_string()) == -5);
    test.add_test(solution("21474836++".to_string()) == 21474836);
    test.add_test(solution("+-12".to_string()) == 0);
    test.add_test(solution("   +0 123".to_string()) == 0);
}
