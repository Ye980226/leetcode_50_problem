struct Solution();
// use std::collections::HashMap;

// impl Solution {
//     pub fn majority_element(nums: Vec<i32>) -> i32 {
//         let mut map:HashMap<i32,u32>=HashMap::new();
//         let length=nums.len();
//         let half_length=(length+1)/2;
//         for i in 0..length{
//             match map.get_mut(&nums[i]){
//                 Some(value)=>{
//                     *value+=1;
//                     if *value>=half_length as u32{
//                         return nums[i]
//                     }

//                 }
//                 None=>{
//                     map.insert(nums[i],1);
//                 }
//             }
//         }
//         nums[0]
//     }
// }

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut candidate,length,mut count)=(nums[0],nums.len(),1);
        for i in 1..length{
            count=if candidate==nums[i]{count + 1}else {count-1};
            if count==0{
                candidate=nums[i];
                count=1;
            }
        }
        candidate
    }
}
fn main(){
    println!("{}",Solution::majority_element(vec![3,2,3]));
    println!("{}",Solution::majority_element(vec![2,2,1,1,1,2,2]));
    println!("{}",Solution::majority_element(vec![6,5,5]));
}