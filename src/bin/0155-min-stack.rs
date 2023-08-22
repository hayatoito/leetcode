// @leetup=info id=155 lang=rust slug=min-stack

// @leetup=code

#[allow(unused_imports)]
use std::collections::*;

struct MinStack {
    s: Vec<i32>,
    min_s: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            s: Vec::new(),
            min_s: Vec::new(),
        }
    }

    fn push(&mut self, val: i32) {
        self.s.push(val);
        self.min_s.push(if self.min_s.is_empty() {
            val
        } else {
            self.get_min().min(val)
        })
    }

    fn pop(&mut self) {
        self.s.pop();
        self.min_s.pop();
    }

    fn top(&self) -> i32 {
        *self.s.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_s.last().unwrap()
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
// @leetup=code

struct Solution;

fn main() {
    // let _ = Solution::;
}
