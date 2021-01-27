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
