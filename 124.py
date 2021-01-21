# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution:
    def __init__(self):
        self.max_=-2**32+1
    def maxPathSum(self, root: TreeNode) -> int:
        
        def maxPathSumTmp(root:TreeNode)->int:
            max_=0
            if not root:
                return 0
            else:
                
                left=max(0,maxPathSumTmp(root.left))
                right=max(0,maxPathSumTmp(root.right))
                lmr=root.val+left+right
                ret=root.val+max(left,right)
                # max_=max(tmp_rslt,max_)
                # max_
                # self.max_s.append(max_)
                # max_=0
                # max_=max(self.maxPathSum(root.left),max_)
                # self.max_s.append(max_)
                # max_=0
                # max_=max(self.maxPathSum(root.right),max_)
                # self.max_s.append(max_)
                self.max_=max(lmr,ret,self.max_)

                # print("val",root.val)
                # print("max_",max_)
                
            return ret
        maxPathSumTmp(root)
        return self.max_
tree=TreeNode(-10,TreeNode(9),TreeNode(20,TreeNode(15),TreeNode(7)))
s=Solution()
print(s.maxPathSum(tree))
