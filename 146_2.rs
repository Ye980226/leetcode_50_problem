use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

struct LRUCache {
    map: HashMap<i32, Option<Rc<RefCell<ListNode>>>>,
    capacity: usize,
    head: Option<Rc<RefCell<ListNode>>>,
    tail: Option<Rc<RefCell<ListNode>>>,
}

struct ListNode {
    key: i32,
    val: i32,
    prev: Option<Rc<RefCell<ListNode>>>,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn init() -> (Rc<RefCell<ListNode>>, Rc<RefCell<ListNode>>) {
        let head_rc = Rc::new(RefCell::new(ListNode{ key: 0, val: 0, prev: None, next: None,}));
        let tail_rc = Rc::new(RefCell::new(ListNode{ key: 0, val: 0, prev: Some(Rc::clone(&head_rc)), next: None,}));      
        head_rc.borrow_mut().next = Some(Rc::clone(&tail_rc));
        (head_rc, tail_rc)
    }

    fn rmv(node_delete: Rc<RefCell<ListNode>>) -> Rc<RefCell<ListNode>> {
        // update prev
        let node_prev = Rc::clone(node_delete.borrow().prev.as_ref().unwrap());
        node_prev.borrow_mut().next = Some(Rc::clone(node_delete.borrow().next.as_ref().unwrap()));

        // update next
        let node_next = Rc::clone(node_delete.borrow().next.as_ref().unwrap());
        node_next.borrow_mut().prev = Some(Rc::clone(node_delete.borrow().prev.as_ref().unwrap()));

        node_delete
    }

    fn remove(node_delete: &mut Option<Rc<RefCell<ListNode>>>) -> Rc<RefCell<ListNode>> {
        let node_delete = node_delete.take().unwrap();
        ListNode::rmv(node_delete)
    }

    fn push_back(tail: &Option<Rc<RefCell<ListNode>>>, new: &Option<Rc<RefCell<ListNode>>>) {
        let node = Rc::clone(new.as_ref().unwrap());

        let node_tail = Rc::clone(tail.as_ref().unwrap());
        
        // last
        let node_tail_prev = Rc::clone(node_tail.borrow().prev.as_ref().unwrap());
        let mut node_tail_prev = node_tail_prev.borrow_mut();
        node_tail_prev.next = Some(Rc::clone(new.as_ref().unwrap()));
        
        // new
        node.borrow_mut().prev = Some(Rc::clone(node_tail.borrow().prev.as_ref().unwrap()));
        node.borrow_mut().next = Some(Rc::clone(&node_tail));

        // tail
        node_tail.borrow_mut().prev = Some(Rc::clone(new.as_ref().unwrap()));
    }

    fn pop_front(head: &Option<Rc<RefCell<ListNode>>>) -> Rc<RefCell<ListNode>> {
        let mut node_head = head.as_ref().unwrap().borrow_mut();
        let node_delete = node_head.next.take().unwrap();
        std::mem::drop(node_head); // borrow check is too strict to compile success
        ListNode::rmv(node_delete)
    }
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let (head_rc, tail_rc) = ListNode::init();
        LRUCache {
            map: HashMap::new(),
            capacity: capacity as usize,
            head: Some(head_rc),
            tail: Some(tail_rc),
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some(v) = self.map.get_mut(&key) {
            let value = ListNode::remove(v);
            let ret = value.borrow().val;
            ListNode::push_back(&self.tail, &Some(Rc::clone(&value)));
            self.map.insert(key, Some(value));
            ret
        }else {
            -1
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if self.map.contains_key(&key) {
            self.get(key);
            if let Some(v) = self.map.get(&key) {
                let node = v.as_ref().unwrap();
                node.borrow_mut().key = key;
                node.borrow_mut().val = value;
            }
        } else {
            if self.map.len() == self.capacity {
                let node_del = ListNode::pop_front(&self.head);
                let key_del = node_del.borrow().key;
                self.map.remove(&key_del);
            } 
            let new = Rc::new(RefCell::new(ListNode{ key, val: value, prev: None, next: None,}));
            ListNode::push_back(&self.tail, &Some(Rc::clone(&new)));
            self.map.insert(key, Some(new));
        }
    }
}
