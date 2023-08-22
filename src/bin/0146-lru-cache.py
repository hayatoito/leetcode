# @leetup=info id=146 lang=python3 slug=lru-cache



# @leetup=code


import collections


class LRUCache:
    def __init__(self, capacity: int):
        self.cap = capacity
        self.cache = collections.OrderedDict()

    def get(self, key: int) -> int:
        if key in self.cache:
            self.cache.move_to_end(key)
            return self.cache[key]
        return -1

    def put(self, key: int, value: int) -> None:
        if key in self.cache:
            self.cache[key] = value
            self.cache.move_to_end(key)
            return
        if len(self.cache) == self.cap:
            self.cache.popitem(last=False)

        self.cache[key] = value
        self.cache.move_to_end(key)





class Node:
    def __init__(self, key: int, val: int):
        self.key = key
        self.val = val
        self.prev: Node | None = None
        self.next: Node | None = None


class LRUCache_xxxx:
    def __init__(self, capacity: int):
        self.cap = capacity
        self.cache = {}  # map key to ndoe
        # double linked list
        # Left.next=LRU, right=most recent
        self.left = Node(0, 0)
        self.right = Node(0, 0)
        self.left.next = self.right
        self.right.prev = self.left

    def get(self, key: int) -> int:
        if key in self.cache:
            node = self.cache[key]
            self.remove(node)
            self.insert(node)
            return node.val
        return -1

    def put(self, key: int, value: int) -> None:
        if key in self.cache:
            self.remove(self.cache[key])
        node = Node(key, value)
        self.cache[key] = node
        self.insert(node)
        if len(self.cache) > self.cap:
            # Remove from the list and delete the LRU from the cache
            lru = self.left.next
            self.remove(lru)
            del self.cache[lru.key]

    # For double linked list
    def remove(self, node: Node):
        prev, nxt = node.prev, node.next
        prev.next, nxt.prev = nxt, prev

    def insert(self, node: Node):
        prev, nxt = self.right.prev, self.right
        prev = self.right.prev
        prev.next = node
        nxt.prev = node
        node.next, node.prev = nxt, prev


# @leetup=code
