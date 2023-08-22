// @leetup=info id=295 lang=rust slug=find-median-from-data-stream

// @leetup=code

use std::{cmp::Reverse, collections::*};

struct MedianFinder {
    // max heap
    small: BinaryHeap<i32>,
    // min heap
    large: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            small: Default::default(),
            large: Default::default(),
        }
    }

    fn add_num(&mut self, num: i32) {
        match (self.small.peek().cloned(), self.large.peek().cloned()) {
            (Some(a), _) => {
                if num <= a {
                    self.small.push(num);
                } else {
                    self.large.push(Reverse(num));
                }
            }
            (_, Some(Reverse(b))) => {
                if b <= num {
                    self.large.push(Reverse(num));
                } else {
                    self.small.push(num);
                }
            }
            (None, None) => {
                self.small.push(num);
            }
        };
        if self.small.len() > self.large.len() + 1 {
            self.large.push(Reverse(self.small.pop().unwrap()));
        }
        if self.small.len() + 1 < self.large.len() {
            let Reverse(a) = self.large.pop().unwrap();
            self.small.push(a);
        }
    }

    fn find_median(&self) -> f64 {
        if self.small.len() == self.large.len() {
            let (a, Reverse(b)) = (self.small.peek().unwrap(), self.large.peek().unwrap());
            (*a + *b) as f64 / 2.0
        } else if self.small.len() == self.large.len() + 1 {
            *self.small.peek().unwrap() as f64
        } else if self.small.len() + 1 == self.large.len() {
            let Reverse(a) = self.large.peek().unwrap();
            *a as f64
        } else {
            unreachable!()
        }
    }
}

// @leetup=code

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

fn main() {
    let mut a = MedianFinder::new();
    a.add_num(1);
    a.find_median();
}
