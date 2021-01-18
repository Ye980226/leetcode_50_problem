struct Solution();
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        //[0][0],[0][1],[0][2],[0][3]...[0][n-1]
        //[1][n-1],[2][n-1],[2][n-1]...[m-1][n-1]
        //[m-1][n-2],...[m-1][0]
        //[m-2][0]...[]
        //[1][0]
        let m=matrix.len();
        let n=matrix[0].len();
        let mut new_order=Vec::new();
        let mut k=0;//逻辑控制
        for i in 0..m{
            if k==0{
                for j in i..n-i{
                    new_order.push(matrix[i][j]);
                    k=1;
                }
            }else{
                break;
            }
            // new_order.extend_from_slice(& matrix[i..=m-i-1][n-1-i]);
            
            // println!("{},shang:{:?}",i+1,new_order);
            
            
            if k==1{
                for j in i+1..m-i{//1,2-1
                    // println!("i:{},j:{}",i,j);
                    new_order.push(matrix[j][n-1-i]);
                    k=2;
                }
                
            }else{
                break;
            }
            // println!("{},you:{:?}",i+1,new_order);
            // println!("{:?}",matrix[i..=m-i-1][n-1-i]);
            
            
            if k==2{
                let mut tmp_n=n-i;//n=4,i=1,tmp_n=3
                while tmp_n>=i+1{
                    if tmp_n==i+1{
                        break;
                    }else{
                        tmp_n-=1;//tmp_n=2
                    }
                    new_order.push(matrix[m-i-1][tmp_n-1]);//tmp_n-1=1,0
                    k=3;
                }
                
            }else{
                break;
            }
            // println!("{},xia:{:?}",i+1,new_order);
            if k==3 {
                let mut tmp_m=m-i;
                while tmp_m>=i+2{
                    if tmp_m==i+2{
                        break;
                    }else{
                        tmp_m-=1;
                    }

                    new_order.push(matrix[tmp_m-1][i]);
                    k=0;
                }
                
            }else{
                break;
            }
            // println!("{},zuo:{:?}",i+1,new_order);
        }
        new_order

            
        // }
        // println!("{:?}",matrix[m-1..=0][n-1]);
        // vec![1,2,3]
    }

}
fn main(){
    println!("{:?}",Solution::spiral_order(vec![
        vec![1,2,3],
        vec![4,5,6],
        vec![7,8,9]
    ]));

    println!("{:?}",Solution::spiral_order(vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9,10,11,12]
    ]));
    println!("{:?}",Solution::spiral_order(vec![vec![1]]));
    println!("{:?}",Solution::spiral_order(vec![
        vec![2],
        vec![3]
    ]));
    println!("{:?}",Solution::spiral_order(vec![
        vec![7],
        vec![8],
        vec![9]
    ]));
    println!("{:?}",Solution::spiral_order(vec![
        vec![1,2,3,4],
        vec![5,6,7,8],
        vec![9,10,11,12],
        vec![13,14,15,16]
    ]));

}