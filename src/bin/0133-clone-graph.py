# @leetup=info id=133 lang=python3 slug=clone-graph



from typing import Optional


# @leetup=code

from collections import deque

class Node:
    def __init__(self, val=0, neighbors=None):
        self.val = val
        self.neighbors = neighbors if neighbors is not None else []


class Solution:
    def cloneGraph(self, node: Node) -> Node:
        if not node:
            return node

        q = deque([node])
        clones = {node.val: Node(node.val, [])}
        while q:
            cur = q.popleft()
            cur_clone = clones[cur.val]

            for ngbr in cur.neighbors:
                if ngbr.val not in clones:
                    clones[ngbr.val] = Node(ngbr.val, [])
                    q.append(ngbr)

                cur_clone.neighbors.append(clones[ngbr.val])

        return clones[node.val]


# @leetup=code
