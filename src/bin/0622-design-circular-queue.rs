// @leetup=info id=622 lang=rust slug=design-circular-queue

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

struct MyCircularQueue {
    q: Vec<i32>,
    cap: usize,
    size: usize,
    front: usize,
    rear: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            q: vec![0; k as usize],
            cap: k as usize,
            size: 0,
            front: 0,
            rear: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.q[self.rear] = value;
            self.rear = (self.rear + 1) % self.cap;
            self.size += 1;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.front = (self.front + 1) % self.cap;
            self.size -= 1;
            true
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.q[self.front]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            // self.q[(self.rear - 1) % self.cap]
            self.q[(self.rear + (self.cap - 1)) % self.cap]
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.cap
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */
// @leetup=code

fn main() {
    let mut obj = MyCircularQueue::new(8);
    let _ret_1: bool = obj.en_queue(1);
    let _ret_2: bool = obj.de_queue();
    let _ret_3: i32 = obj.front();
    let _ret_4: i32 = obj.rear();
    let _ret_5: bool = obj.is_empty();
    let _ret_6: bool = obj.is_full();
}
