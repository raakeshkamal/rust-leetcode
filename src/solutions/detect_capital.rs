/*
520. Detect Capital
Easy
Topics
premium lock iconCompanies

We define the usage of capitals in a word to be right when one of the following cases holds:
    All letters in this word are capitals, like "USA".
    All letters in this word are not capitals, like "leetcode".
    Only the first letter in this word is capital, like "Google".
Given a string word, return true if the usage of capitals in it is right.

Example 1:
Input: word = "USA"
Output: true

Example 2:
Input: word = "FlaG"
Output: false

Constraints:
    1 <= word.length <= 100
    word consists of lowercase and uppercase English letters.

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut first_char_up = true;
        let mut all_other_char_up = true;
        let mut any_other_char_up = false;
        for (idx, ch) in word.chars().enumerate() {
            if ch.is_ascii_lowercase() {
                if idx == 0 {
                    first_char_up = false;
                } else {
                    all_other_char_up = false;
                }
            } else if ch.is_ascii_uppercase() && idx > 0 {
                any_other_char_up = true;
            }
        }
        (first_char_up && all_other_char_up) || !any_other_char_up
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let res = Self::detect_capital_use(String::from("abcd"));
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::detect_capital_use(String::from("USA")), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::detect_capital_use(String::from("FlaG")), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::detect_capital_use(String::from("leetcode")), true);
    }
}
