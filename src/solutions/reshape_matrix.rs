/*
566. Reshape the Matrix
Easy
Topics
premium lock iconCompanies
Hint

In MATLAB, there is a handy function called reshape which can reshape an m x n matrix into a new one with a different size r x c keeping its original data.
You are given an m x n matrix mat and two integers r and c representing the number of rows and the number of columns of the wanted reshaped matrix.
The reshaped matrix should be filled with all the elements of the original matrix in the same row-traversing order as they were.
If the reshape operation with given parameters is possible and legal, output the new reshaped matrix; Otherwise, output the original matrix.

Example 1:
Input: mat = [[1,2],[3,4]], r = 1, c = 4
Output: [[1,2,3,4]]

Example 2:
Input: mat = [[1,2],[3,4]], r = 2, c = 4
Output: [[1,2],[3,4]]

Constraints:
    m == mat.length
    n == mat[i].length
    1 <= m, n <= 100
    -1000 <= mat[i][j] <= 1000
    1 <= r, c <= 300

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; c as usize]; r as usize];
        let total = r * c;
        let mat_r = mat.len();
        let mat_c = match mat.first() {
            Some(row) => row.len(),
            None => return mat,
        };

        if mat_r * mat_c != total as usize {
            return mat;
        };

        let mut j = 0;
        for row in mat {
            for i in row {
                let row_num = j / c as usize;
                let col_num = j % c as usize;
                res[row_num][col_num] = i;
                j += 1;
            }
        }
        res
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let mat = vec![vec![1, 2], vec![3, 4]];
        let res = Self::matrix_reshape(mat, 1, 4);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![1, 2, 3, 4]];
        assert_eq!(Solution::matrix_reshape(mat, 1, 4), expected);
    }

    #[test]
    fn test_case_2() {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::matrix_reshape(mat, 2, 4), expected);
    }

    #[test]
    fn test_case_3() {
        let mat = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(Solution::matrix_reshape(mat, 2, 2), expected);
    }
}
