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