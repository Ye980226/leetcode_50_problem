## 160.相交链表

编写一个程序，找到两个单链表相交的起始节点。

如下面的两个链表**：**

![img](160_statement.png)

在节点 c1 开始相交。

```c++
/**
Definition for singly-linked list.
**/

struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};
#include<algorithm>
#include<vector>
class Solution {
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        std::vector<ListNode*> vec;
        ListNode *pA=headA;
        ListNode *pB=headB;
        while (pA){
            vec.push_back(pA);
            pA=pA->next;
        }
        while (pB){
            std::vector<ListNode*>::iterator it=std::find(vec.begin(),vec.end(),pB);
            if (it!=vec.end()){
                return pB->val;
            }else{
                pB=pB->next;
            }
            
        }
        
    }
};
int main(){
    Solution s;
    
    s.getIntersectionNode();
    return 0;
}
```

## 169. 多数元素

给定一个大小为 n 的数组，找到其中的多数元素。多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素。

你可以假设数组是非空的，并且给定的数组总是存在多数元素。

```rust
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let (mut candidate,length,mut count)=(nums[0],nums.len(),1);
        for i in 1..length{
            count=if candidate==nums[i]{count + 1}else {count-1};
            if count==0{
                candidate=nums[i];
                count=1;
            }
        }
        candidate
    }
}
```

## 206.反转链表

反转一个链表

```rust
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre_node = None; 
        let mut current_node = head;

        while current_node != None{
            if let Some(mut x) = current_node{
                current_node = (*x).next;
                x.next = pre_node;
                pre_node = Some(x);
            }
        }

        pre_node
    }
}
```

