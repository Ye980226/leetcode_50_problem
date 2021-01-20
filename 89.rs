struct Solution();
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        //动态规划
        let mut gray_vec=Vec::new();
        
        gray_vec.push(0);
        // if n==0{
        //     return gray_vec;
        // }
        // gray_vec.push(1);
        // if n==1{
        //     return gray_vec;
        // }
        for i in 1..n+1{
            let mut length=gray_vec.len();
            while length>0{
                length-=1;
                // println!("{}",1<<(i-1));
                // println!("{}",gray_vec[length]);
                gray_vec.push(gray_vec[length]+(1<<(i-1)));
            }
        }
        gray_vec
    }
}
fn main(){
    println!("{:?}",Solution::gray_code(2));
    println!("{:?}",Solution::gray_code(0));
    println!("{:?}",Solution::gray_code(1));
    println!("{:?}",Solution::gray_code(3));
}