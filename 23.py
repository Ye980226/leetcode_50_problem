class Solution:
    def mergeKLists(self, lists: List[ListNode]) -> ListNode:
        """分治合并"""
        def merge2Lists(list1, list2):
            """list1和list2分别是两个链表的头结点"""
            head = ListNode(0)  # 新建一个头节点用来返回list1和list2的排序后结果
            dummy = head  # 指向head链表的尾部，用来拆入新节点
            while list1 and list2:  # 两个链表都不为空时
                if list1.val < list2.val:
                    dummy.next = list1  # 插入新节点
                    dummy = list1  # dummy指向新插入的节点
                    list1 = list1.next  # 移动list1
                    continue
                dummy.next = list2
                dummy = list2
                list2 = list2.next
            dummy.next = list2 if not list1 else list1  # 如果list1为None，说明list1中的元素全部进入了head链表，所以把list2中剩余的元素直接添加进来，反之亦然。
            return head.next
        # print(merge2Lists(lists[1], lists[2]))

        res = None
        def divideMergeKLists(lists):
            """分治归并k个链表"""
            nonlocal res
            k = len(lists)
            temp = []

            if k == 0: return  # lists为空，直接结束
            if k == 1 :  # 归并结束
                res = lists[0]  # 保存结果
                return
            for i in range(0, k, 2):
                if i+1 == k:  # k为奇数，i的最后一个取值是k-1，此时i+1已经超出范围
                    temp.append(lists[i])
                    break
                # 如果k为偶数，i的最后一个取值永远是k-2
                temp.append(merge2Lists(lists[i], lists[i+1]))
            divideMergeKLists(temp)

        divideMergeKLists(lists)
        return res
        