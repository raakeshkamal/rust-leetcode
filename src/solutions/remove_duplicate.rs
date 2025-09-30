/*
26. Remove Duplicates from Sorted Array
Easy
Topics
Companies
Hint

Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.

Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

    Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
    Return k.

Custom Judge:

The judge will test your solution with the following code:

int[] nums = [...]; // Input array
int[] expectedNums = [...]; // The expected answer with correct length

int k = removeDuplicates(nums); // Calls your implementation

assert k == expectedNums.length;
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}

If all assertions pass, then your solution will be accepted.

 */

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut uniq_elem = nums[0];
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] > uniq_elem {
                k += 1;
                nums[k] = nums[i];
                uniq_elem = nums[i];
            }
        }
        k += 1;
        k as i32
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let res = Self::remove_duplicates(&mut nums1);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        assert_eq!(Solution::remove_duplicates(&mut nums1), 3);
    }

    #[test]
    fn test_case_2() {
        let mut nums1 = vec![1];
        assert_eq!(Solution::remove_duplicates(&mut nums1), 1);
    }

    #[test]
    fn test_case_3() {
        let mut nums1 = vec![0];
        assert_eq!(Solution::remove_duplicates(&mut nums1), 1);
    }
}
