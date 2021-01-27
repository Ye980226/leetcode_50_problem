## 231.2的幂

给定一个整数，编写一个函数来判断它是否是 2 的幂次方。

```rust
struct Solution();
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        // if n==0{
        //     return false;
        // }
        // let mut n = n;

        // while n!=0{
        //     // n>>=1;
        //     // println!("{}",n);
        //     if n==1{
        //         return true;
        //     }
        //     if n!=(n>>1)*2{
        //         return false
        //     }else{
        //         n>>=1;
        //     }
        // }
        // true
        (n>0)&&(n==(n&-n))
    }
}

fn main(){
    println!("{}",Solution::is_power_of_two(1));    
    println!("{}",Solution::is_power_of_two(16));    
    println!("{}",Solution::is_power_of_two(218));    
}
```

## 235.二叉搜索树

给定一个二叉搜索树, 找到该树中两个指定节点的最近公共祖先。

百度百科中最近公共祖先的定义为：“对于有根树 T 的两个结点 p、q，最近公共祖先表示为一个结点 x，满足 x 是 p、q 的祖先且 x 的深度尽可能大（一个节点也可以是它自己的祖先）。”

例如，给定如下二叉搜索树:  root = [6,2,8,0,4,7,9,null,null,3,5]

```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut current= root;
        let (val_p, val_q) = (p.unwrap().borrow().val, q.unwrap().borrow().val);
        let (left, right) = if val_q > val_p { (val_p, val_q) } else { (val_q, val_p) };
        loop {
            let val_cur = current.as_ref().unwrap().borrow().val;
            if left <= val_cur && val_cur <= right {
                return current;
            } else if left > val_cur && right > val_cur {
                let node = current.unwrap().borrow_mut().right.take();
                current = node;
            } else {
                let node = current.unwrap().borrow_mut().left.take();
                current = node;
            }
        }
    }
}
```

## 236.二叉树的最近公共祖先

```rust
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
```





