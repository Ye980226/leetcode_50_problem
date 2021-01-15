
struct Solution();
use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut right_left=HashMap::new();
        right_left.insert(')','(');
        right_left.insert(']','[');
        right_left.insert('}','{');
        let mut record=Vec::new();
        let length=s.len();
        // let mut flag=true;
        let chars=s.chars().collect::<Vec<char>>();
        for i in 0..length{
            let c= chars[i];
            if c=='(' || c=='[' || c=='{'{
                record.push(c);
            }else{
                // println!("{:?}",record);
                if let Some(left)=right_left.get(&c)
                {
                        
                    match record.pop()
                    {
                        Some(value)=>
                        {
                            if value!=*left
                            {
                                return false;
                            }
                        }
                        None=>
                        {
                            return false;
                        }
                    }
                    
                }
               
            }
        }
        if record.len()>0{
            false
        }else{
            true
        }
    }
}
fn main(){
    println!("{}",Solution::is_valid(String::from("(((([]{}")));
    println!("{}",Solution::is_valid(String::from("()")));
    println!("{}",Solution::is_valid(String::from("()[]{}")));
    println!("{}",Solution::is_valid(String::from("([)]")));
    println!("{}",Solution::is_valid(String::from("{[]}")));
    println!("{}",Solution::is_valid(String::from("]")));
    println!("{}",Solution::is_valid(String::from("((")));
    println!("{}",Solution::is_valid(String::from("(([]){})")));
    
}