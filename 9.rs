struct Solution();
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut result:i64=0;
        let mut mod_;
        let mut y:i64=x as i64;
        while y!=0{
            // println!("y value is{}",y);
            mod_ = y%10;
            y=(y-mod_)/10;
            result=result*10+mod_
        }
        if result>i32::MAX as i64 || result < i32::MIN as i64{
            let result:i64=0 as i64;
        }
        if result<0{
            false
        }else if result==x as i64{
            true
        }else{
            false
        }
    }
}
fn main(){
    println!("{}",Solution::is_palindrome(121));
    println!("{}",Solution::is_palindrome(-121));
    println!("{}",Solution::is_palindrome(10));

}