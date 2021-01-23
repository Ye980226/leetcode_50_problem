// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    /// Sort List
    /// 
    /// Sort a linked list in O(nlogn) time using constant space complexity
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let len = |mut x: &Option<Box<ListNode>>|{
            let mut cnt = 0;
            while let Some(node) = x {
                cnt += 1;
                x = &node.next;
            }
            cnt
        };
        
        let len = len(&head);
        ListNode::sort_list_help1_(head, len)
    }

}

impl ListNode {
        /// split list
    pub fn sort_list_help1_(mut list: Option<Box<ListNode>>, len: usize) -> Option<Box<ListNode>> {
        if list.is_none() || list.as_deref()?.next.is_none() {
            return list;
        }
        
        let half_palce = (len + 1) >> 1;
        let mut right = &mut list;
        for _ in 0..half_palce {
            right = &mut right.as_deref_mut()?.next;
        }
        
        let right = right.take();
        let left = ListNode::sort_list_help1_(list, half_palce);
        let right = ListNode::sort_list_help1_(right, len - half_palce);
        
        ListNode::sort_list_help2_(left, right)
    }

    /// merge list
    pub fn sort_list_help2_(mut left: Option<Box<ListNode>>, mut right: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut anchor = ListNode::new(0);
        let mut tail = &mut anchor.next;
        
        'out: loop {
            
            let is_left = if left.is_some() && right.is_some() {
                if left.as_deref()?.val < right.as_deref()?.val { true } else { false }
            } else if left.is_some() { true
            } else if right.is_some() { false
            } else { break 'out; };
            
            *tail = if is_left {
                let mut tmp= left.take();
                left = tmp.as_deref_mut()?.next.take();
                tmp
            } else {
                let mut tmp = right.take();
                right = tmp.as_deref_mut()?.next.take();
                tmp
            };
            
            tail = &mut tail.as_deref_mut()?.next;
        }
        
        anchor.next
    }

}

