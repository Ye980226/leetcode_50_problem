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