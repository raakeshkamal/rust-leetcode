/*

Easy
Topics
Companies
Hint
Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.

You must not use any built-in exponent function or operator.

For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.
 

Example 1:

Input: x = 4
Output: 2
Explanation: The square root of 4 is 2, so we return 2.
Example 2:

Input: x = 8
Output: 2
Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.
 

Constraints:

0 <= x <= 231 - 1
*/
pub fn my_sqrt(x: i32) -> i32 {
    let end:u64 = (x+1) as u64;
    for i in 0..end {
        if i*i > x as u64{
            return (i-1) as i32
        } else if i*i == x as u64{
            return i as i32
        }
    }
    return end as i32;    
}