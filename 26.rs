struct Solution();
// use std::collections::HashSet;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        //因为数组已经排序过了，所以可以不需要
        let length=nums.len();
        if length<2{
            return length as i32;
        }
        let mut i:usize=0;
        loop{
            if i>=nums.len()-1{
                break;
            }
            if nums[i]==nums[i+1]{
                nums.remove(i+1);
                // println!("{:?}",nums);
                continue;
            }
            i+=1;
        }
        nums.len() as i32
    }
}
fn main(){
    println!("{}",Solution::remove_duplicates(&mut vec![1,1,2]));
    println!("{}",Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]));
    println!("{}",Solution::remove_duplicates(&mut vec![]));
    println!("{}",Solution::remove_duplicates(&mut vec![1]));
}