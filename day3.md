## 11. 盛最多水的容器
给你 n 个非负整数 a_1，a_2，...，a_n，每个数代表坐标中的一个点 (i, a_i) 。在坐标内画 n 条垂直线，垂直线 i 的两个端点分别为 (i, a_i) 和 (i, 0) 。找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。

说明：你不能倾斜容器。
```rust
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
```
## 14. 最长公共前缀
编写一个函数来查找字符串数组中的最长公共前缀。

如果不存在公共前缀，返回空字符串 ""。
```rust
use std::cmp::min;
struct Solution();
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let length=strs.len();
        if length==0{
            return String::from("");
        }
        let mut longest=String::from(&strs[0]);
        
        let mut min_str_length;
        let mut longest_tmp=String::from("");
        let mut p_str:String;
        let mut q_str:String;
        for i in 1..length{
            min_str_length=min(longest.len(),strs[i].len());
            let p_str=match longest.get(..min_str_length){
                Some(value)=>value.to_string(),
                None=>"".to_string(),
            };
            let q_str=match strs[i].get(..min_str_length){
                Some(value)=>value.to_string(),
                None=>"".to_string(),
            };
            
            for (l,s) in p_str.chars().zip(q_str.chars())
            {
                if l != s{
                    break;
                }
                
                longest_tmp.push(l);
            }
            longest=longest_tmp;
            longest_tmp=String::from("");
        }
        longest
    }
}
fn main(){
    println!("{}",Solution::longest_common_prefix(vec![String::from("flower"),String::from("flow"),String::from("flight")]));
    println!("{}",Solution::longest_common_prefix(vec![String::from("dog"),String::from("racecar"),String::from("car")]));
}
```
## 15. 三数之和
给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有和为 0 且不重复的三元组。

注意：答案中不可以包含重复的三元组。

```rust
struct Solution();
// struct ThreeNum{
//     first:usize,
//     second:usize,
//     third:i32,
// }
// impl Solution {
//     pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
//         let mut three_num_vec:Vec<ThreeNum>=Vec::new();
//         let length =nums.len();
//         if length<3{
//             return vec![];
//         }
//         for i in 0..length{
//             for j in i+1..length{
//                 three_num_vec.push(ThreeNum{first:i,second:j,third:-(nums[i]+nums[j])});
//             }
//         }
//         let mut capacity:Vec<Vec<i32>>=Vec::new();
//         for i in 0..length{
//             for j in 0..three_num_vec.len(){
//                 let three_num=&three_num_vec[j];
//                 if three_num.third==nums[i] && three_num.first<i  &&i > three_num.second{
//                     let mut vec_tmp=vec![nums[three_num.first],nums[three_num.second],three_num.third];
//                     vec_tmp.sort();
//                     if !capacity.contains(&vec_tmp){
//                         capacity.push(vec_tmp);
//                     }
//                 }
//             }
//         }
//         capacity.sort();
//         capacity 
//     }
// }//超时
//根据答案是先排序，再双指针
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums_sort:Vec<i32>=Vec::new();
        let length =nums.len();
        let mut sum_;
        let mut k;
        let mut j;
        if length<3{
            return vec![];
        }
        nums_sort.extend_from_slice(&nums);
        let mut capacity:Vec<Vec<i32>>=Vec::new();
        nums_sort.sort();
        //third的坐标是k
        for i in 0..length{
            k=length-1;
            j=i+1;
            loop{
                //如果j＞=k就退出
                
                // println!("{},{},{},{}",i,j,k,sum_);
                
                if j>=k{
                    break;
                }
                sum_=nums_sort[i]+nums_sort[j]+nums_sort[k];
                if sum_>0{
                    k-=1;
                }else if sum_<0{
                    j+=1;
                }else{
                    capacity.push(vec![nums_sort[i],nums_sort[j],nums_sort[k]]);
                    k-=1;
                    j+=1;
                }
            }   
            
        }
        capacity.sort();
        capacity.dedup();
        capacity
    }
}
fn main(){
    println!("{:?}",Solution::three_sum(vec![-1,0,1,2,-1,-4]));
    println!("{:?}",Solution::three_sum(vec![]));
    println!("{:?}",Solution::three_sum(vec![0]));
    println!("{:?}",Solution::three_sum(vec![0,0,0,0]));
    println!("{:?}",Solution::three_sum(vec![3,0,-2,-1,1,2]));
    println!("{:?}",Solution::three_sum(vec![-4,-2,-2,-2,0,1,2,2,2,3,3,4,4,6,6]));

    // println!("{:?}",Solution::three_sum(vec![vec![]]));
}

```