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