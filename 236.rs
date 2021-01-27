// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        dfs(&root, p.unwrap().borrow().val, q.unwrap().borrow().val)
    }
}

fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, p_val: i32, q_val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(x) = node {
        if x.borrow().val == p_val || x.borrow().val == q_val { return Some(Rc::clone(x)); }
        let left = dfs(&x.borrow().left, p_val, q_val);
        let right = dfs(&x.borrow().right, p_val, q_val);
        if left.is_none() { right } else if right.is_none() { left } else { Some(Rc::clone(x)) }
    } else {
        None
    }
}