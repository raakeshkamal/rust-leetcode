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

pub fn is_palindrome(s: String) -> bool {
    let byte_str = s.as_bytes();
    let mut final_byte_str: Vec<u8> = Vec::new();
    for i in byte_str {
        if (*i >= 97 && *i <= 122) || (*i >= 48 && *i <= 57) {
            final_byte_str.push(*i);
        } else if *i >= 65 && *i <= 90 {
            final_byte_str.push(*i + 32);
        }
    }
    let str_len = final_byte_str.len();
    if str_len == 0 {
        return true;
    }
    let mut mid: usize = str_len;
    if (str_len % 2) > 0 {
        mid /= 2;
    } else {
        mid = (mid / 2) + 1;
    }
    for i in 0..mid {
        let j = str_len - i - 1;
        if final_byte_str[j] != final_byte_str[i] {
            return false;
        }
    }
    true
}
