/*
You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
You may assume the two numbers do not contain any leading zero, except the number 0 itself.

Example 1:

Input: l1 = [2,4,3], l2 = [5,6,4]
Output: [7,0,8]
Explanation: 342 + 465 = 807.

Example 2:

Input: l1 = [0], l2 = [0]
Output: [0]

Example 3:

Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
Output: [8,9,9,9,0,0,0,1]

Constraints:

The number of nodes in each linked list is in the range [1, 100].
0 <= Node.val <= 9
It is guaranteed that the list represents a number that does not have leading zeros.
*/
use crate::Tests;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn generate_nodes(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut output = Box::new(ListNode::new(nums[0]));
    let mut current = &mut output;
    for i in 1..nums.len() {
        let b = Box::new(ListNode::new(nums[i]));
        (*current).next = Some(b);
        current = current.next.as_mut()?;
    }
    Some(output)
}

pub fn solution(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>
) -> Option<Box<ListNode>> {
    let mut carry = false;
    let mut item1 = l1.unwrap();
    let mut item2 = l2.unwrap();
    let mut output = ListNode::new(handle_add(
        item1.val, item2.val,
        &mut carry
    ));
    let mut output_cursor = &mut output;
    loop {
        if item1.next.is_none() && item2.next.is_none() {
            if carry {
                output_cursor.next = Some(Box::new(ListNode::new(
                    1
                )));
            }
            return Some(Box::new(output));
        }
        item1 = item1.next.unwrap_or(Box::new(ListNode::new(0)));
        item2 = item2.next.unwrap_or(Box::new(ListNode::new(0)));
        output_cursor.next = Some(Box::new(ListNode::new(
            handle_add(item1.val, item2.val, &mut carry)
        )));
        output_cursor = output_cursor.next.as_mut().unwrap().as_mut();
    }
}
fn handle_add(x: i32, y: i32, carry: &mut bool) -> i32 {
    let output = if *carry { x + y + 1 } else { x + y };
    if output >= 10 {
        *carry = true;
        return output - 10;
    }
    *carry = false;
    return output;
}

pub fn test(test: &mut Tests) {
    test.add_test(
        solution(
            generate_nodes(vec![2, 4, 3]),
            generate_nodes(vec![5, 6, 4]),
        ) ==
        generate_nodes(vec![7, 0, 8]),
    );
    test.add_test(
        solution(
            generate_nodes(vec![0]),
            generate_nodes(vec![0]),
        ) ==
        generate_nodes(vec![0]),
    );
    test.add_test(
        solution(
            generate_nodes(vec![9, 9, 9, 9, 9, 9, 9]),
            generate_nodes(vec![9, 9, 9, 9]),
        ) ==
        generate_nodes(vec![8, 9, 9, 9, 0, 0, 0, 1]),
    );
}
