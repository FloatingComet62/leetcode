/*
Given an array of strings strs, group the anagrams together. You can return the answer in any order.

Example 1:
Input: strs = ["eat","tea","tan","ate","nat","bat"]
Output: [["bat"],["nat","tan"],["ate","eat","tea"]]

Explanation:
    There is no string in strs that can be rearranged to form "bat".
    The strings "nat" and "tan" are anagrams as they can be rearranged to form each other.
    The strings "ate", "eat", and "tea" are anagrams as they can be rearranged to form each other.

Example 2:
Input: strs = [""]
Output: [[""]]

Example 3:
Input: strs = ["a"]
Output: [["a"]]

Constraints:
    1 <= strs.length <= 104
    0 <= strs[i].length <= 100
    strs[i] consists of lowercase English letters.
*/

use crate::Tests;
use std::collections::HashMap;

pub fn anagram_set(s: &String) -> [u32; 26] {
    let mut chars: [u32; 26] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    for c in s.chars() {
        chars[c as usize - 'a' as usize] += 1;
    }
    return chars;
}
pub fn solution(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut maps: HashMap<[u32; 26], Vec<String>> = HashMap::new();
    for str in strs {
        maps
            .entry(anagram_set(&str))
            .and_modify(|vec| vec.push(str.to_string()))
            .or_insert(vec![str.to_string()]);
    }
    let mut output = vec![];
    for (_, value) in maps.iter() {
        output.push(value.to_vec());
    }
    output
}

pub fn test(test: &mut Tests) {
    // This is correct
    test.add_test(solution(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()
    ]) == vec![
        vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
        vec!["tan".to_string(), "nat".to_string()],
        vec!["bat".to_string()],
    ]);

    test.add_test(solution(vec![
            "".to_string(),
    ]) == vec![
        vec!["".to_string()],
    ]);
    test.add_test(solution(vec![
            "a".to_string(),
    ]) == vec![
        vec!["a".to_string()],
    ]);
}
