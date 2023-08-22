# @leetup=info id=380 lang=python3 slug=insert-delete-getrandom-o1



# @leetup=code


import random


class RandomizedSet:
    def __init__(self):
        self.num_map = {}
        self.num_list = []

    def insert(self, val: int) -> bool:
        if val in self.num_map:
            return False
        self.num_map[val] = len(self.num_list)
        self.num_list.append(val)
        return True

    def remove(self, val: int) -> bool:
        if val not in self.num_map:
            return False
        index = self.num_map[val]
        last_value = self.num_list[-1]
        self.num_list[index] = last_value
        self.num_list.pop()
        self.num_map[last_value] = index
        del self.num_map[val]
        return True

    def getRandom(self) -> int:
        return random.choice(self.num_list)


# @leetup=code
