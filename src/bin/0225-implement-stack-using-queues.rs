// @leetup=info id=225 lang=rust slug=implement-stack-using-queues

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

struct MyStack {
    q1: VecDeque<i32>,
    q2: VecDeque<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    pub fn new() -> Self {
        MyStack {
            q1: VecDeque::new(),
            q2: VecDeque::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.q1.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        assert!(!self.empty());
        self.shorten_q1();
        self.q1.pop_front().unwrap()
    }

    pub fn top(&mut self) -> i32 {
        assert!(!self.empty());
        self.shorten_q1();
        *self.q1.get(0).unwrap()
    }

    fn shorten_q1(&mut self) {
        if self.q1.is_empty() {
            std::mem::swap(&mut self.q1, &mut self.q2);
        }
        while self.q1.len() > 1 {
            self.q2.push_back(self.q1.pop_front().unwrap());
        }
    }

    pub fn empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */
// @leetup=code

struct Solution;

fn main() {
    let _ = MyStack::new();
}
