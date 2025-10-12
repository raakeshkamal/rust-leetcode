/*
485. Max Consecutive Ones
Easy
Topics
premium lock iconCompanies
Hint

Given a binary array nums, return the maximum number of consecutive 1's in the array.

Example 1:
Input: nums = [1,1,0,1,1,1]
Output: 3
Explanation: The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.

Example 2:
Input: nums = [1,0,1,1,0,1]
Output: 2

Constraints:
    1 <= nums.length <= 105
    nums[i] is either 0 or 1.
*/

use crate::Runnable; // Import the trait from the libray root

pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut curr_len = 0;
        for i in nums {
            if i == 1 {
                curr_len += 1;
            } else {
                curr_len = 0;
            }
            if curr_len > res {
                res = curr_len;
            }
        }
        res
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let res = Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]);
        println!("{:?}", res);
    }
}

/*
fill up the test-cases based on description and examples
dont need more than 3 test-cases
dont touch the solution and also dont add comments
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::find_max_consecutive_ones(vec![1]), 1);
    }
}
