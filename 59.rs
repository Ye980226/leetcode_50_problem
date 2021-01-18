struct Solution();
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        //[0][0],[0][1],[0][2],[0][3]...[0][n-1]
        //[1][n-1],[2][n-1],[2][n-1]...[m-1][n-1]
        //[m-1][n-2],...[m-1][0]
        //[m-2][0]...[]
        //[1][0]
        let n=n as usize;
        let m=n ;
        let mut new_order=vec![
            vec![0;n];n
            ];
        let mut k=0;//逻辑控制
        let mut count=1;
        for i in 0..m{
            if k==0{
                for j in i..n-i{
                    new_order[i][j]=count;
                    count+=1;
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
                    new_order[j][n-1-i]=count;
                    count+=1;
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
                    new_order[m-i-1][tmp_n-1]=count;//tmp_n-1=1,0
                    count+=1;
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

                    new_order[tmp_m-1][i]=count;
                    count+=1;
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
    println!("{:?}",Solution::generate_matrix(3));
    println!("{:?}",Solution::generate_matrix(4));

}