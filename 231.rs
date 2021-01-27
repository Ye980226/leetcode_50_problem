struct Solution();
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        // if n==0{
        //     return false;
        // }
        // let mut n = n;

        // while n!=0{
        //     // n>>=1;
        //     // println!("{}",n);
        //     if n==1{
        //         return true;
        //     }
        //     if n!=(n>>1)*2{
        //         return false
        //     }else{
        //         n>>=1;
        //     }
        // }
        // true
        (n>0)&&(n==(n&-n))
    }
}

fn main(){
    println!("{}",Solution::is_power_of_two(1));    
    println!("{}",Solution::is_power_of_two(16));    
    println!("{}",Solution::is_power_of_two(218));    
}