/*
70. Climbing Stairs
Easy
Topics
premium lock iconCompanies
Hint

You are climbing a staircase. It takes n steps to reach the top.

Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

Example 1:

Input: n = 2
Output: 2
Explanation: There are two ways to climb to the top.
1. 1 step + 1 step
2. 2 steps

Example 2:

Input: n = 3
Output: 3
Explanation: There are three ways to climb to the top.
1. 1 step + 1 step + 1 step
2. 1 step + 2 steps
3. 2 steps + 1 step

Constraints:

    1 <= n <= 45

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut temp = vec![0, 1, 2];
        for i in 3..=n as usize {
            temp.push(temp[i - 1] + temp[i - 2])
        }
        temp[n as usize]
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let val = 4;
        let res = Self::climb_stairs(val);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::climb_stairs(4), 5);
    }
    
    #[test]
    fn test_case_4() {
        assert_eq!(Solution::climb_stairs(1), 1);
    }
}
