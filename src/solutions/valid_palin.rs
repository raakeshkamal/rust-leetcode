/*
125. Valid Palindrome
Easy
Topics
Companies
A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.



Example 1:

Input: s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" is a palindrome.
Example 2:

Input: s = "race a car"
Output: false
Explanation: "raceacar" is not a palindrome.
Example 3:

Input: s = " "
Output: true
Explanation: s is an empty string "" after removing non-alphanumeric characters.
Since an empty string reads the same forward and backward, it is a palindrome.


Constraints:

1 <= s.length <= 2 * 105
s consists only of printable ASCII characters.
*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let byte_str: Vec<char> = s.chars().collect();
        let mut final_str: Vec<char> = Vec::new();
        for i in byte_str {
            if i.is_alphanumeric() {
                final_str.push(i.to_ascii_lowercase());
            }
        }
        let str_len = final_str.len();
        for i in 0..str_len {
            let rev_i = str_len - 1 - i;
            if final_str[i] != final_str[rev_i] {
                return false;
            }
            if i > rev_i {
                break;
            }
        }
        true
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let res = Self::is_palindrome("A man, a plan, a canal: Panama".to_string());
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
    }
}
