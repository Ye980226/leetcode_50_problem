struct Solution();
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let length=nums.len();
        let mut j;
        let mut k;
        let mut num_closest=i32::MAX;
        let mut result=0;
        let mut nums=nums;
        let mut sum_;
        nums.sort();//先排序
        for i in 0..length{
            j=i+1;//j指向第二个元素
            k=length-1;   //k指向第三个元素
            
            loop{
                
                if j>=k{
                    break;
                }
                sum_=nums[i]+nums[j]+nums[k];
                if sum_>target{
                    k-=1;
                }else if sum_<target{
                    j+=1;
                }else{
                    return sum_
                }
                if i32::abs(sum_-target)<num_closest{
                    num_closest=i32::abs(sum_-target);
                    result=sum_;
                }

            }
            
        }
        result
    }
}
fn main(){
    println!("{}",Solution::three_sum_closest(vec![-1,2,1,-4],1));
}