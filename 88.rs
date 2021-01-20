struct Solution();
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) ->Vec<i32>{
        
        if m==0{
            return nums2.to_vec();
        }
        if n==0{
            return nums1.to_vec();
        }
        let length=(m+n) as usize;
        let mut m = m as usize;

        let mut p=0;//指向nums1
        let mut q=0;//指向nums2
        loop{
            if nums1[p]<=nums2[q]{
                p+=1;
            }else{
                let mut i = m-1;
                while i>=p{
                    nums1[i+1]=nums1[i];
                    // println!("i:{},nums1[i]:{},p:{},q:{}",i,nums1[i],p,q);
                    i-=1;
                    
                }
                nums1[i+1]=nums2[q];
                // println!("i:{},nums1[i]:{},p:{},q:{}",i,nums1[i],p,q);
                q+=1;
                m+=1;
            }
            // println!("2:{:?}",nums1);
            if p==m{
                for i in p..length{
                    nums1[i]=nums2[q];
                    q+=1;
                }
                break;
            }
            // println!("3:{:?}",nums1);
            if q>=m{
                break;
            }
        }
    nums1.to_vec()
    }
}
fn main(){
    println!("{:?}",Solution::merge(&mut vec![1,2,3,0,0,0],3,&mut vec![2,5,6],3));
    println!("{:?}",Solution::merge(&mut vec![0,0,0,0,0,0],3,&mut vec![2,5,6],3));
    println!("{:?}",Solution::merge(&mut vec![1],1,&mut vec![],0));
    println!("{:?}",Solution::merge(&mut vec![0],0,&mut vec![1],1));
}