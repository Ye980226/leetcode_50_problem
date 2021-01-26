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