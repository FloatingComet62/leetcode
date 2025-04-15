/*
Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-2^31, 2^31 - 1], then return 0.
Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

Example 1:

Input: x = 123
Output: 321

Example 2:

Input: x = -123
Output: -321

Example 3:

Input: x = 120
Output: 21

Constraints:

-2^31 <= x <= 2^31 - 1
*/
use crate::Tests;

pub fn solution(x: i32) -> i32 {
    x.signum()
        * match x
            .abs()
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse()
        {
            Ok(num) => num,
            Err(_) => 0,
        }
}

pub fn test(test: &mut Tests) {
    test.add_test(solution(123) == 321);
    test.add_test(solution(-123) == -321);
    test.add_test(solution(120) == 21);
}
