// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>,
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode { next: None, val }
//   }
// }

impl Solution {
  pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    use std::iter::successors;

    let list_len = successors(head.as_ref(), |node| node.next.as_ref()).count();
    let mut dummy_head = Some(Box::new(ListNode { val: 0, next: head }));

    let mut prev =
      (0..list_len - n as usize).fold(dummy_head.as_mut(), |node, _| node.unwrap().next.as_mut());
    prev.unwrap().next = prev.as_mut().unwrap().next.as_mut().unwrap().next.take();

    dummy_head.unwrap().next
  }
}
