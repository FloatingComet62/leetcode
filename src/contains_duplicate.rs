/*
Given an integer array nums, return true if any value appears more than once in the array, otherwise return false.

Example 1:
Input: nums = [1, 2, 3, 3]
Output: true

Example 2:
Input: nums = [1, 2, 3, 4]
Output: false
*/
use crate::Tests;
use std::collections::HashSet;

pub fn solution(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for num in nums {
        if set.contains(&num) {
            return true;
        }
        set.insert(num);
    }
    false
}

pub fn test(test: &mut Tests) {
    test.add_test(solution(vec![1, 2, 3, 3]) == true);
    test.add_test(solution(vec![1, 2, 3, 4]) == false);
}
