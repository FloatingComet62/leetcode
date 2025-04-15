/*
Given two strings s and t, return true if t is an anagram of s, and false otherwise.

Example 1:
Input: s = "anagram", t = "nagaram"
Output: true

Example 2:
Input: s = "rat", t = "car"
Output: false

Constraints:
    1 <= s.length, t.length <= 5 * 104
    s and t consist of lowercase English letters.

Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?
*/

use crate::Tests;

pub fn solution(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut chars: [u32; 26] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for c in s.chars() {
        chars[c as usize - 'a' as usize] += 1;
    }
    for t in t.chars() {
        if chars[t as usize - 'a' as usize] == 0 {
            return false;
        }
        chars[t as usize - 'a' as usize] -= 1;
    }
    true
}

pub fn test(test: &mut Tests) {
    test.add_test(solution("anagram".to_string(), "nagaram".to_string()) == true);
    test.add_test(solution("rat".to_string(), "car".to_string()) == false);
    test.add_test(solution("ab".to_string(), "a".to_string()) == false);
}
