/*
191. Number of 1 Bits
Easy
Topics
premium lock iconCompanies

Given a positive integer n, write a function that returns the number of
in its binary representation (also known as the Hamming weight).

Example 1:
Input: n = 11
Output: 3
Explanation:
The input binary string 1011 has a total of three set bits.

Example 2:
Input: n = 128
Output: 1
Explanation:
The input binary string 10000000 has a total of one set bit.

Example 3:
Input: n = 2147483645
Output: 30

Explanation:
The input binary string 1111111111111111111111111111101 has a total of thirty set bits.

Constraints:
    1 <= n <= 231 - 1
 
Follow up: If this function is called many times, how would you optimize it?

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut temp = n;
        let mut res = 0;
        while temp > 0 {
            res += temp%2;
            temp /= 2;
        }
        res
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let val = 4;
        let res = Self::hamming_weight(val);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::hamming_weight(11), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::hamming_weight(128), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::hamming_weight(2147483645), 30);
    }
}
