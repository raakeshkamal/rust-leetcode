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

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::<char>::new(); 

    for c in s.chars() {
        if stack.is_empty() {
            stack.push(c);
        } else {
            let temp = stack.last();
            let mut pop_last = false;
            if *temp.unwrap() == '(' && c == ')' {
                pop_last = true;
            } else if *temp.unwrap() == '{' && c == '}' {
                pop_last = true;
            } else if *temp.unwrap() == '[' && c == ']' {
                pop_last = true;
            }

            if pop_last {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
    }
    return stack.is_empty();
}
