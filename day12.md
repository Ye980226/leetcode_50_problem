## 146. LRU 缓存机制
运用你所掌握的数据结构，设计和实现一个  LRU (最近最少使用) 缓存机制 。
实现 LRUCache 类：

LRUCache(int capacity) 以正整数作为容量 capacity 初始化 LRU 缓存
int get(int key) 如果关键字 key 存在于缓存中，则返回关键字的值，否则返回 -1 。
void put(int key, int value) 如果关键字已经存在，则变更其数据值；如果关键字不存在，则插入该组「关键字-值」。当缓存容量达到上限时，它应该在写入新数据之前删除最久未使用的数据值，从而为新的数据值留出空间。
```rust
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
```
## 148. 排序链表
给你链表的头结点 head ，请将其按 升序 排列并返回 排序后的链表 。

进阶：

你可以在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序吗？

```rust
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
```
## 155. 最小栈
设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。

push(x) —— 将元素 x 推入栈中。
pop() —— 删除栈顶的元素。
top() —— 获取栈顶元素。
getMin() —— 检索栈中的最小元素。

```rust
struct MinStack {
    stack_value: Vec<i32>,
    stack_min: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack{
            stack_value: vec![],
            stack_min: vec![],
        }
    }
    
    fn push(&mut self, x: i32) {
        self.stack_value.push(x);
        let length = self.stack_min.len();
        if length == 0{
            self.stack_min.push(x);
        }else{
            let min = self.stack_min[length-1];
            if x <= min{
                self.stack_min.push(x);
            }else{
                self.stack_min.push(min);
            }
        }
    }
    
    fn pop(&mut self) {
        self.stack_value.pop();
        self.stack_min.pop();
    }
    
    fn top(&self) -> i32 {
        let length = self.stack_value.len();
        self.stack_value[length-1]
    }
    
    fn get_min(&self) -> i32 {
        let length = self.stack_min.len();
        self.stack_min[length-1]
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(x);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
```
