struct Solution();
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
fn main(){
    println!("{}",Solution::contains_duplicate(vec![1,2,3,1]));
}