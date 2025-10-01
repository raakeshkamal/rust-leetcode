/*
168. Excel Sheet Column Title
Easy
Topics
Companies
Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.

For example:

A -> 1
B -> 2
C -> 3
...
Z -> 26
AA -> 27
AB -> 28
...


Example 1:

Input: columnNumber = 1
Output: "A"
Example 2:

Input: columnNumber = 28
Output: "AB"
Example 3:

Input: columnNumber = 701
Output: "ZY"


Constraints:

1 <= columnNumber <= 231 - 1
*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut str = String::from("");
        let mut i = column_number;
        loop {
            let x: u8;
            if i % 26 == 0 {
                x = 26;
            } else {
                x = ((i) % 26) as u8;
            }
            str.insert(0, (x + 64) as char);
            if i <= 26 {
                break;
            }
            if i % 26 == 0 {
                i -= 26;
            }
            i /= 26;
        }
        str
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let res = Self::convert_to_title(1);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::convert_to_title(1), "A".to_string());
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::convert_to_title(28), "AB");
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::convert_to_title(701), "ZY");
    }
}
