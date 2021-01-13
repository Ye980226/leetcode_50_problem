use std::cmp::min;
struct Solution();
// impl Solution {
//     pub fn max_area(height: Vec<i32>) -> i32 {
//         let mut max_=0;
//         let length=height.len();
//         let mut x_=0;
//         let mut area;
//         let mut min_=0;
//         let mut x_tmp;
//         let mut min_tmp;
//         for i in 0..length{
//             for j in i+1..length{
//                 x_tmp=(j-i) as i32;
//                 min_tmp=min(height[i],height[j]);
//                 if x_tmp>=x_ || min_tmp>=min_{
//                     area=min_tmp*x_tmp;
//                     if area>max_{
//                         max_=area;
//                         x_=x_tmp;
//                         min_=min_tmp;
//                     }
//                 }
                
//             }
//         }
//         max_

//     }
// }
//看了答案知道还可以用双指针
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut p=0;//左边的指针
        let mut q=height.len()-1;//右边的指针
        let mut area_:i32=0;
        let mut min_;
        let mut area_tmp;
        loop{
            //判断哪边移动，谁小谁移动
            min_=min(height[p],height[q]);
            area_tmp=(q-p) as i32*min_ ;
            if area_tmp>area_{
                area_=area_tmp;
            }
            if min_==height[p]{
                p+=1;
            }else{
                q-=1;
            }
            if p>=q{
                break;
            }
        }
        area_
    }
}
fn main(){
    println!("max :{}",Solution::max_area(vec![1,2,3]));
    println!("max :{}",Solution::max_area(vec![1,8,6,2,5,4,8,3,7]));
    println!("max :{}",Solution::max_area(vec![1,1]));
    println!("max :{}",Solution::max_area(vec![4,3,2,1,4]));
    println!("max :{}",Solution::max_area(vec![1,2,1]));
}