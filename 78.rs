struct Solution();
impl Solution {
    //vecs[0]=[]
    //vecs[i]=[].extend_from_slice(&nums[i-1]).push(nums[i])
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let length=nums.len();
        let mut subset_vec:Vec<Vec<i32>>=vec![vec![];2_usize.pow(length as u32)-1];
        // let mut subset_vec:Vec<Vec<i32>>=Vec::new();
        subset_vec.push(vec![]);
        let mut subset_length=1;
        // let subset_length=subset_vec.len();
        for i in 0..length{
            // if i<length{
            //     subset_vec[i].push(nums[i-1]);
            // }else{
            //     subset_vec[i].extend_from_slice(subset_vec[i-length]).
            // }
            let num=nums[i];
            // println!("{}",num);
            // subset_vec.extend_from_slice()
            // subset_vec[subset_length..subset_length*2].copy_from_slice(& subset_vec[0..subset_length]);
            {
                let (left,right)=subset_vec.split_at_mut(subset_length);
                right[0..subset_length].clone_from_slice(&left[0..subset_length])
            }
            // println!("{:?}",subset_vec);
            for j in subset_length..2*subset_length{
                subset_vec[j].push(num);
            }
            // println!("{:?}",subset_vec);
            subset_length*=2;
            if subset_length>= subset_vec.len(){
                break;
            }
            // for j in 0..subset_vec.len(){

            // }
        }
        subset_vec


    }
}
fn main(){
    println!("{:?}",Solution::subsets(vec![1,2,3]));
    println!("{:?}",Solution::subsets(vec![1]));
    println!("{:?}",Solution::subsets(vec![]));
}