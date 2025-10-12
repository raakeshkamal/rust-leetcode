/*
557. Reverse Words in a String III
Easy
Topics
premium lock iconCompanies

Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.

Example 1:
Input: s = "Let's take LeetCode contest"
Output: "s'teL ekat edoCteeL tsetnoc"

Example 2:
Input: s = "Mr Ding"
Output: "rM gniD"

Constraints:
    1 <= s.length <= 5 * 104
    s contains printable ASCII characters.
    s does not contain any leading or trailing spaces.
    There is at least one word in s.
    All the words in s are separated by a single space.

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut res = "".to_string();
        let mut last_pos = 0;
        for (pos, _) in s.match_indices(' ') {
            let rev_str = s
                .get(last_pos..pos)
                .unwrap()
                .chars()
                .rev()
                .collect::<String>();
            
            res = res + &rev_str + " ";
            last_pos = pos+1;
        }
        let mut rev_str = s.get(last_pos..).unwrap().to_string();
        rev_str = rev_str.chars().rev().collect::<String>();
        res += &rev_str;
        res
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let res = Self::reverse_words(String::from("abcd"));
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
            Solution::reverse_words(String::from("Let's take LeetCode contest")),
            String::from("s'teL ekat edoCteeL tsetnoc")
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::reverse_words(String::from("Mr Ding")),
            String::from("rM gniD")
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            Solution::reverse_words(String::from("Hello World")),
            String::from("olleH dlroW")
        );
    }
}
