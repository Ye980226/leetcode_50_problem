## 54. 螺旋矩阵
给定一个包含 m x n 个元素的矩阵（m 行, n 列），请按照顺时针螺旋顺序，返回矩阵中的所有元素。
```rust
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
```
## 59. 螺旋矩阵 II
给定一个正整数 n，生成一个包含 1 到 n2 所有元素，且元素按顺时针顺序螺旋排列的正方形矩阵。
```rust
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
```
## 61. 旋转链表
给定一个链表，旋转链表，将链表每个节点向右移动 k 个位置，其中 k 是非负数。
```rust
impl Solution {
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // rust 链表无法成环，拆成两个链表再连接
        if head.is_none() {
            return head;
        }
        let mut len = 0;
        let mut head_refer = &head;
        while head_refer.as_ref().is_some() {
            // 记录链表长度
            len += 1;
            head_refer = &head_refer.as_ref()?.next;
        }
        let offset = k % len;
        if offset == 0 {
            return head;
        }
        let mut head_mut_refer = &mut head;
        while len - offset != 1 {
            len -= 1;
            head_mut_refer = &mut head_mut_refer.as_mut()?.next;
        }
        // right = 4->5  head = 1->2->3
        let mut right = head_mut_refer.as_mut()?.next.take();
        let mut right_mut_refer = &mut right;
        while right_mut_refer.as_ref()?.next.is_some() {
            // 找到 right 的尾部
            right_mut_refer = &mut right_mut_refer.as_mut()?.next;
        }
        right_mut_refer.as_mut()?.next = head;
        right
    }
}
```