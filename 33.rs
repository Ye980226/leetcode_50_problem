struct Solution();

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        //从左往右扫描，找到那个转折点
        let length=nums.len();
        let mut volve_pos=length-1;
        let mut pos:i32=-1;
        if length==1{
            if nums[0]==target{
                return 0;
            }
            else{
                return -1;
            }
        }
        //[0..volve_pos]递增,[volve_pos+1..length]递减
        for i in 0..length-1{
            if nums[i+1]<nums[i]{
                volve_pos=i;
            }
        }
        // println!("{:?}",&nums[0..volve_pos+1]);
        // println!("{:?}",&nums[volve_pos+1..length]);
        if target<=nums[volve_pos]&&target>=nums[0]{
            //在[0..volve_pos]二分查找
            let mut i:usize=0;
            let mut j:usize=volve_pos;
            let mut median;
            loop{
                
                // println!("{},{}",i,j);
                median=(i+j)/2;
                if nums[median]==target{
                    return median as i32;
                }else if nums[median]>target{
                    //target 可能在[i..median]
                    j=median;
                }else{
                    i=median+1;
                }
                if j-i==0 && nums[i]==target{
                    return i as i32;
                }
                if j<=i{
                    break;
                }
                let x=&nums[i ..=j];
                println!{"{:?}",x};
                
            }
            
        }else if volve_pos != length-1 &&target>=nums[volve_pos+1]&&target<=nums[length-1] {
            //在[volve_pos+1..length]二分查找
            let mut i:usize=volve_pos+1;
            let mut j:usize=length-1;
            let mut median;
            loop{
                
                // println!("{},{}",i,j);
                median=(i+j)/2;
                if nums[median]==target{
                    return median as i32;
                }else if nums[median]>target{
                    //target 可能在[i..median]
                    j=median;
                }else{
                    i=median+1;
                }
                if j-i==0 && nums[i]==target{
                    return i as i32;
                }
                if j<=i{
                    break;
                }
                // let x=&nums[i ..=j];
                // println!{"{:?}",x};
                
            }
        }
        pos

    }
}
fn main(){
    println!("{}",Solution::search(vec![4,5,6,7,0,1,2],0));
    println!("{}",Solution::search(vec![1],0));
    println!("{}",Solution::search(vec![4,5,6,7,0,1,2],3));
    println!("{}",Solution::search(vec![1,3],0));
    println!("{}",Solution::search(vec![1,3],2));
    println!("{}",Solution::search(vec![1,3],3));
}

