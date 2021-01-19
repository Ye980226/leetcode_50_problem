## 62. 不同路径
一个机器人位于一个` m x n `网格的左上角 （起始点在下图中标记为 “Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish” ）。

问总共有多少条不同的路径？
```rust
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
```
时间复杂度：O(mn)

空间复杂度：O(mn)

用的是动态规划思想

每多走一步就是多一条路径，所以初始值全部设为1

理论上还有初值
ways[0][0]=0,但如果从1开始循环，发现完全用不上，就不用多此一举

状态转移矩阵

ways[i][j]=ways[i-1][j]+ways[i][j-1]

最后执行结果是
![62.不同路径](./62.jpg "62.不同路径")

## 70.爬楼梯
假设你正在爬楼梯。需要 n 阶你才能到达楼顶。

每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？

注意：给定 n 是一个正整数。

```rust
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1{
            return 1;
        }
        let n = n as usize;
        let mut stairs=vec![1;n];
        stairs[1]=2;
        for i in 2..n{
            stairs[i]=stairs[i-1]+stairs[i-2];
        }
        stairs[n-1]
    }
}
```
时间复杂度：O(n)

空间复杂度:O(n)

初值:

stairs[0]=1
,stairs[1]=2

状态转移矩阵

stairs[i]=stairs[i-1]+stairs[i-2];

最后执行结果是
![70.爬楼梯](./70.jpg "70.爬楼梯")

## 78. 子集
给你一个整数数组 `nums `，返回该数组所有可能的子集（幂集）。解集不能包含重复的子集。
```rust
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let length=nums.len();
        //初始化一个2**n-1的空[]
        let mut subset_vec:Vec<Vec<i32>>=vec![vec![];2_usize.pow(length as u32)-1];
        //初值是[]
        subset_vec.push(vec![]);
        //记录每次复制的长度
        let mut subset_length=1;
        
        for i in 0..length{
            //rust由于所有权模型必须用如下手段克隆
            let num=nums[i];
            {
                //将subset_vec分成左右两个部分，左边长度是subset_length
                let (left,right)=subset_vec.split_at_mut(subset_length);
                //将左边部分，复制到右边
                right[0..subset_length].clone_from_slice(&left[0..subset_length])
            }
            // println!("{:?}",subset_vec);
            for j in subset_length..2*subset_length{
                //将复制到右边的部分，后面都加上一个nums[i]
                subset_vec[j].push(num);
            }
            // println!("{:?}",subset_vec);
            //更新subset_length的长度
            subset_length*=2;
            //提前结束
            if subset_length>= subset_vec.len(){
                break;
            }
            // for j in 0..subset_vec.len(){

            // }
        }
        subset_vec
    }
}
```

时间复杂度：O(2^n)

外层循环，循环到第n次时，内层循环，循环2^n，所以是2^1+2^2+...+2^n=2^(n+1)-2=2*2^n-2,所以时间复杂度是O(2^n)

空间复杂度: O(2^n)

只有在一开始初始化了2^n-1长度的Vector，之后都是在原地操作

本题主要思想也是动态规划，利用前面的来更新后面的，由于rust本身语言模型的限制，所以可能代码有一些不优雅，但查了官方手册，发现只能用这种方式，来实现复制

最后执行结果是
![78.子集](./78.jpg "78.子集 ")