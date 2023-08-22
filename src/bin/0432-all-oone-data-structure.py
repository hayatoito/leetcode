# @leetup=info id=432 lang=python3 slug=all-oone-data-structure



# @leetup=code


class Node:
    def __init__(self, count):
        self.count = count
        # For O(1), create only one node for all keys which have the same count.
        self.keys = set()
        self.next = None
        self.prev = None


class AllOne:
    def __init__(self):
        self.key_to_node = {}

        # double linked list
        # [left, min_node, ...., max_node, right]
        self.left = Node(0)
        self.right = Node(inf)
        self.left.next = self.right
        self.right.prev = self.left

    def inc(self, key: str) -> None:
        if key not in self.key_to_node:
            node = Node(0)
            node.keys = set()
            node.keys.add(key)
            self.insert_node(node, self.left)
            self.key_to_node[key] = node
        else:
            node = self.key_to_node[key]

        if node.count + 1 == node.next.count:
            # Add key to node.next
            node.next.keys.add(key)
            self.key_to_node[key] = node.next
        else:
            # insert new node
            new_node = Node(node.count + 1)
            new_node.keys.add(key)
            self.insert_node(new_node, node)
            self.key_to_node[key] = new_node

        # remove key from node
        node.keys.remove(key)
        if not node.keys:
            self.remove_node(node)

    def dec(self, key: str) -> None:
        assert key in self.key_to_node
        node = self.key_to_node[key]

        if node.count == 1:
            del self.key_to_node[key]
        elif node.count - 1 == node.prev.count:
            # Add key to node.prev
            node.prev.keys.add(key)
            self.key_to_node[key] = node.prev
        else:
            # insert new node
            new_node = Node(node.count - 1)
            new_node.keys.add(key)
            self.insert_node(new_node, node.prev)
            self.key_to_node[key] = new_node

        # remove key from node
        node.keys.remove(key)
        if not node.keys:
            self.remove_node(node)

    def insert_node(self, node, prev):
        # [ prev, nxt] => [prev, node, nxt]
        nxt = prev.next

        prev.next = node
        node.prev = prev

        node.next = nxt
        nxt.prev = node

    def remove_node(self, node):
        #  [prev, node, nxt] => [ prev, nxt]
        prev, next = node.prev, node.next
        prev.next, next.prev = next, prev

    def getMaxKey(self) -> str:
        max_node = self.right.prev
        if max_node.keys:
            res = max_node.keys.pop()
            max_node.keys.add(res)
            return res
        return ""

    def getMinKey(self) -> str:
        min_node = self.left.next
        if min_node.keys:
            res = min_node.keys.pop()
            min_node.keys.add(res)
            return res
        return ""


# @leetup=code
