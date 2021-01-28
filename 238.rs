struct Solution();
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    //     let length = nums.len();
    //     let mut except_self_vec=vec![1;length];
    //     for i in 0..length{
    //         for j in 0..length{
    //             if i != j{
    //                 except_self_vec[i]*=nums[j];
    //             }
    //         }
    //     }
    //     except_self_vec
    // }
        // let length=nums.len();
        // let multiply=nums.iter().product();
        // let mut except_self_vec=vec![multiply;length];
        // for i in 0..length{
        //     except_self_vec[i]/=nums[i];
        // }
        // except_self_vec
        if nums.is_empty() { return Vec::new(); }//这一句加了好多速度，看来0作为边界值的判断，会拖慢很多检测。
        let length=nums.len();
        let mut except_self_vec=vec![1;length];
        let mut r=1;
        for i in 1..length{
            except_self_vec[i]=except_self_vec[i-1]*nums[i-1];
        }
        for i in (0..length).rev()
        {
            except_self_vec[i]*=r;
            r*=nums[i];
        }
        except_self_vec
    }
}
fn main(){
    println!("{:?}",Solution::product_except_self(vec![1,2,3,4]));
}