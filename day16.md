## 237.删除链表中的节点

请编写一个函数，使其可以删除某个链表中给定的（非末尾）节点。传入函数的唯一参数为 要被删除的节点 。

 

现有一个链表 -- head = [4,5,1,9]

```python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def deleteNode(self, node):
        """
        :type node: ListNode
        :rtype: void Do not return anything, modify node in-place instead.
        """
        node.val = node.next.val
        node.next = node.next.next
```

## 238.除自身以外数组的乘积

给你一个长度为 n 的整数数组 nums，其中 n > 1，返回输出数组 output ，其中 output[i] 等于 nums 中除 nums[i] 之外其余各元素的乘积。

```rust
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    //     let length = nums.len();
    //     let mut except_self_vec=vec![1;length];
    //     for i in 0..length{
    //         for j in 0..length{
    //             if i != j{
    //                 except_self_vec[i]*=nums[j];
    //             }
    //         }
    //     }
    //     except_self_vec
    // }
        // let length=nums.len();
        // let multiply=nums.iter().product();
        // let mut except_self_vec=vec![multiply;length];
        // for i in 0..length{
        //     except_self_vec[i]/=nums[i];
        // }
        // except_self_vec
        if nums.is_empty() { return Vec::new(); }//这一句加了好多速度
        let length=nums.len();
        let mut except_self_vec=vec![1;length];
        let mut r=1;
        for i in 1..length{
            except_self_vec[i]=except_self_vec[i-1]*nums[i-1];
        }
        for i in (0..length).rev()
        {
            except_self_vec[i]*=r;
            r*=nums[i];
        }
        except_self_vec
    }
}
```

## 292.Nim游戏

你和你的朋友，两个人一起玩 Nim 游戏：

桌子上有一堆石头。
你们轮流进行自己的回合，你作为先手。
每一回合，轮到的人拿掉 1 - 3 块石头。
拿掉最后一块石头的人就是获胜者。
假设你们每一步都是最优解。请编写一个函数，来判断你是否可以在给定石头数量为 n 的情况下赢得游戏。如果可以赢，返回 true；否则，返回 false 。

```rust
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
```

这题目让我感觉我是小丑。。。