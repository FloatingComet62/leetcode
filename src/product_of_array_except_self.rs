/*
Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
You must write an algorithm that runs in O(n) time and without using the division operation.

Example 1:
Input: nums = [1,2,3,4]
Output: [24,12,8,6]

Example 2:
Input: nums = [-1,1,0,-3,3]
Output: [0,0,9,0,0]

Constraints:
    2 <= nums.length <= 105
    -30 <= nums[i] <= 30
    The input is generated such that answer[i] is guaranteed to fit in a 32-bit integer.

Follow up: Can you solve the problem in O(1) extra space complexity? (The output array does not count as extra space for space complexity analysis.)
*/

use crate::Tests;

pub fn solution(nums: Vec<i32>) -> Vec<i32> {
    let mut output = vec![1; nums.len()];

    for i in 1..nums.len() {
        output[i] = output[i - 1] * nums[i - 1];
    }

    let mut acc = 1;
    for i in (0..nums.len()).rev() {
        output[i] *= acc;
        acc *= nums[i];
    }

    output
}

pub fn test(test: &mut Tests) {
    test.add_test(solution(vec![1, 2, 3, 4]) == vec![24, 12, 8, 6]);
    test.add_test(solution(vec![-1, 1, 0, -3, 3]) == vec![0, 0, 9, 0, 0]);
}
