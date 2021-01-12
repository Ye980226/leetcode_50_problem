struct Solution();
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut result=0;
        let mut mod_;
        let mut y:i64=x as i64;
        while y!=0{
            // println!("y value is{}",y);
            mod_ = y%10;
            y=(y-mod_)/10;
            result=result*10+mod_
        }
        if result>i32::MAX as i64 || result < i32::MIN as i64{
            0 
        }else{
            result as i32
        }
    }
    
}
fn main(){
    let a=Solution::reverse(1534236469);
    println!("{}",a);
}