// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
struct Solution();
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut ans = 0;
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, level: &mut i32, ans: &mut i32)->bool{
            if let Some(node) = root {//root可以是引用
                let node = node.borrow();
                if dfs(&node.left, level, ans) {
                    return true;
                }
                *level -= 1;
                if *level == 0 {
                    *ans = node.val;
                    return true ;
                }
                if dfs(&node.right, level, ans) {
                    return true ;
                }
                return false;
            }
            false
        }
        let mut level = k;
        dfs(&root, &mut level, &mut ans);
        ans
    }
}


fn main(){
    println!("{}",Solution::kth_smallest(Some(Rc::new(RefCell::new(TreeNode{val:3,left:Some(Rc::new(RefCell::new(TreeNode{val:1,left:None,right:Some(Rc::new(RefCell::new(TreeNode{val:2,left:None,right:None})))}))),right:Some(Rc::new(RefCell::new(TreeNode{val:4,left:None,right:None})))}))),2));
}
