/*
401. Binary Watch
Easy
Topics
premium lock iconCompanies
Hint

A binary watch has 4 LEDs on the top to represent the hours (0-11), and 6 LEDs on the bottom to represent the minutes (0-59). Each LED represents a zero or one, with the least significant bit on the right.
    For example, the below binary watch reads "4:51".
Given an integer turnedOn which represents the number of LEDs that are currently on (ignoring the PM), return all possible times the watch could represent. You may return the answer in any order.

The hour must not contain a leading zero.
    For example, "01:00" is not valid. It should be "1:00".
The minute must consist of two digits and may contain a leading zero.
    For example, "10:2" is not valid. It should be "10:02".

Example 1:
Input: turnedOn = 1
Output: ["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"]

Example 2:
Input: turnedOn = 9
Output: []

Constraints:
    0 <= turnedOn <= 10

*/

use crate::Runnable; // Import the trait from the libray root

pub struct Solution;

impl Solution {
    pub fn get_binary_ones(time_rep:i32) -> i32 {
        let mut temp = time_rep;
        let mut res = 0;
        while temp > 0 {
            res += temp % 2;
            temp /= 2;
        }
        res
    }
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut res:Vec<String> = vec![];
        for i in 0..=11 {
            for j in 0..=59 {
                if Solution::get_binary_ones(i << 6 | j) == turned_on {
                    res.push(i.to_string() + ":" + &format!("{:02}", j));
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
        let res = Solution::read_binary_watch(1);
        println!("{:?}", res);
    }
}

/*
fill up the test-cases based on description and examples
dont need more than 3 test-cases
dont touch the solution and also dont add comments
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            Solution::read_binary_watch(1),
            vec![
                "0:01".to_string(),
                "0:02".to_string(),
                "0:04".to_string(),
                "0:08".to_string(),
                "0:16".to_string(),
                "0:32".to_string(),
                "1:00".to_string(),
                "2:00".to_string(),
                "4:00".to_string(),
                "8:00".to_string()
            ]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(Solution::read_binary_watch(9), Vec::<String>::new());
    }

    #[test]
    fn test_case_3() {
        assert_eq!(Solution::read_binary_watch(0), vec!["0:00".to_string()]);
    }
}
