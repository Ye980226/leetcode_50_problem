# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
class Solution:
    def maxDepth(self, root: TreeNode) -> int: 
        
        if not root:
            return 0
        else:
            return  1+max(self.maxDepth(root.left),self.maxDepth(root.right))
tree=TreeNode(3,TreeNode(9,None,None),TreeNode(20,TreeNode(15),TreeNode(7)))
s=Solution()
print(s.maxDepth(tree))
