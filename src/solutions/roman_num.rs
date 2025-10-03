/*
13. Roman to Integer
Easy
Topics
premium lock iconCompanies
Hint

Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000

For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

    I can be placed before V (5) and X (10) to make 4 and 9.
    X can be placed before L (50) and C (100) to make 40 and 90.
    C can be placed before D (500) and M (1000) to make 400 and 900.

Given a roman numeral, convert it to an integer.



Example 1:

Input: s = "III"
Output: 3
Explanation: III = 3.

Example 2:

Input: s = "LVIII"
Output: 58
Explanation: L = 50, V= 5, III = 3.

Example 3:

Input: s = "MCMXCIV"
Output: 1994
Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.



Constraints:

    1 <= s.length <= 15
    s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
    It is guaranteed that s is a valid roman numeral in the range [1, 3999].

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let num = [1000, 500, 100, 50, 10, 5, 1];
        let rom = ['M', 'D', 'C', 'L', 'X', 'V', 'I'];
        let mut res = 0;
        let mut i = 0;

        loop {
            if s.chars().nth(i) == Some('I') && s.chars().nth(i + 1) == Some('V') {
                res += 4;
                i += 2;
                continue;
            } else if s.chars().nth(i) == Some('I') && s.chars().nth(i + 1) == Some('X') {
                res += 9;
                i += 2;
                continue;
            } else if s.chars().nth(i) == Some('X') && s.chars().nth(i + 1) == Some('L') {
                res += 40;
                i += 2;
                continue;
            } else if s.chars().nth(i) == Some('X') && s.chars().nth(i + 1) == Some('C') {
                res += 90;
                i += 2;
                continue;
            } else if s.chars().nth(i) == Some('C') && s.chars().nth(i + 1) == Some('D') {
                res += 400;
                i += 2;
                continue;
            } else if s.chars().nth(i) == Some('C') && s.chars().nth(i + 1) == Some('M') {
                res += 900;
                i += 2;
                continue;
            }
            for j in 0..rom.len() {
                if s.chars().nth(i) == Some(rom[j]) {
                    res += num[j];
                }
            }
            if i == s.len() {
                break;
            }
            i +=1;
        }
        res
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let input = "XXVII".to_string();
        let res = Self::roman_to_int(input);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
