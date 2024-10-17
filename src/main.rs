/*
mod sqrt;
use sqrt::my_sqrt;

fn main() {
    println!("{}", my_sqrt(2));
}
*/

/*
mod merge_sort;
use merge_sort::merge;

fn main() {
    let mut nums1: Vec<i32> = vec![1, 2, 2, 5, 5, 0, 0, 0, 0, 0];
    let mut nums2: Vec<i32> = vec![1, 2, 2, 4, 5];
    merge(&mut nums1, 5, &mut nums2, 5);
    println!("{:?}", nums1);
}
*/

/*
mod valid_palin;
use valid_palin::is_palindrome;

fn main() {
    let str = String::from("0P");
    println!("{}", is_palindrome(str));
}
*/

mod column;
use column::convert_to_title;

fn main(){
    println!("{}",convert_to_title(703));
}