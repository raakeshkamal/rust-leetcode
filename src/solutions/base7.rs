/*
504. Base 7
Easy
Topics
premium lock iconCompanies

Given an integer num, return a string of its base 7 representation.

Example 1:
Input: num = 100
Output: "202"

Example 2:
Input: num = -7
Output: "-10"

Constraints:
    -107 <= num <= 107

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let mut temp = num.abs();
        let mut res = String::new();
        while temp > 0 {
            res = format!("{:?}", temp % 7) + &res;
            temp /= 7;
        }
        if num < 0 {
            res = "-".to_string() + &res;
        } else if num == 0 {
            res = "0".to_string();
        }
        res
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let res = Self::convert_to_base7(100);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::convert_to_base7(103), "205");
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::convert_to_base7(-7), "-10");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::convert_to_base7(0), "0");
    }
}
