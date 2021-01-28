struct Solution();
impl Solution {
    pub fn can_win_nim(n: i32) -> bool {
        // let n = n as usize;
        // let mut win_nim_vec=vec![true;n];
        // if n==1||n==2||n==3{
        //     return true;
        // }
        // let mut first=true;
        // let mut second=true;
        // let mut third=true;
        // let mut fourth=true;
        // for _ in 3..n{
            // fourth=!(first&second&third);
            // first=second;
            // second=third;
            // third=fourth;
        // }
        // fourth
        // if n==1{
        //     return true;
        // }
        // let mod_=((n-1)/3)%2;//如果是5,5/3=1%2=1,如果是7/3=2,2%2=0,也就是mod_取0的时候返回true，否则返回false,如果是3,3/3=1%2=1，这是不对的
        // if mod_==0{
        //     true
        // }else{
        //     false
        // }
    //打印出来看，发现4的倍数是false，否则就是true
    // if n%4==0{
    //     false
    // }else{
    //     true
    // }
    //更节省内存
    n%4!=0
    }
}
fn main(){
    // println!("{}",Solution::can_win_nim(201782680));
    // println!("{}",Solution::can_win_nim(655678107
    // ));
    println!("{}",Solution::can_win_nim(30));
    
}