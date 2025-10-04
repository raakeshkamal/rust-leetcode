/*
83. Remove Duplicates from Sorted List
Easy
Topics
premium lock iconCompanies

Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.

Example 1:
Input: head = [1,1,2]
Output: [1,2]

Example 2:
Input: head = [1,1,2,3,3]
Output: [1,2,3]

Constraints:
    The number of nodes in the list is in the range [0, 300].
    -100 <= Node.val <= 100
    The list is guaranteed to be sorted in ascending order.

*/

use crate::Runnable; // Import the trait from the library root

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// A struct to represent the list itself
#[derive(Default)]
pub struct List {
    head: Option<Box<ListNode>>,
}

impl List {
    // Creates a new, empty list
    pub fn new() -> Self {
        Self::default()
    }

    /// Pushes a new node to the front of the list.
    /// Time complexity: O(1)
    pub fn push_front(&mut self, val: i32) {
        let mut new_node = Box::new(ListNode::new(val));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    /// Pushes a new node to the back of the list.
    /// Time complexity: O(n)
    pub fn push_back(&mut self, val: i32) {
        match self.head.as_mut() {
            None => self.head = Some(Box::new(ListNode::new(val))),
            Some(mut current) => {
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = Some(Box::new(ListNode::new(val)));
            }
        }
    }

    /// Pops a node from the front of the list.
    /// Returns the value of the popped node, or None if the list is empty.
    /// Time complexity: O(1)
    pub fn pop_front(&mut self) -> Option<i32> {
        self.head.take().map(|mut old_head| {
            self.head = old_head.next.take();
            old_head.val
        })
    }

    /// Pops a node from the back of the list.
    /// Returns the value of the popped node, or None if the list is empty.
    /// Time complexity: O(n) because the list is singly linked.
    pub fn pop_back(&mut self) -> Option<i32> {
        self.head.as_ref()?;

        if self.head.as_ref()?.next.is_none() {
            return self.head.take().map(|node| node.val);
        }

        let mut current = self.head.as_mut()?;
        while current.next.is_some() {
            if current
                .next
                .as_ref()
                .map(|next| next.next.is_none())
                .unwrap_or(false)
            {
                let tail = current.next.take()?;
                return Some(tail.val);
            }
            current = current.next.as_mut()?;
        }

        None
    }
}

impl Solution {
    pub fn print_immutable_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;
        let mut current = head.as_ref();

        while let Some(node) = current {
            print!("{} ", node.val);
            current = node.next.as_ref();
        }
        head
    }

    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = List::default();
        match head {
            None => return head,
            Some(_) => {
                let mut prev = head.as_ref();
                let mut next = prev.unwrap().next.as_ref();
                list.push_back(prev.unwrap().val);
                while let Some(node) = next {
                    let same = prev.unwrap().val == next.unwrap().val;
                    if !same {
                        list.push_back(node.val);
                    }
                    prev = next;
                    next = node.next.as_ref();
                }
            }
        }

        list.head
    }
}

// Implement the Runnable trait for this specific solution
impl Runnable for Solution {
    fn run() {
        let mut list = List::default();
        let input = [1, 1, 2];

        for i in input {
            list.push_back(i);
        }
        let res = Solution::delete_duplicates(list.head);
        println!("{:?}", res);
    }
}

// Tests remain unchanged
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut list = List::default();
        let input = [1, 1, 2];
        let mut res = List::default();
        let output = [1, 2];

        for i in input {
            list.push_back(i);
        }

        for i in output {
            res.push_back(i);
        }
        assert_eq!(Solution::delete_duplicates(list.head), res.head);
    }

    #[test]
    fn test_case_2() {
        let mut list = List::default();
        let input = [1, 1, 2, 3, 3];
        let mut res = List::default();
        let output = [1, 2, 3];

        for i in input {
            list.push_back(i);
        }

        for i in output {
            res.push_back(i);
        }
        assert_eq!(Solution::delete_duplicates(list.head), res.head);
    }

    #[test]
    fn test_case_3() {
        let mut list = List::default();
        let input = [5, 5];
        let mut res = List::default();
        let output = [5];

        for i in input {
            list.push_back(i);
        }

        for i in output {
            res.push_back(i);
        }
        assert_eq!(Solution::delete_duplicates(list.head), res.head);
    }

    #[test]
    fn test_case_4() {
        let mut list = List::default();
        let input = [1];
        let mut res = List::default();
        let output = [1];

        for i in input {
            list.push_back(i);
        }

        for i in output {
            res.push_back(i);
        }
        assert_eq!(Solution::delete_duplicates(list.head), res.head);
    }
}
