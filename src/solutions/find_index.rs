/*
28. Find the Index of the First Occurrence in a String
Easy
Topics
Companies

Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.



Example 1:

Input: haystack = "sadbutsad", needle = "sad"
Output: 0
Explanation: "sad" occurs at index 0 and 6.
The first occurrence is at index 0, so we return 0.

Example 2:

Input: haystack = "leetcode", needle = "leeto"
Output: -1
Explanation: "leeto" did not occur in "leetcode", so we return -1.



Constraints:

    1 <= haystack.length, needle.length <= 104
    haystack and needle consist of only lowercase English characters.
*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }
        for i in 0..haystack.len() - needle.len() + 1 {
            let sub_str = &haystack[i..i + needle.len()];
            if sub_str == needle {
                return i as i32;
            }
        }
        -1
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let haystack = "sadbutsad".to_string();
        let needle = "sad".to_string();
        let res = Self::str_str(haystack, needle);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::str_str("leetcode".to_string(), "leeto".to_string()), -1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::str_str("leetcode".to_string(), "leet".to_string()), 0);
    }
}
