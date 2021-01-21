struct Solution();
// position=true or false
// if position{
//      profits[i][j]+=prices[j]//i是之前的持仓
// }
// 

use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let length=prices.len();
        let mut profits=0;
        for i in 1..length{
            profits+=max(prices[i]-prices[i-1],0);
        }
        profits
    }  
}

fn main(){
    println!("{}",Solution::max_profit(vec![7,1,5,3,6,4]));
    println!("{}",Solution::max_profit(vec![7,1,5,3,4,6]));
    println!("{}",Solution::max_profit(vec![1,2,3,4,5]));
    println!("{}",Solution::max_profit(vec![7,6,4,3,1]));


}