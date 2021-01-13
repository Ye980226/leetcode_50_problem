use std::cmp::min;
struct Solution();
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let length=strs.len();
        if length==0{
            return String::from("");
        }
        let mut longest=String::from(&strs[0]);
        
        let mut min_str_length;
        let mut longest_tmp=String::from("");
        let mut p_str:String;
        let mut q_str:String;
        for i in 1..length{
            min_str_length=min(longest.len(),strs[i].len());
            let p_str=match longest.get(..min_str_length){
                Some(value)=>value.to_string(),
                None=>"".to_string(),
            };
            let q_str=match strs[i].get(..min_str_length){
                Some(value)=>value.to_string(),
                None=>"".to_string(),
            };
            
            for (l,s) in p_str.chars().zip(q_str.chars())
            {
                if l != s{
                    break;
                }
                
                longest_tmp.push(l);
            }
            longest=longest_tmp;
            longest_tmp=String::from("");
        }
        longest
    }
}
fn main(){
    println!("{}",Solution::longest_common_prefix(vec![String::from("flower"),String::from("flow"),String::from("flight")]));
    println!("{}",Solution::longest_common_prefix(vec![String::from("dog"),String::from("racecar"),String::from("car")]));
}