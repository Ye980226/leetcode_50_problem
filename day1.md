## 2.两数相加
给你两个 `非空` 的链表，表示两个非负的整数。它们每位数字都是按照 `逆序` 的方式存储的，并且每个节点只能存储 `一位` 数字。
请你将两个数相加，并以相同形式返回一个表示和的链表。
你可以假设除了数字 0 之外，这两个数都不会以 0 开头。
```python
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
```
## 4.寻找两个正序数组的中位数
给定两个大小为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。请你找出并返回这两个正序数组的中位数。
进阶：你能设计一个时间复杂度为 O(log (m+n)) 的算法解决此问题吗？
作弊方法。
```python
class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        nums=nums1+nums2
        nums.sort()
        if len(nums)%2==0:
            median=len(nums)//2
            return (nums[median-1]+nums[median])/2
        else:
            return nums[len(nums)//2]
```

```python
class Solution:
    def findMedianSortedArrays(self, nums1, nums2) -> float:
        m=len(nums1)
        n=len(nums2)
        def getKthElement(k):
            #[index1..],[index2..]
            index1,index2=0,0
            # 特殊情况
            
            while True:
                if index1 == m:
                    return nums2[index2 + k - 1]
                if index2 == n:
                    return nums1[index1 + k - 1]
                if k == 1:
                    return min(nums1[index1], nums2[index2])

                newIndex1=min(index1+k//2-1,m-1)
                newIndex2=min(index2+k//2-1,n-1)
                pivot1,pivot2=nums1[newIndex1],nums2[newIndex2]
                #当nums1 k//2-1小于nums2的，这部分就不可能是第k个元素，这部分区间是[index1...newIndex1],所以剔除了newIndex1-index1+1个数,变成区间[newIndex+1..]
                if pivot1<=pivot2:
                    k-=newIndex1-index1+1
                    index1=newIndex1+1
                else:
                    k -= newIndex2 - index2 + 1
                    index2 = newIndex2 + 1
                #这样总会达到边界条件，要么是index1=m了，要么是index2=n了，意味着已经越界，也就newIndex1取得是m-1，要么是k=1了，也就是前面已经出去k-1个了，也就是要么是index1和index2指向了k和k+1了
        totallength=m+n
        if(totallength%2==1):
            return getKthElement((totallength+1)//2)
        else:
            return (getKthElement(totallength//2)+getKthElement(totallength//2+1))/2    

```
## 5.最长回文子串
给你一个字符串 `s`，找到 `s` 中最长的回文子串。
```python
class Solution:
    def expandAroundCenter(self, s, left, right):
        #从边界条件出发开始扩展，直到不能扩
        while left >= 0 and right < len(s) and s[left] == s[right]:
            left -= 1
            right += 1
        return left + 1, right - 1

    def longestPalindrome(self, s: str) -> str:
        start, end = 0, 0
        for i in range(len(s)):
            #两个边界条件，一个是1，一个是2
            left1, right1 = self.expandAroundCenter(s, i, i)
            left2, right2 = self.expandAroundCenter(s, i, i + 1)
            #找出最大的
            if right1 - left1 > end - start:
                start, end = left1, right1
            if right2 - left2 > end - start:
                start, end = left2, right2
        return s[start: end + 1]
```