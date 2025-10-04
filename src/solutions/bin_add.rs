/*
67. Add Binary
Easy
Topics
premium lock iconCompanies

Given two binary strings a and b, return their sum as a binary string.

Example 1:
Input: a = "11", b = "1"
Output: "100"

Example 2:
Input: a = "1010", b = "1011"
Output: "10101"

Constraints:

    1 <= a.length, b.length <= 104
    a and b consist only of '0' or '1' characters.
    Each string does not contain leading zeros except for the zero itself.


*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (max_str, min_str, diff) = if a.len() > b.len() {
            (a.clone(), b.clone(), a.len() - b.len())
        } else {
            (b.clone(), a.clone(), b.len() - a.len())
        };
        let min_str = "0".repeat(diff) + min_str.as_str();
        let max_str: Vec<char> = max_str.chars().collect();
        let min_str: Vec<char> = min_str.chars().collect();

        let mut temp = "".to_string();
        let mut carry = false;
        for i in (0..max_str.len()).rev() {
            let max_char = *max_str.get(i).unwrap();
            let min_char = *min_str.get(i).unwrap();
            if max_char == '1' && min_char == '1' {
                if carry {
                    temp.push('1');
                } else {
                    temp.push('0');
                }
                carry = true;
            } else if max_char == '1' || min_char == '1' {
                if carry {
                    temp.push('0');
                } else {
                    temp.push('1');
                    carry = false;
                }
            } else {
                if carry {
                    temp.push('1');
                } else {
                    temp.push('0');
                }
                carry = false;
            }
        }
        if carry {
            temp.push('1');
        }
        let res: String = temp.chars().rev().collect();
        res
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let a = String::from("1010");
        let b = String::from("1011");
        let res = Self::add_binary(a, b);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let a = String::from("11");
        let b = String::from("1");
        assert_eq!(Solution::add_binary(a, b), String::from("100"));
    }

    #[test]
    fn test_case_2() {
        let a = String::from("1010");
        let b = String::from("1011");
        assert_eq!(Solution::add_binary(a, b), String::from("10101"));
    }

    #[test]
    fn test_case_3() {
        let a = String::from("111");
        let b = String::from("111");
        assert_eq!(Solution::add_binary(a, b), String::from("1110"));
    }
}
