/*
14. Longest Common Prefix
Easy
Topics
premium lock iconCompanies

Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".



Example 1:

Input: strs = ["flower","flow","flight"]
Output: "fl"

Example 2:

Input: strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.



Constraints:

    1 <= strs.length <= 200
    0 <= strs[i].length <= 200
    strs[i] consists of only lowercase English letters if it is non-empty.


*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    fn get_smallest_str(strs: &[String]) -> usize {
        let mut min = usize::MAX;
        let mut pos = 0;
        for i in 0..strs.len() {
            let temp = strs.get(i).unwrap().len();
            if temp < min {
                min = temp;
                pos = i;
            }
        }
        pos
    }
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let min_pos = Self::get_smallest_str(&strs);
        let min_str = strs.get(min_pos).unwrap();
        let mut res = String::from("");

        for end_pos in 0..=min_str.len() {
            let mut all_contain = true;
            if let Some(sub_str1) = min_str.get(0..end_pos) {
                for s in strs.iter() {
                    if s == min_str {
                        continue;
                    }
                    if s.get(0..end_pos).unwrap() != sub_str1 {
                        all_contain = false;
                        break;
                    }
                }
                if res.len() < sub_str1.len() && all_contain {
                    res = sub_str1.to_string();
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
        let input = ["car", "card", "cardboard"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let res = Self::longest_common_prefix(input);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let input = ["flower", "flow", "flight"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::longest_common_prefix(input), "fl");
    }

    #[test]
    fn test_case_2() {
        let input = ["dog", "racecar", "car"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::longest_common_prefix(input), "");
    }

    #[test]
    fn test_case_3() {
        let input = ["flower", "fkow"].iter().map(|x| x.to_string()).collect();
        assert_eq!(Solution::longest_common_prefix(input), "f");
    }
}
