## 16. 最接近的三数之和
给定一个包括 `n` 个整数的数组 `nums` 和 一个目标值 `target`。找出 `nums` 中的三个整数，使得它们的和与 `target` 最接近。返回这三个数的和。假定每组输入只存在唯一答案。

```rust
struct Solution();
impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let length=nums.len();
        let mut j;
        let mut k;
        let mut num_closest=i32::MAX;
        let mut result=0;
        let mut nums=nums;
        let mut sum_;
        nums.sort();//先排序
        for i in 0..length{
            j=i+1;//j指向第二个元素
            k=length-1;   //k指向第三个元素
            
            loop{
                
                if j>=k{
                    break;
                }
                sum_=nums[i]+nums[j]+nums[k];
                if sum_>target{
                    k-=1;
                }else if sum_<target{
                    j+=1;
                }else{
                    return sum_
                }
                if i32::abs(sum_-target)<num_closest{
                    num_closest=i32::abs(sum_-target);
                    result=sum_;
                }

            }
            
        }
        result
    }
}
fn main(){
    println!("{}",Solution::three_sum_closest(vec![-1,2,1,-4],1));
}
```
## 20. 有效的括号
给定一个只包括`'('`，`')'`，`'{'`，`'}'`，`'['`，`']'` 的字符串，判断字符串是否有效。

有效字符串需满足：

左括号必须用相同类型的右括号闭合。
左括号必须以正确的顺序闭合。
注意空字符串可被认为是有效字符串。

```rust
struct Solution();
use std::collections::HashMap;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut right_left=HashMap::new();
        right_left.insert(')','(');
        right_left.insert(']','[');
        right_left.insert('}','{');
        let mut record=Vec::new();
        let length=s.len();
        // let mut flag=true;
        let chars=s.chars().collect::<Vec<char>>();
        for i in 0..length{
            let c= chars[i];
            if c=='(' || c=='[' || c=='{'{
                record.push(c);
            }else{
                // println!("{:?}",record);
                if let Some(left)=right_left.get(&c)
                {
                        
                    match record.pop()
                    {
                        Some(value)=>
                        {
                            if value!=*left
                            {
                                return false;
                            }
                        }
                        None=>
                        {
                            return false;
                        }
                    }
                    
                }
               
            }
        }
        if record.len()>0{
            false
        }else{
            true
        }
    }
}
fn main(){
    println!("{}",Solution::is_valid(String::from("(((([]{}")));
    println!("{}",Solution::is_valid(String::from("()")));
    println!("{}",Solution::is_valid(String::from("()[]{}")));
    println!("{}",Solution::is_valid(String::from("([)]")));
    println!("{}",Solution::is_valid(String::from("{[]}")));
    println!("{}",Solution::is_valid(String::from("]")));
    println!("{}",Solution::is_valid(String::from("((")));
    println!("{}",Solution::is_valid(String::from("(([]){})")));
    
}
```
## 21. 合并两个有序链表
将两个升序链表合并为一个新的 `升序` 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 
```python
class Solution:
    def mergeTwoLists(self, l1, l2):
        if l1 is None:
            return l2
        elif l2 is None:
            return l1
        elif l1.val < l2.val:
            l1.next = self.mergeTwoLists(l1.next, l2)
            return l1
        else:
            l2.next = self.mergeTwoLists(l1, l2.next)
            return l2
```
