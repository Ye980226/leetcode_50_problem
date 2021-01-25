// Definition for singly-linked list.
struct Solution();
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

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
 if head.is_none() {
    return None;
  }
  let mut prev: Option<Box<ListNode>> = None;
  let mut curr: Option<Box<ListNode>> = head;
  while curr.is_some() {
    //拿走了Option，留下None在原地
    let mut node = curr.take().unwrap();
    println!("node:{:?}",node);
    // println!("first curr{:?}",curr);
    curr = node.next;//curr指向未翻转以及这一次循环也不会参与翻转的链表，拿走了部分的所有权
    println!("{:?}",node);
    node.next = prev;//prev指向curr前面那部分，这一部分，又指向了prev，拿走了prev的所有权
    // println!("node second:{:?}",node);
    prev = Some(node);//prev又拿走了node的所有权
    println!("prev:{:?}",prev);
    println!("curr:{:?}",curr);
    println!("*******************");
    println!("*******************");
  }
  prev
    }
}
fn main(){
    Solution::reverse_list(Some(Box::new(ListNode{val:1,next:Some(Box::new(ListNode{val:2,next:Some(Box::new(ListNode{val:3,next:Some(Box::new(ListNode{val:4,next:Some(Box::new(ListNode{val:5,next:None}))}))}))}))})));
}