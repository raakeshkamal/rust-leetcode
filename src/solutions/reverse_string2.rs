/*
541. Reverse String II
Easy
Topics
premium lock iconCompanies

Given a string s and an integer k, reverse the first k characters for every 2k characters counting from the start of the string.
If there are fewer than k characters left, reverse all of them. If there are less than 2k but greater than or equal to k characters, then reverse the first k characters and leave the other as original.

Example 1:
Input: s = "abcdefg", k = 2
Output: "bacdfeg"

Example 2:
Input: s = "abcd", k = 2
Output: "bacd"

Constraints:
    1 <= s.length <= 104
    s consists of only lowercase English letters.
    1 <= k <= 104
*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let step = k as usize;
        let mut res = "".to_string();
        let mut i = 0;
        let mut j = 0;
        while i + step < s.len() {
            let mut rev_str = s.get(i..i + step).unwrap().to_string();
            if j % 2 == 0 {
                rev_str = rev_str.chars().rev().collect::<String>();
            }
            res += &rev_str;
            i += step;
            j += 1;
        }
        let mut rev_str = s.get(i..).unwrap().to_string();
        if j % 2 == 0 {
            rev_str = rev_str.chars().rev().collect::<String>();
        }
        res += &rev_str;
        res
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let res = Self::reverse_str(String::from("abcd"), 4);
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
            Solution::reverse_str(String::from("abcdefg"), 2),
            String::from("bacdfeg")
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::reverse_str(String::from("hyzqyljrnigxvdtneasepfahmtyhlohwxmkqcdfehybknvdmfrfvtbsovjbdhevlfxpdaovjgunjqlimjkfnqcqnajmebeddqsgl"), 39),
            String::from("fdcqkmxwholhytmhafpesaentdvxginrjlyqzyhehybknvdmfrfvtbsovjbdhevlfxpdaovjgunjqllgsqddebemjanqcqnfkjmi")
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            Solution::reverse_str(String::from("abcd"), 4),
            String::from("dcba")
        );
    }
}
