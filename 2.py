# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        l1_val=""
        l2_val=""
        # print(l1)
        # print(l2)
        l3=ListNode(0,None)
        while l1:
            l1_val+=str(l1.val)
            l1=l1.next
        while l2:
            l2_val+=str(l2.val)
            l2=l2.next
        l3_val=str(int(l1_val[::-1])+int(l2_val[::-1]))
        # print(l3_val)
        l3=ListNode(int(l3_val[-1]))
        p=l3
        for i in range(len(l3_val)-1,0,-1):
            p.next=ListNode(int(l3_val[i-1]))
            p=p.next
        return l3


# l1=ListNode(2,ListNode(4,ListNode(3,None)))
# l2=ListNode(5,ListNode(6,ListNode(4,None)))
# l1=ListNode(0)
# l2=ListNode(0)
l1=ListNode(9,ListNode(9,ListNode(9,ListNode(9,ListNode(9,ListNode(9,ListNode(9,ListNode(9))))))))
l2=ListNode(9,ListNode(9,ListNode(9,ListNode(9))))

s=Solution()
l3=s.addTwoNumbers(l1,l2)
l3_val=""
while l3:
    l3_val+=str(l3.val)
    l3=l3.next
print(l3_val)