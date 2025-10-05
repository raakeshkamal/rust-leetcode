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
    pub fn title_to_number(column_title: String) -> i32 {
        let mut res = 0;
        for ch in column_title.chars() {
            res *= 26;
            res += ch as u32 - 64;
        }
        res as i32
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let input = "Hello World".to_string();
        let res = Self::title_to_number(input);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
    }
}
