/*
507. Perfect Number
Easy
Topics
premium lock iconCompanies

A perfect number is a positive integer that is equal to the sum of its positive divisors, excluding the number itself. A divisor of an integer x is an integer that can divide x evenly.
Given an integer n, return true if n is a perfect number, otherwise return false.

Example 1:
Input: num = 28
Output: true
Explanation: 28 = 1 + 2 + 4 + 7 + 14
1, 2, 4, 7, and 14 are all divisors of 28.

Example 2:
Input: num = 7
Output: false

Constraints:
    1 <= num <= 108
*/

use crate::Runnable; // Import the trait from the libray root

pub struct Solution;

impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        let max = num / 2;
        let mut res = 0;
        for i in 1..=max {
            if num % i == 0 {
                res += i;
            }
        }
        res == num
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let res = Solution::check_perfect_number(28);
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
        assert_eq!(Solution::check_perfect_number(28), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::check_perfect_number(7), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::check_perfect_number(6), true);
    }
}
