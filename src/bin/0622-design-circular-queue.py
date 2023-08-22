# @leetup=info id=622 lang=python3 slug=design-circular-queue



from typing import Optional

# @leetup=code


class MyCircularQueue:
    def __init__(self, k: int):
        self.q = [0] * k
        self.cap = k
        self.size = 0
        self.front = 0
        self.rear = 0
        # q: [-, -, a, b, c,  -, -, .. ]
        #          ^ front
        #                   ^ rear

    def enQueue(self, value: int) -> bool:
        if self.isFull():
            return False
        self.q[self.rear] = value
        self.rear = (self.rear + 1) % self.cap
        self.size += 1
        return True

    def deQueue(self) -> bool:
        if self.isEmpty():
            return False
        self.front = (self.front + 1) % self.cap
        self.size -= 1
        return True

    def Front(self) -> int:
        if self.isEmpty():
            return -1
        return self.q[self.front]

    def Rear(self) -> int:
        if self.isEmpty():
            return -1
        # Pythons' '%' works here.
        return self.q[(self.rear - 1) % self.cap]

    def isEmpty(self) -> bool:
        return self.size == 0

    def isFull(self) -> bool:
        return self.size == self.cap


# @leetup=code
