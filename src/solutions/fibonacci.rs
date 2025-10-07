/*
509. Fibonacci Number
Easy
Topics
premium lock iconCompanies

The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. That is
F(0) = 0, F(1) = 1
F(n) = F(n - 1) + F(n - 2), for n > 1.
Given n, calculate F(n).

Example 1:
Input: n = 2
Output: 1
Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1

Example 2:
Input: n = 3
Output: 2
Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.

Example 3
Input: n = 4
Output: 3
Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.

Constraints:
    0 <= n <= 30
*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        let mut res = vec![0, 1];
        for i in 2..=n as usize {
            res.push(res[i - 1] + res[i - 2]);
        }
        res[n as usize]
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let res = Self::fib(10);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::fib(2), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::fib(3), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::fib(4), 3);
    }
}
