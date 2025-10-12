/*
496. Next Greater Element I
Easy
Topics
premium lock iconCompanies

The next greater element of some element x in an array is the first greater element that is to the right of x in the same array.
You are given two distinct 0-indexed integer arrays nums1 and nums2, where nums1 is a subset of nums2.
For each 0 <= i < nums1.length, find the index j such that nums1[i] == nums2[j] and determine the next greater element of nums2[j] in nums2. If there is no next greater element, then the answer for this query is -1.
Return an array ans of length nums1.length such that ans[i] is the next greater element as described above.

Example 1:
Input: nums1 = [4,1,2], nums2 = [1,3,4,2]
Output: [-1,3,-1]
Explanation: The next greater element for each value of nums1 is as follows:
- 4 is underlined in nums2 = [1,3,4,2]. There is no next greater element, so the answer is -1.
- 1 is underlined in nums2 = [1,3,4,2]. The next greater element is 3.
- 2 is underlined in nums2 = [1,3,4,2]. There is no next greater element, so the answer is -1.

Example 2:
Input: nums1 = [2,4], nums2 = [1,2,3,4]
Output: [3,-1]
Explanation: The next greater element for each value of nums1 is as follows:
- 2 is underlined in nums2 = [1,2,3,4]. The next greater element is 3.
- 4 is underlined in nums2 = [1,2,3,4]. There is no next greater element, so the answer is -1.

Constraints:
    1 <= nums1.length <= nums2.length <= 1000
    0 <= nums1[i], nums2[i] <= 104
    All integers in nums1 and nums2 are unique.
    All the integers of nums1 also appear in nums2.

Follow up: Could you find an O(nums1.length + nums2.length) solution?
*/

use std::vec;

use crate::Runnable; // Import the trait from the libray root

pub struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        for i in nums1 {
            let pos = nums2.iter().position(|&x| x == i);
            let mut idx = nums2.len(); // this is invalid index

            if let Some(temp) = pos {
                idx = temp;
            }

            let mut next_great = idx;

            if idx < nums2.len() {
                for j in idx..nums2.len() {
                    if nums2[j] > i {
                        next_great = j;
                        break;
                    }
                }
            }

            if next_great > idx {
                res.push(nums2[next_great]);
            } else {
                res.push(-1);
            }
        }
        res
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let res = Self::next_greater_element(nums1, nums2);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums1 = vec![4, 1, 2];
        let nums2 = vec![1, 3, 4, 2];
        let expected = vec![-1, 3, -1];
        assert_eq!(Solution::next_greater_element(nums1, nums2), expected);
    }

    #[test]
    fn test_case_2() {
        let nums1 = vec![2, 4];
        let nums2 = vec![1, 2, 3, 4];
        let expected = vec![3, -1];
        assert_eq!(Solution::next_greater_element(nums1, nums2), expected);
    }

    #[test]
    fn test_case_3() {
        let nums1 = vec![3, 1];
        let nums2 = vec![3, 4, 1, 2];
        let expected = vec![4, 2];
        assert_eq!(Solution::next_greater_element(nums1, nums2), expected);
    }
}
