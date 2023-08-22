// @leetup=info id=338 lang=rust slug=counting-bits

// @leetup=code
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![0; n as usize + 1];
        for i in 0..(n as usize + 1) {
            res[i] = res[i / 2] + (i as i32) % 2;
        }
        res
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::count_bits;
}
