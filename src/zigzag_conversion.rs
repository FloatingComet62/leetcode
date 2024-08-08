/*
The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

P   A   H   N
A P L S I I G
Y   I   R

And then read line by line: "PAHNAPLSIIGYIR"
Write the code that will take a string and make this conversion given a number of rows:
string convert(string s, int numRows);

Example 1:

Input: s = "PAYPALISHIRING", numRows = 3
Output: "PAHNAPLSIIGYIR"

Example 2:

Input: s = "PAYPALISHIRING", numRows = 4
Output: "PINALSIGYAHRPI"
Explanation:
P     I    N
A   L S  I G
Y A   H R
P     I

Example 3:

Input: s = "A", numRows = 1
Output: "A"

Constraints:

1 <= s.length <= 1000
s consists of English letters (lower-case and upper-case), ',' and '.'.
1 <= numRows <= 1000
*/
use crate::Tests;
pub fn solution(s: String, num_rows: i32) -> String {
    if s.len() <= 2 || num_rows == 1 || num_rows >= s.len() as i32 {
        return s;
    }
    let mut row_strings = vec![String::new(); num_rows as usize];
    let mut iter: i32 = 0;
    let mut step: i8 = 1;
    let mut chars = s.chars();
    for _ in 0..s.len() {
        row_strings[iter as usize].push(chars.next().unwrap());
        iter += step as i32;
        if iter == num_rows - 1 || iter == 0 {
            step *= -1;
        }
    }
    row_strings.iter().map(|r| r.chars()).flatten().collect()
}

pub fn solution1(s: String, num_rows: i32) -> String {
    if s.len() <= 2 || num_rows == 1 || num_rows >= s.len() as i32 {
        return s;
    }
    let mut iter: i32 = 0;
    let mut step: i8 = 1;
    let mut rows = Vec::with_capacity(s.len());
    for i in 0..s.len() {
        rows.push((iter, i));
        iter += step as i32;
        if iter == num_rows - 1 || iter == 0 {
            step *= -1;
        }
    }
    rows.sort_by(|(iter1, _), (iter2, _)| iter1.partial_cmp(iter2).unwrap());
    let chars: Vec<char> = s.chars().collect();
    let mut output = String::with_capacity(s.len());
    for (_, i) in rows {
        output.push(chars[i]);
    }
    output
}

pub fn test(test: &mut Tests) {
    test.add_test(solution("PAYPALISHIRING".to_string(), 3) == "PAHNAPLSIIGYIR".to_string());
    test.add_test(solution("PAYPALISHIRING".to_string(), 4) == "PINALSIGYAHRPI".to_string());
    test.add_test(solution("A".to_string(), 1) == "A".to_string());
}
