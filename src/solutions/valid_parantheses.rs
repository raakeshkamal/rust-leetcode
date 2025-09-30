/*
20. Valid Parentheses
Easy
Topics
Companies
Hint

Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

    Open brackets must be closed by the same type of brackets.
    Open brackets must be closed in the correct order.
    Every close bracket has a corresponding open bracket of the same type.

 */

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::<char>::new();

        for c in s.chars() {
            if stack.is_empty() {
                stack.push(c);
            } else {
                let temp = stack.last();
                let mut pop_last = false;
                if *temp.unwrap() == '(' && c == ')'
                    || *temp.unwrap() == '{' && c == '}'
                    || *temp.unwrap() == '[' && c == ']'
                {
                    pop_last = true;
                }

                if pop_last {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            }
        }
        stack.is_empty()
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let res = Self::is_valid("A man, a plan, a canal: Panama".to_string());
        println!("{:?}", res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(Solution::is_valid("([])".to_string()), true);
    }

    #[test]
    fn test_case_5() {
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
    }
}
