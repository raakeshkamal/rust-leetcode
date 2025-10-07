/*
202. Happy Number
Easy
Topics
premium lock iconCompanies

Write an algorithm to determine if a number n is happy.
A happy number is a number defined by the following process:
    Starting with any positive integer, replace the number by the sum of the squares of its digits.
    Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
    Those numbers for which this process ends in 1 are happy.

Return true if n is a happy number, and false if not.

Example 1:
Input: n = 19
Output: true
Explanation:
12 + 92 = 82
82 + 22 = 68
62 + 82 = 100
12 + 02 + 02 = 1

Example 2:
Input: n = 2
Output: false

Constraints:
    1 <= n <= 231 - 1

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashMap;
        let mut hash_map:HashMap<i32,i32> = HashMap::new();
        let mut temp = n;
        loop {
            let mut res = 0;
            while temp > 0 {
                res += (temp % 10).pow(2);
                temp /= 10;
            }
            temp = res;
            if hash_map.contains_key(&temp) {
                break;
            }
            hash_map.insert(temp, 1);
            // println!("{:?}", temp);
        }
        temp == 1
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let val = 1;
        let res = Self::is_happy(val);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert!(Solution::is_happy(19));
    }

    #[test]
    fn test_case_2() {
        assert!(!Solution::is_happy(2));
    }

    #[test]
    fn test_case_3() {
        assert!(Solution::is_happy(1));
    }
}
