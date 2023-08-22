// @leetup=info id=191 lang=rust slug=number-of-1-bits

// @leetup=code
impl Solution {
    pub fn hammingWeight(mut n: u32) -> i32 {
        let mut cnt = 0;
        while n != 0 {
            n &= (n - 1);
            cnt += 1;
        }
        cnt
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::hammingWeight;
}
