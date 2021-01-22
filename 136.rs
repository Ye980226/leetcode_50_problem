struct Solution();
// use std::collections::HashMap;
impl Solution {
    // //
    pub fn single_number(nums: Vec<i32>) -> i32 {
    //     let length=nums.len();
    //     let mut num_count=HashMap::new();
    //     for i in 0..length{
    //         match num_count.remove(&nums[i]){
    //             Some(_value)=>{},
    //             None=>{num_count.insert(nums[i],1);},   
    //         }
    //     }
    //     if let Some(&value)=num_count.keys().next(){
    //         return value;
    //     }
    //     nums[0]

    // }
    //异或可以直接判断
        let length=nums.len();
        let mut rslt=0;//0^任何数=任何数
        //同样的数取异或=0
        for i in 0..length{
            rslt^=nums[i];
        }
        rslt
    }
}
fn main(){
    println!("{}",Solution::single_number(vec![2,2,1]));
    println!("{}",Solution::single_number(vec![4,1,2,1,2]));
}