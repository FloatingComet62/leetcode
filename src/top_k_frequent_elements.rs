/*
Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.

Example 1:
Input: nums = [1,1,1,2,2,3], k = 2
Output: [1,2]

Example 2:
Input: nums = [1], k = 1
Output: [1]

Constraints:
    1 <= nums.length <= 105
    -104 <= nums[i] <= 104
    k is in the range [1, the number of unique elements in the array].
    It is guaranteed that the answer is unique.

Follow up: Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
*/

use crate::Tests;
use std::collections::{HashMap, binary_heap::BinaryHeap};

pub fn solution(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for num in nums {
        map.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut heap = BinaryHeap::new();
    for (key, value) in map.iter() {
        heap.push((*value, *key));
    }
    let mut output_vec = Vec::with_capacity(k as usize);
    for _ in 0..k {
        output_vec.push(heap.pop().unwrap().1);
    }
    output_vec
}

pub fn test(test: &mut Tests) {
    test.add_test(solution(vec![1, 1, 1, 2, 2, 3], 2) == vec![1, 2]);
    test.add_test(solution(vec![1], 1) == vec![1]);
}
