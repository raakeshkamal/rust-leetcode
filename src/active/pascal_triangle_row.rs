/*
118. Pascal's Triangle
Easy
Topics
premium lock iconCompanies

Given an integer numRows, return the first numRows of Pascal's triangle.

In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:

Example 1:
Input: numRows = 5
Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]

Example 2:
Input: numRows = 1
Output: [[1]]

Constraints:
    1 <= numRows <= 30

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let num_rows = row_index+1;
        let mut res = vec![vec![]; num_rows as usize];
        for (i, row) in res.iter_mut().enumerate() {
            *row = vec![0; i + 1];
            row[0] = 1;
            row[i] = 1;
        }
        for i in 2..num_rows as usize {
            for j in 0..i {
                if res[i][j] == 0 {
                    res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
                }
            }
        }
        res.last().unwrap().clone()
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let res = Self::get_row(5);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::get_row(0), vec![1]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::get_row(2), vec![1, 2, 1]);
    }
}
