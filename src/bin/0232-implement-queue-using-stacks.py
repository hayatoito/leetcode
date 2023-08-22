# @leetup=info id=232 lang=python3 slug=implement-queue-using-stacks



from typing import Optional

# @leetup=code
class MyQueue:
    def __init__(self):
        self.s1 = []
        self.s2 = []

    def push(self, x: int) -> None:
        self.s1.append(x)

    def pop(self) -> int:
        self.move_s1_to_s2_if_s2_is_empty()
        return self.s2.pop()

    def peek(self) -> int:
        self.move_s1_to_s2_if_s2_is_empty()
        return self.s2[-1]

    def move_s1_to_s2_if_s2_is_empty(self):
        if not self.s2:
            # s1: [1, 2, 3]
            # s2: []
            while self.s1:
                self.s2.append(self.s1.pop())
            # s1: []
            # s2: [3, 2, 1]

    def empty(self) -> bool:
        return (not self.s1) and (not self.s2)


# @leetup=code
