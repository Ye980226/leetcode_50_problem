## 7. 整数反转
给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
注意：
假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−2^31,  2^31 − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。
```rust
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
```
## 8. 字符串转换整数 (atoi)
```python
class Solution:
    def myAtoi(self, s: str) -> int:
        s=s.strip()
        result=""
        count=0
        flag=False
        sign=""
        for i in s:
            if i.isdigit():
                result+=i
                flag=True
            elif (i=="+" or i=="-") and count==0 and not flag:
                sign=i
                count+=1
            else:
                break
            if (flag or count>1) and not i.isdigit():
                # print("break")
                break
        if result=="":
            return 0
        else:
            if sign =="-":
                result="-"+result
            result=int(result)
            if result>2**31-1:
                return 2**31-1
            elif result<-2**31:
                return -2**31
            else :
                return result
s=Solution()
print(s.myAtoi("+-12"))
print(s.myAtoi(".1"))
print(s.myAtoi(" 1"))
print(s.myAtoi(" .1"))
print(s.myAtoi("3.1415926"))
print(s.myAtoi(" -42"))
print(s.myAtoi("4193 with words"))
print(s.myAtoi("words and 987"))
print(s.myAtoi("-91283472332"))
print(s.myAtoi("00000-42a1234"))
print(s.myAtoi("-13+8"))
print(s.myAtoi("123-"))
```
rust版本有bug，怀疑是标准库有问题，以后再来研究
```rust
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
```
## 9. 回文数
判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。
```rust
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
```