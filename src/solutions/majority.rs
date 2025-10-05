/*
169. Majority Element
Easy
Topics
premium lock iconCompanies

Given an array nums of size n, return the majority element.
The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.

Example 1:
Input: nums = [3,2,3]
Output: 3

Example 2:
Input: nums = [2,2,1,1,1,2,2]
Output: 2

Constraints:
    n == nums.length
    1 <= n <= 5 * 104
    -109 <= nums[i] <= 109

Follow-up: Could you solve the problem in linear time and in O(1) space?
*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let max_count = (nums.len() / 2) as u32;
        let mut hash_map: HashMap<i32, u32> = HashMap::new();

        for i in nums {
            let value = hash_map.entry(i).or_insert(0);
            *value += 1;
            if *value > max_count {
                return i;
            }
        }
        0
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let nums1 = vec![3, 2, 3];
        let res = Self::majority_element(nums1);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums1 = vec![3, 2, 3];
        assert_eq!(Solution::majority_element(nums1), 3);
    }

    #[test]
    fn test_case_2() {
        let nums1 = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(Solution::majority_element(nums1), 2);
    }

    #[test]
    fn test_case_3() {
        let nums1 = vec![3, 2, 3, 3];
        assert_eq!(Solution::majority_element(nums1), 3);
    }
}
