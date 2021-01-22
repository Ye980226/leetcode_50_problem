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