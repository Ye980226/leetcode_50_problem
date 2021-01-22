## 136. 只出现一次的数字
给定一个非空整数数组，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。

说明：

你的算法应该具有线性时间复杂度。 你可以不使用额外空间来实现吗？
```rust
struct Solution();
// use std::collections::HashMap;
impl Solution {
    // //
    pub fn single_number(nums: Vec<i32>) -> i32 {
    //     let length=nums.len();
    //     let mut num_count=HashMap::new();
    //     for i in 0..length{
    //         match num_count.remove(&nums[i]){
    //             Some(_value)=>{},
    //             None=>{num_count.insert(nums[i],1);},   
    //         }
    //     }
    //     if let Some(&value)=num_count.keys().next(){
    //         return value;
    //     }
    //     nums[0]

    // }
    //异或可以直接判断
        let length=nums.len();
        let mut rslt=0;//0^任何数=任何数
        //同样的数取异或=0
        for i in 0..length{
            rslt^=nums[i];
        }
        rslt
    }
}
fn main(){
    println!("{}",Solution::single_number(vec![2,2,1]));
    println!("{}",Solution::single_number(vec![4,1,2,1,2]));
}
```

## 141. 环形链表
给定一个链表，判断链表中是否有环。

如果链表中有某个节点，可以通过连续跟踪 next 指针再次到达，则链表中存在环。 为了表示给定链表中的环，我们使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。 如果 pos 是 -1，则在该链表中没有环。注意：pos 不作为参数进行传递，仅仅是为了标识链表的实际情况。

如果链表中存在环，则返回 true 。 否则，返回 false 。

 

进阶：

你能用 O(1)（即，常量）内存解决此问题吗？
```c++
#include<cstddef>
#include<cstdio>
// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

 

class Solution {
public:
    bool hasCycle(ListNode *head) {
        if (head==NULL||head->next==NULL){
            return false;
        }
        ListNode *fast=head->next;
        ListNode *slow=head;
        while (1){
            if (slow==NULL||fast->next==NULL||fast->next->next==NULL){
                break;
            }
            fast=fast->next->next;
            slow=slow->next;
            if (fast==slow){
                return true;
            }
        }
        return false;
    }
};

int main(){
    ListNode *head=new ListNode(3);
    head->next=new ListNode(2);
    // ListNode *p=head->next;
    head->next->next=new ListNode(0);
    head->next->next->next=new ListNode(-4);
    head->next->next->next->next=head->next;
    Solution s;
    printf("%d\n",s.hasCycle(head));
    ListNode *head1=new ListNode(1);
    head1->next=new ListNode(2);
    printf("%d\n",s.hasCycle(head1));
    return 0;
}
```

## 142. 环形链表 II
给定一个链表，返回链表开始入环的第一个节点。 如果链表无环，则返回 null。

为了表示给定链表中的环，我们使用整数 pos 来表示链表尾连接到链表中的位置（索引从 0 开始）。 如果 pos 是 -1，则在该链表中没有环。注意，pos 仅仅是用于标识环的情况，并不会作为参数传递到函数中。

说明：不允许修改给定的链表。

进阶：

你是否可以使用 O(1) 空间解决此题？

```c++
#include<cstdio>
#include<cstddef>
// Definition for singly-linked list.
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
    ListNode *detectCycle(ListNode *head) {
        if(head==NULL||head->next==NULL){
            return NULL;
        }
        ListNode *slow=head;
        ListNode *fast=head->next;
        while(slow!=fast){
            if(slow->next==NULL||fast->next==NULL||fast->next->next==NULL){
                return NULL;
            }
            slow=slow->next;
            fast=fast->next->next;
            printf("fast:%d\n",fast->val);
            printf("slow:%d\n",slow->val);
        }
        slow=slow->next;
        ListNode *p=head;
        while(p!=slow){
            p=p->next;
            slow=slow->next;
            printf("p:%d\n",p->val);
            printf("slow:%d\n",slow->val);
        }
        return p;
    }
};
int main(){
    ListNode *head=new ListNode(3);
    head->next=new ListNode(2);
    // ListNode *p=head->next;
    head->next->next=new ListNode(0);
    head->next->next->next=new ListNode(-4);
    head->next->next->next->next=head->next;
    Solution s;
    printf("%d\n",s.detectCycle(head)->val);
    ListNode *head1=new ListNode(1);
    head1->next=new ListNode(2);
    printf("%d\n",s.detectCycle(head1));

    return 0;
}
```
