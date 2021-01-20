## 88. 合并两个有序数组
给你两个有序整数数组 `nums1` 和 `nums2`，请你将 `nums2` 合并到 `nums1` 中，使 `nums1` 成为一个有序数组。

初始化 `nums1` 和 `nums2` 的元素数量分别为 `m` 和 `n` 。你可以假设 `nums1` 的空间大小等于 `m + n`，这样它就有足够的空间保存来自 `nums2` 的元素。
```rust
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
    println!("{:?}",Solution::merge(&mut vec![1],1,&mut vec![],0));
    println!("{:?}",Solution::merge(&mut vec![0],0,&mut vec![1],1));
}
```

时间复杂度：O(m+n)

空间复杂度：O(1)

LeetCode结果与本地不同，此题LeetCode的rust代码有问题，给的函数也没有返回值，所以rust提交没过，没有结果截图



## 89. 格雷编码
格雷编码是一个二进制数字系统，在该系统中，两个连续的数值仅有一个位数的差异。

给定一个代表编码总位数的非负整数 n，打印其格雷编码序列。即使有多个不同答案，你也只需要返回其中一种。

格雷编码序列必须以 0 开头。
```rust
struct Solution();
impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        //动态规划
        let mut gray_vec=Vec::new();
        
        gray_vec.push(0);
        // if n==0{
        //     return gray_vec;
        // }
        // gray_vec.push(1);
        // if n==1{
        //     return gray_vec;
        // }
        for i in 1..n+1{
            let mut length=gray_vec.len();
            while length>0{
                length-=1;
                // println!("{}",1<<(i-1));
                // println!("{}",gray_vec[length]);
                gray_vec.push(gray_vec[length]+(1<<(i-1)));
            }
        }
        gray_vec
    }
}
fn main(){
    println!("{:?}",Solution::gray_code(2));
    println!("{:?}",Solution::gray_code(0));
    println!("{:?}",Solution::gray_code(1));
    println!("{:?}",Solution::gray_code(3));
}
```

时间复杂度:O(2^n)

空间复杂度:O(1)

运行结果

![89.格雷编码](89.jpg "89.格雷编码")

## 104.二叉树的最大深度
给定一个二叉树，找出其最大深度。

二叉树的深度为根节点到最远叶子节点的最长路径上的节点数。

说明: 叶子节点是指没有子节点的节点。

```python
# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution:
    def maxDepth(self, root: TreeNode) -> int: 
        
        if not root:
            return 0
        else:
            return  1+max(self.maxDepth(root.left),self.maxDepth(root.right))
tree=TreeNode(3,TreeNode(9,None,None),TreeNode(20,TreeNode(15),TreeNode(7)))
s=Solution()
print(s.maxDepth(tree))
```

时间复杂度:O(n)

空间复杂度:O(maxDepth)