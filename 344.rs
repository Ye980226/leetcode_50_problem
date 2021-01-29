struct Solution();
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>){
        if s.len()<=1{
            return ;
        }
        let n = s.len();
        let mut idx;
        for i in 0..n/2{
            idx=n-i-1;
            s.swap(i,idx);
        }
        // let mut p=0;
        // let mut q=s.len()-1;
        // loop{
        //     if p>=q{
        //         break;
        //     }else{
        //         s.swap(p,q);
        //         q-=1;
        //         p+=1;
        //     }
        // }
        // s.clone()
    }
}
fn main(){
    println!("{:?}",Solution::reverse_string(&mut vec!['h','e','l','l','o']));
    println!("{:?}",Solution::reverse_string(&mut vec!['h']));
    println!("{:?}",Solution::reverse_string(&mut vec![]));
}