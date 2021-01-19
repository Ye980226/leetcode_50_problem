struct Solution();
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1{
            return 1;
        }
        let n = n as usize;
        let mut stairs=vec![1;n];
        stairs[1]=2;
        for i in 2..n{
            stairs[i]=stairs[i-1]+stairs[i-2];
        }
        stairs[n-1]
    }
}
fn main(){
    println!("{}",Solution::climb_stairs(2));
    println!("{}",Solution::climb_stairs(3));
    println!("{}",Solution::climb_stairs(1));

}