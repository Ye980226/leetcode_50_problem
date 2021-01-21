## 121. 买卖股票的最佳时机
给定一个数组，它的第 i 个元素是一支给定股票第 i 天的价格。

如果你最多只允许完成一笔交易（即买入和卖出一支股票一次），设计一个算法来计算你所能获取的最大利润。

注意：你不能在买入股票前卖出股票。
```rust
struct Solution();
use std::cmp::{max,min};
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // let mut p=0;//p指向第一个元素
        // let mut q=p+1;//q指向第二个元素
        let length=prices.len();
        let mut max_:i32=0;
        let mut min_:i32=100000;
        // loop{
        //     if p>=length-1{
        //         break;
        //     }
        //     if prices[p+1]<=prices[p]{
        //         p+=1;
        //         q+=1;
        //         continue;
        //     }
        //     loop{
        //         if prices[p]>=prices[q]{
        //             q+=1;
        //         }else{
        //             max_=max(prices[q]-prices[p],max_);
        //             q+=1;
        //         }
        //         if q>length-1{
        //             p+=1;
        //             break;
        //         }
        //     }
        //     q=p+1;
        // }
        for i in 0..length{
            max_=max(prices[i]-min_,max_);
            min_=min(prices[i],min_);
            // println!("max:{},min:{}",max_,min_);
        }
        max_
    }
}
fn main(){
    println!("{}",Solution::max_profit(vec![7,1,5,3,6,4]));
    println!("{}",Solution::max_profit(vec![7,6,4,3,1]));
    println!("{}",Solution::max_profit(vec![3,2,6,5,0,3]));

}
```

## 122. 买卖股票的最佳时机 II
给定一个数组，它的第 i 个元素是一支给定股票第 i 天的价格。

设计一个算法来计算你所能获取的最大利润。你可以尽可能地完成更多的交易（多次买卖一支股票）。

注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
```rust
struct Solution();
// position=true or false
// if position{
//      profits[i][j]+=prices[j]//i是之前的持仓
// }
// 

use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let length=prices.len();
        let mut profits=0;
        for i in 1..length{
            profits+=max(prices[i]-prices[i-1],0);
        }
        profits
    }  
}

fn main(){
    println!("{}",Solution::max_profit(vec![7,1,5,3,6,4]));
    println!("{}",Solution::max_profit(vec![7,1,5,3,4,6]));
    println!("{}",Solution::max_profit(vec![1,2,3,4,5]));
    println!("{}",Solution::max_profit(vec![7,6,4,3,1]));


}
```
## 124. 二叉树中的最大路径和

路径 被定义为一条从树中任意节点出发，沿父节点-子节点连接，达到任意节点的序列。该路径 至少包含一个 节点，且不一定经过根节点。

路径和 是路径中各节点值的总和。

给你一个二叉树的根节点 root ，返回其 最大路径和 。

示例 1：

![img](exx1.jpg)

输入：root = [1,2,3]
输出：6
解释：最优路径是 2 -> 1 -> 3 ，路径和为 2 + 1 + 3 = 6
示例 2：

![img](exx2.jpg)

输入：root = [-10,9,20,null,null,15,7]
输出：42
解释：最优路径是 15 -> 20 -> 7 ，路径和为 15 + 20 + 7 = 42

```python
# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution:
    def __init__(self):
        self.max_=-2**32+1
    def maxPathSum(self, root: TreeNode) -> int:
        
        def maxPathSumTmp(root:TreeNode)->int:
            max_=0
            if not root:
                return 0
            else:
                
                left=max(0,maxPathSumTmp(root.left))
                right=max(0,maxPathSumTmp(root.right))
                lmr=root.val+left+right
                ret=root.val+max(left,right)
                # max_=max(tmp_rslt,max_)
                # max_
                # self.max_s.append(max_)
                # max_=0
                # max_=max(self.maxPathSum(root.left),max_)
                # self.max_s.append(max_)
                # max_=0
                # max_=max(self.maxPathSum(root.right),max_)
                # self.max_s.append(max_)
                self.max_=max(lmr,ret,self.max_)

                # print("val",root.val)
                # print("max_",max_)
                
            return ret
        maxPathSumTmp(root)
        return self.max_
tree=TreeNode(-10,TreeNode(9),TreeNode(20,TreeNode(15),TreeNode(7)))
s=Solution()
print(s.maxPathSum(tree))

```