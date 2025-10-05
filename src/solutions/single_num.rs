/*
136. Single Number
Easy
Topics
premium lock iconCompanies
Hint

Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
You must implement a solution with a linear runtime complexity and use only constant extra space.

Example 1:
Input: nums = [2,2,1]
Output: 1

Example 2:
Input: nums = [4,1,2,1,2]
Output: 4

Example 3:
Input: nums = [1]
Output: 1

Constraints:
    1 <= nums.length <= 3 * 104
    -3 * 104 <= nums[i] <= 3 * 104
    Each element in the array appears twice except for one element which appears only once.

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        for i in nums {
            let value = hash_map.entry(i).or_insert(0);
            *value += 1;
        }
        for (key, value) in hash_map.iter() {
            if *value == 1 {
                return *key;
            }
        }
        0
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let input = vec![2, 2, 1];
        let res = Self::single_number(input);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::single_number(vec![9]), 9);
    }
}
