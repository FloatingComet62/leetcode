/*
Given a string s, find the length of the longest substring without repeating characters.

Example 1:

Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.
 
Example 2:

Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.
 
Example 3:

Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

Constraints:

0 <= s.length <= 5 * 104
s consists of English letters, digits, symbols and spaces.
*/
use std::collections::HashMap;

pub fn solution(s: String) -> i32 {
    let mut map_of_chars: HashMap<char, usize> = HashMap::new();
    let mut substr_len = 0;
    let mut max_substr_len = 0;
    let mut i = 0;
    let mut start: usize = 0;
    while i < s.len() {
        if start > max_substr_len * 10 {
            return max_substr_len as i32;
        }
        let character = s.chars().nth(i).unwrap();
        
        if map_of_chars.contains_key(&character) {
            map_of_chars.clear();
            if max_substr_len < substr_len {
                max_substr_len = substr_len;
            }
            substr_len = 0;
            start += 1;
            i = start;
            continue;
        }
        substr_len += 1;
        map_of_chars.insert(character, i);
        i += 1;
    }
    map_of_chars.clear();
    if max_substr_len < substr_len {
        max_substr_len = substr_len;
    }
    max_substr_len as i32
}
