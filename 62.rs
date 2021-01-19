struct Solution();
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        //ways[i][j]=ways[i-1][j]+ways[i][j-1]
        let m=m as usize;
        let n=n as usize;
        let mut ways=vec![vec![1;n];m];
        // println!("{:?}",ways);

        // ways[0][1]=1;
        // ways[1][0]=1;
        for i in 1..m{
            for j in 1..n{
                ways[i][j]=ways[i-1][j]+ways[i][j-1];
                // println!("i:{},j:{},ways:{}",i,j,ways[i][j]);
            }
        }
        ways[m-1][n-1]
        
    }
}
fn main(){
    println!("{}",Solution::unique_paths(1,1));
}