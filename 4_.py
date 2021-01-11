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


