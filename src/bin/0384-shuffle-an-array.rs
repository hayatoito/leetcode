// @leetup=info id=384 lang=rust slug=shuffle-an-array

// @leetup=code

use rand::distributions::Uniform;
use rand::prelude::*;

struct Solution {
    nums: Vec<i32>,
    rng: ThreadRng,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            nums,
            rng: thread_rng(),
        }
    }

    fn reset(&mut self) -> Vec<i32> {
        self.nums.clone()
    }

    fn shuffle(&mut self) -> Vec<i32> {
        let mut r = self.nums.clone();
        for i in 0..r.len() {
            let uniform = Uniform::from(i..r.len());
            let j = self.rng.sample(uniform);
            r.swap(i, j);
        }
        r
    }
}

// @leetup=code

fn main() {
    let mut s = Solution::new(vec![1, 2, 3, 4, 5]);
    println!("{:?}", s.reset());
    for _ in 0..100 {
        println!("{:?}", s.shuffle());
    }
}
