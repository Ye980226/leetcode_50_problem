struct Solution();
use std::cmp::{max,min};
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // let mut p=0;//p指向第一个元素
        // let mut q=p+1;//q指向第二个元素
        let length=prices.len();
        let mut max_:i32=0;
        let mut min_:i32=100000;
        // loop{
        //     if p>=length-1{
        //         break;
        //     }
        //     if prices[p+1]<=prices[p]{
        //         p+=1;
        //         q+=1;
        //         continue;
        //     }
        //     loop{
        //         if prices[p]>=prices[q]{
        //             q+=1;
        //         }else{
        //             max_=max(prices[q]-prices[p],max_);
        //             q+=1;
        //         }
        //         if q>length-1{
        //             p+=1;
        //             break;
        //         }
        //     }
        //     q=p+1;
        // }
        for i in 0..length{
            max_=max(prices[i]-min_,max_);
            min_=min(prices[i],min_);
            // println!("max:{},min:{}",max_,min_);
        }
        max_
    }
}
fn main(){
    println!("{}",Solution::max_profit(vec![7,1,5,3,6,4]));
    println!("{}",Solution::max_profit(vec![7,6,4,3,1]));
    println!("{}",Solution::max_profit(vec![3,2,6,5,0,3]));

}