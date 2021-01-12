struct Solution();
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut result=String::from("");
        let mut r_prime =s;
        //判断第一个是不是数字
        let mut flag=0;
        let mut sign=' ';
        let mut sign_flag=true;
        if let Some(value)=r_prime.strip_prefix(&[' '][..]){
            r_prime=value.to_string();
        }
        println!("{}",r_prime);
        for c in r_prime.chars(){
            if  c=='+'||c=='-'||c==' '{
                sign=c;

            }else if !c.is_digit(10){
                break;
            }else if c.is_digit(10){
                flag+=1;
                result.push(c);
            }

        }
        if result.is_empty(){
            0
        }else{
            let mut a=result.parse::<i64>().unwrap();
            if a>i32::MAX as i64{
                i32::MAX
            }else if a<i32::MIN as i64{
                i32::MIN
            }else{
                a as i32
            }

        }
    }
}
fn main(){
    let a=Solution::my_atoi(String::from("      .1"));
    let b=Solution::my_atoi(String::from(".1"));
    println!("{}",a);
    println!("{}",b);
    println!("{}",Solution::my_atoi(String::from("   -+12   ")));
    println!("{}",Solution::my_atoi(String::from("3.1415926")));
    println!("{}",Solution::my_atoi(String::from("words and 987")));
    println!("{}",Solution::my_atoi(String::from("-91283472332")));
    println!("{}",Solution::my_atoi(String::from("  -42")));
}