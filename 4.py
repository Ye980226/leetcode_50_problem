class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        nums=nums1+nums2
        nums.sort()
        if len(nums)%2==0:
            median=len(nums)//2
            return (nums[median-1]+nums[median])/2
        else:
            return nums[len(nums)//2]