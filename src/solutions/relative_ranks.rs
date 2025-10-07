/*
506. Relative Ranks
Easy
Topics
premium lock iconCompanies

You are given an integer array score of size n, where score[i] is the score of the ith athlete in a competition. All the scores are guaranteed to be unique.

The athletes are placed based on their scores, where the 1st place athlete has the highest score, the 2nd place athlete has the 2nd highest score, and so on. The placement of each athlete determines their rank
    The 1st place athlete's rank is "Gold Medal".
    The 2nd place athlete's rank is "Silver Medal".
    The 3rd place athlete's rank is "Bronze Medal".
    For the 4th place to the nth place athlete, their rank is their placement number (i.e., the xth place athlete's rank is "x
Return an array answer of size n where answer[i] is the rank of the ith athlete.

Example 1
Input: score = [5,4,3,2,1]
Output: ["Gold Medal","Silver Medal","Bronze Medal","4","5"]
Explanation: The placements are [1st, 2nd, 3rd, 4th, 5th].

Example 2
Input: score = [10,3,8,9,4]
Output: ["Gold Medal","5","Bronze Medal","Silver Medal","4"]
Explanation: The placements are [1st, 5th, 3rd, 2nd, 4th].

Constraints
    n == score.length
    1 <= n <= 104
    0 <= score[i] <= 106
    All the values in score are unique

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut res: Vec<String> = vec![String::new(); score.len()];
        let mut temp = score.clone();
        let mut pos_arr: Vec<usize> = (0..temp.len()).map(|i| i).collect();
        for i in 0..temp.len() {
            for j in 0..temp.len() - i - 1 {
                if temp[j] < temp[j + 1] {
                    let tmp = temp[j];
                    temp[j] = temp[j + 1];
                    temp[j + 1] = tmp;
                    let tmp = pos_arr[j];
                    pos_arr[j] = pos_arr[j + 1];
                    pos_arr[j + 1] = tmp;
                }
            }
        }
        for (idx,pos) in pos_arr.iter().enumerate() {
            match idx {
                0 => res[*pos] = format!("Gold Medal"),
                1 => res[*pos] = format!("Silver Medal"),
                2 => res[*pos] = format!("Bronze Medal"),
                _ => res[*pos] = format!("{:?}", idx+1)
            }
        }
        res
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        // This is the example code you wanted to move
        let val = vec![10, 3, 8, 9, 4];
        let res = Self::find_relative_ranks(val);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
            vec!["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
            vec!["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            Solution::find_relative_ranks(vec![1, 2, 3, 4, 5]),
            vec!["5", "4", "Bronze Medal", "Silver Medal", "Gold Medal"]
        );
    }
}
