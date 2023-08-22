// @leetup=info id=190 lang=rust slug=reverse-bits

// @leetup=code
impl Solution {
    pub fn reverse_bits(mut x: u32) -> u32 {
        let mut y = 0;
        for _ in 0..32 {
            y = (y << 1) | (x & 1);
            x >>= 1;
        }
        y
    }
}
// @leetup=code

struct Solution;

fn main() {
    let _ = Solution::reverse_bits;
}
