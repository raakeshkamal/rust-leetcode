/*
35. Search Insert Position
Easy
Topics
Companies

Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You must write an algorithm with O(log n) runtime complexity.



Example 1:

Input: nums = [1,3,5,6], target = 5
Output: 2

Example 2:

Input: nums = [1,3,5,6], target = 2
Output: 1

Example 3:

Input: nums = [1,3,5,6], target = 7
Output: 4



Constraints:

    1 <= nums.length <= 104
    -104 <= nums[i] <= 104
    nums contains distinct values sorted in ascending order.
    -104 <= target <= 104


*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut start: usize = 0;
        let mut end: usize = nums.len() - 1;
        let mut pos: usize = (start + end) / 2;
        if target < nums[0] {
            return 0;
        }
        if target > nums[end] {
            return nums.len() as i32;
        }
        while start < end {
            if nums[pos] == target {
                return pos as i32;
            } else if target < nums[pos] {
                end = pos - 1;
            } else if nums[pos] < target {
                start = pos + 1;
            }
            pos = (start + end) / 2;
        }
        if target > nums[pos] {
            pos += 1;
        }
        pos as i32
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let nums1 = vec![1,3,5,6];
        let val = 5;
        let res = Self::search_insert(nums1, val);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums1 = vec![1,3,5,6];
        assert_eq!(Solution::search_insert(nums1, 5), 2);
    }

    #[test]
    fn test_case_2() {
        let nums1 = vec![1,3,5,6];
        assert_eq!(Solution::search_insert(nums1, 2), 1);
    }

    #[test]
    fn test_case_3() {
        let nums1 = vec![1,3,5,6];
        assert_eq!(Solution::search_insert(nums1, 7), 4);
    }
}
