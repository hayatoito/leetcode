// @leetup=info id=232 lang=rust slug=implement-queue-using-stacks

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            s1: Vec::new(),
            s2: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.s1.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.move_s1_to_s2_if_s2_is_empty();
        self.s2.pop().unwrap()
    }

    fn move_s1_to_s2_if_s2_is_empty(&mut self) {
        if self.s2.is_empty() {
            while let Some(n) = self.s1.pop() {
                self.s2.push(n);
            }
        }
    }

    fn peek(&mut self) -> i32 {
        self.move_s1_to_s2_if_s2_is_empty();
        *self.s2.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.s1.is_empty() && self.s2.is_empty()
    }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
// @leetup=code

struct Solution;

fn main() {
    let _ = MyQueue::new();
}
