## 23. 合并K个升序链表
给你一个链表数组，每个链表都已经按升序排列。
请你将所有链表合并到一个升序链表中，返回合并后的链表。
```python
class Solution:
    def mergeKLists(self, lists: List[ListNode]) -> ListNode:
        """分治合并"""
        def merge2Lists(list1, list2):
            """list1和list2分别是两个链表的头结点"""
            head = ListNode(0)  # 新建一个头节点用来返回list1和list2的排序后结果
            dummy = head  # 指向head链表的尾部，用来拆入新节点
            while list1 and list2:  # 两个链表都不为空时
                if list1.val < list2.val:
                    dummy.next = list1  # 插入新节点
                    dummy = list1  # dummy指向新插入的节点
                    list1 = list1.next  # 移动list1
                    continue
                dummy.next = list2
                dummy = list2
                list2 = list2.next
            dummy.next = list2 if not list1 else list1  # 如果list1为None，说明list1中的元素全部进入了head链表，所以把list2中剩余的元素直接添加进来，反之亦然。
            return head.next
        # print(merge2Lists(lists[1], lists[2]))

        res = None
        def divideMergeKLists(lists):
            """分治归并k个链表"""
            nonlocal res
            k = len(lists)
            temp = []

            if k == 0: return  # lists为空，直接结束
            if k == 1 :  # 归并结束
                res = lists[0]  # 保存结果
                return
            for i in range(0, k, 2):
                if i+1 == k:  # k为奇数，i的最后一个取值是k-1，此时i+1已经超出范围
                    temp.append(lists[i])
                    break
                # 如果k为偶数，i的最后一个取值永远是k-2
                temp.append(merge2Lists(lists[i], lists[i+1]))
            divideMergeKLists(temp)

        divideMergeKLists(lists)
        return res
```

## 26. 删除排序数组中的重复项
给定一个排序数组，你需要在` 原地 `删除重复出现的元素，使得每个元素只出现一次，返回移除后数组的新长度。

不要使用额外的数组空间，你必须在` 原地 ``修改输入数组` 并在使用 O(1) 额外空间的条件下完成。
```rust
struct Solution();
// use std::collections::HashSet;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        //因为数组已经排序过了，所以可以不需要
        let length=nums.len();
        if length<2{
            return length as i32;
        }
        let mut i:usize=0;
        loop{
            if i>=nums.len()-1{
                break;
            }
            if nums[i]==nums[i+1]{
                nums.remove(i+1);
                // println!("{:?}",nums);
                continue;
            }
            i+=1;
        }
        nums.len() as i32
    }
}
fn main(){
    println!("{}",Solution::remove_duplicates(&mut vec![1,1,2]));
    println!("{}",Solution::remove_duplicates(&mut vec![0,0,1,1,1,2,2,3,3,4]));
    println!("{}",Solution::remove_duplicates(&mut vec![]));
    println!("{}",Solution::remove_duplicates(&mut vec![1]));
}
```
## 33. 搜索旋转排序数组
升序排列的整数数组 nums 在预先未知的某个点上进行了旋转（例如， `[0,1,2,4,5,6,7]` 经旋转后可能变为 `[4,5,6,7,0,1,2] `）。

请你在数组中搜索` target `，如果数组中存在这个目标值，则返回它的索引，否则返回` -1 `。
```rust
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        //从左往右扫描，找到那个转折点
        let length=nums.len();
        let mut volve_pos=length-1;
        let mut pos:i32=-1;
        if length==1{
            if nums[0]==target{
                return 0;
            }
            else{
                return -1;
            }
        }
        //[0..volve_pos]递增,[volve_pos+1..length]递减
        for i in 0..length-1{
            if nums[i+1]<nums[i]{
                volve_pos=i;
            }
        }
        // println!("{:?}",&nums[0..volve_pos+1]);
        // println!("{:?}",&nums[volve_pos+1..length]);
        if target<=nums[volve_pos]&&target>=nums[0]{
            //在[0..volve_pos]二分查找
            let mut i:usize=0;
            let mut j:usize=volve_pos;
            let mut median;
            loop{
                
                // println!("{},{}",i,j);
                median=(i+j)/2;
                if nums[median]==target{
                    return median as i32;
                }else if nums[median]>target{
                    //target 可能在[i..median]
                    j=median;
                }else{
                    i=median+1;
                }
                if j-i==0 && nums[i]==target{
                    return i as i32;
                }
                if j<=i{
                    break;
                }
                let x=&nums[i ..=j];
                println!{"{:?}",x};
                
            }
            
        }else if volve_pos != length-1 &&target>=nums[volve_pos+1]&&target<=nums[length-1] {
            //在[volve_pos+1..length]二分查找
            let mut i:usize=volve_pos+1;
            let mut j:usize=length-1;
            let mut median;
            loop{
                
                // println!("{},{}",i,j);
                median=(i+j)/2;
                if nums[median]==target{
                    return median as i32;
                }else if nums[median]>target{
                    //target 可能在[i..median]
                    j=median;
                }else{
                    i=median+1;
                }
                if j-i==0 && nums[i]==target{
                    return i as i32;
                }
                if j<=i{
                    break;
                }
                // let x=&nums[i ..=j];
                // println!{"{:?}",x};
                
            }
        }
        pos

    }
}
```