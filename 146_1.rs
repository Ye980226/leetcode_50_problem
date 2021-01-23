use std::collections::HashMap;
use std::rc::{Rc,Weak};
use std::cell::RefCell;

struct ListNode {
    key: i32,
    val: i32,
    prev: Option<Weak<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(key:i32, val: i32) -> Self {
        ListNode{
            key, 
            val,
            prev: None,
            next: None,
        }
    }

    fn insert_after(node: Rc<RefCell<ListNode>>, head: Rc<RefCell<ListNode>>) {
        let mut next = head.borrow_mut().next.take();
        next.as_mut().unwrap().borrow_mut().prev = Some(Rc::downgrade(&node));
        node.borrow_mut().next = next;
        node.borrow_mut().prev = Some(Rc::downgrade(&head));
        head.borrow_mut().next = Some(node);
    }

    fn pop(node: Rc<RefCell<ListNode>>) -> Rc<RefCell<ListNode>> {
        let next = node.borrow_mut().next.take().unwrap();
        let prev = node.borrow_mut().prev.as_ref().unwrap().upgrade().unwrap();
        prev.borrow_mut().next = Some(Rc::clone(&next));
        next.borrow_mut().prev = Some(Rc::downgrade(&prev));
        node.borrow_mut().prev = None;
        node
    }
}

struct LRUCache {
    map: HashMap<i32, Option<Rc<RefCell<ListNode>>>>,
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
    capacity: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let (head, tail) = (Rc::new(RefCell::new(ListNode::new(0, 0))), Rc::new(RefCell::new(ListNode::new(0, 0))));
        head.borrow_mut().next = Some(Rc::clone(&tail));
        tail.borrow_mut().prev = Some(Rc::downgrade(&head));
        LRUCache {
            map: HashMap::new(),
            head: Some(head),
            tail: Some(tail),
            capacity: capacity as usize,
        }
    }
    
    fn get(&self, key: i32) -> i32 {
        self.get_node(key).0
    }

    fn get_node(&self, key: i32) -> (i32, Option<Rc<RefCell<ListNode>>>) {
        let (mut ret, mut ret_node ) = (-1, None);
        let node = self.map.get(&key);
        if node.is_none() { return (ret, ret_node); }
        if let Some(x) = node {
            let node_rc = ListNode::pop(Rc::clone(x.as_ref().unwrap()));
            ret = node_rc.borrow().val;
            ret_node = Some(Rc::clone(&node_rc));
            ListNode::insert_after(node_rc, Rc::clone(&self.head.as_ref().unwrap()))
        }
        (ret, ret_node)
    }
    
    fn put(&mut self, key: i32, value: i32) {
        let (ret, ret_node) = self.get_node(key);
        if  ret != -1 {
            ret_node.unwrap().borrow_mut().val = value;
        } else {
            if self.map.len() >= self.capacity {
                let to_del = Rc::clone(&self.tail.as_ref().unwrap().borrow().prev.as_ref().unwrap().upgrade().unwrap());
                let to_del = ListNode::pop(to_del);
                self.map.remove(&to_del.borrow().key);
            }
            let new = Rc::new(RefCell::new(ListNode::new(key, value)));
            self.map.insert(key, Some(Rc::clone(&new)));
            ListNode::insert_after(new, Rc::clone(&self.head.as_ref().unwrap()));
        }
    }
}
