## 215. 数组中的第K个最大元素

在未排序的数组中找到第 **k** 个最大的元素。请注意，你需要找的是数组排序后的第 k 个最大的元素，而不是第 k 个不同的元素。

```rust
struct Solution();
// impl Solution {
//     pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
//         let mut nums=nums;
//         nums.sort();
//         nums[nums.len()-k as usize]
//     }
// }
use std::collections::BinaryHeap ;
impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut bt: BinaryHeap<i32> = BinaryHeap::new();
        for i in nums {
            bt.push(i);
        }
        for i in 1.. {
            if let Some(val)=bt.pop(){
                if i == k {
                    return val;
                }
            }
        }
        0
    }
}
fn main(){
    println!("{}",Solution::find_kth_largest(vec![3,2,1,5,6,4],2));
}
```



## 217.存在重复元素

给定一个整数数组，判断是否存在重复元素。如果存在一值在数组中出现至少两次，函数返回 `true` 。如果数组中每个元素都不相同，则返回 `false` 。

```rust
use std::collections::HashMap;
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map:HashMap<i32,u8>=HashMap::new();
        for i in 0..nums.len(){
            match map.get_mut(&nums[i]){
                Some(_value)=>{return true;},
                None=>{map.insert(nums[i],1);},
            }
        }
        false
        // println!("{}",nums[0]);//不报错是因为实现了copy
        
        
    }
}
```

## 230.二叉搜索树

给定一个二叉搜索树，编写一个函数 kthSmallest 来查找其中第 k 个最小的元素。

说明：
你可以假设 k 总是有效的，1 ≤ k ≤ 二叉搜索树元素个数。

```rust
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

```



