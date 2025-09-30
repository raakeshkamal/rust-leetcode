/*
58. Length of Last Word
Easy
Topics
Companies

Given a string s consisting of words and spaces, return the length of the last word in the string.

A word is a maximal
substring
consisting of non-space characters only.



Example 1:

Input: s = "Hello World"
Output: 5
Explanation: The last word is "World" with length 5.

Example 2:

Input: s = "   fly me   to   the moon  "
Output: 4
Explanation: The last word is "moon" with length 4.

Example 3:

Input: s = "luffy is still joyboy"
Output: 6
Explanation: The last word is "joyboy" with length 6.



Constraints:

    1 <= s.length <= 104
    s consists of only English letters and spaces ' '.
    There will be at least one word in s.

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut k: usize = 0;
        for i in 0..s.len() {
            if i > 0 && s.chars().nth(i - 1).unwrap() == ' ' && s.chars().nth(i).unwrap() != ' ' {
                k = 0;
            }
            if s.chars().nth(i).unwrap() != ' ' {
                k += 1;
            }
        }
        k as i32
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let s = "Hello World".to_string();
        let res = Self::length_of_last_word(s);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }
}
