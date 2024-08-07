/*
Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
The overall run time complexity should be O(log (m+n)).

Example 1:

Input: nums1 = [1,3], nums2 = [2]
Output: 2.00000
Explanation: merged array = [1,2,3] and median is 2.
 
Example 2:

Input: nums1 = [1,2], nums2 = [3,4]
Output: 2.50000
Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.

Constraints:

nums1.length == m
nums2.length == n
0 <= m <= 1000
0 <= n <= 1000
1 <= m + n <= 2000
-106 <= nums1[i], nums2[i] <= 106
*/
pub fn solution(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut vec = nums1.clone();
    vec.append(&mut nums2.clone());
    vec.sort();
    if vec.len() % 2 == 1 {
        return vec[(vec.len()-1)/2 as usize] as f64;
    }
    f64::from(vec[vec.len()/2] + vec[vec.len()/2 -1])/2.0
}
