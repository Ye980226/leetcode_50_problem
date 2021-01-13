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
