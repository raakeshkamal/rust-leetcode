/*
500. Keyboard Row
Easy
Topics
premium lock iconCompanies

Given an array of strings words, return the words that can be typed using letters of the alphabet on only one row of American keyboard like the image below.
Note that the strings are case-insensitive, both lowercased and uppercased of the same letter are treated as if they are at the same row.

In the American keyboard:
    the first row consists of the characters "qwertyuiop",
    the second row consists of the characters "asdfghjkl", and
    the third row consists of the characters "zxcvbnm".

Example 1:
Input: words = ["Hello","Alaska","Dad","Peace"]
Output: ["Alaska","Dad"]
Explanation
Both "a" and "A" are in the 2nd row of the American keyboard due to case insensitivity.

Example 2:
Input: words = ["omk"]
Output: []

Example 3:
Input: words = ["adsdf","sfd"]
Output: ["adsdf","sfd"]

Constraints:
    1 <= words.length <= 20
    1 <= words[i].length <= 100
    words[i] consists of English letters (both lowercase and uppercase).

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        let rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"]
            .iter()
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        let temp_words = words
            .iter()
            .map(|s| s.to_ascii_lowercase())
            .collect::<Vec<String>>();

        for row in rows {
            for (idx, word) in temp_words.iter().enumerate() {
                let mut same_row = true;
                for ch in word.chars() {
                    if !row.contains(ch) {
                        same_row = false;
                    }
                }
                if same_row {
                    res.push(words[idx].clone());
                }
            }
        }
        res
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let input = ["Hello", "Alaska", "Dad", "Peace"]
            .iter()
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        let res = Self::find_words(input);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = ["Hello", "Alaska", "Dad", "Peace"]
            .iter()
            .map(|word| word.to_string())
            .collect::<Vec<String>>();
        let output = ["Alaska", "Dad"]
            .iter()
            .map(|word| word.to_string())
            .collect::<Vec<String>>();
        assert_eq!(Solution::find_words(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = ["omk"]
            .iter()
            .map(|word| word.to_string())
            .collect::<Vec<String>>();
        let output: Vec<String> = vec![];
        assert_eq!(Solution::find_words(input), output);
    }

    #[test]
    fn test_case_3() {
        let input = ["adsdf", "sfd"]
            .iter()
            .map(|word| word.to_string())
            .collect::<Vec<String>>();
        let output = ["adsdf", "sfd"]
            .iter()
            .map(|word| word.to_string())
            .collect::<Vec<String>>();
        assert_eq!(Solution::find_words(input), output);
    }
}
