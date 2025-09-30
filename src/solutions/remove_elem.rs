/*
27. Remove Element
Easy
Topics
Companies
Hint

Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The order of the elements may be changed. Then return the number of elements in nums which are not equal to val.

Consider the number of elements in nums which are not equal to val be k, to get accepted, you need to do the following things:

    Change the array nums such that the first k elements of nums contain the elements which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
    Return k.

Custom Judge:

The judge will test your solution with the following code:

int[] nums = [...]; // Input array
int val = ...; // Value to remove
int[] expectedNums = [...]; // The expected answer with correct length.
                            // It is sorted with no values equaling val.

int k = removeElement(nums, val); // Calls your implementation

assert k == expectedNums.length;
sort(nums, 0, k); // Sort the first k elements of nums
for (int i = 0; i < actualLength; i++) {
    assert nums[i] == expectedNums[i];
}

If all assertions pass, then your solution will be accepted.

 */

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[k] = nums[i];
                k += 1;
            }
        }
        k as i32
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let val = 2;
        let res = Self::remove_element(&mut nums1, val);
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
        assert_eq!(Solution::remove_element(&mut nums1, 2), 5);
    }

    #[test]
    fn test_case_2() {
        let mut nums1 = vec![1];
        assert_eq!(Solution::remove_element(&mut nums1, 1), 0);
    }

    #[test]
    fn test_case_3() {
        let mut nums1 = vec![0];
        assert_eq!(Solution::remove_element(&mut nums1, 1), 1);
    }
}
