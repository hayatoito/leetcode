# @leetup=info id=225 lang=python3 slug=implement-stack-using-queues



from typing import Optional

# @leetup=code

from collections import deque


class MyStack:
    def __init__(self):
        self.q1 = deque()
        self.q2 = deque()

    def push(self, x: int) -> None:
        self.q1.append(x)

    def pop(self) -> int:
        assert not self.empty()
        self.shorten_q1()
        return self.q1.popleft()

    def top(self) -> int:
        assert not self.empty()
        self.shorten_q1()
        return self.q1[0]

    def shorten_q1(self):
        if not self.q1:
            # q1: []
            # q2: [1, 2, 3]
            self.q1, self.q2 = self.q2, self.q1
        # q1: [1, 2, 3]
        # q2: []
        while len(self.q1) > 1:
            self.q2.append(self.q1.popleft())
        # q1: [3]
        # q2: [1, 2]

    def empty(self) -> bool:
        return (not self.q1) and (not self.q2)


# @leetup=code
